use std::collections::HashMap;
use anyhow::{anyhow, Context, Result};
use coarsetime::Instant as CoarseInstant;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad_tiled_redux::animation_controller::{AnimationController, AnimationTemplateRegistry};
use macroquad_tiled_redux::tile_set_catalogue::{load_tile_set, TilesetCatalogue};
use macroquad_tiled_redux::{TileSet, Tl};

use crate::contract::{
    actor_base_paths, actor_clip, actor_scale, actor_supported_intents, actor_why,
    overlay_fallback_clip, overlay_paths, resolve_repo_path, supported_overlays, ActorArchetype, ActorId,
    ActorIntent, Facing8, OverlayKind, QueuePolicy, ALL_ACTOR_ARCHETYPES,
};

pub struct ActorSpawnSpec {
    pub archetype: ActorArchetype,
    pub position: Tl,
    pub facing: Facing8,
    pub overlays: Vec<OverlayKind>,
}

pub struct ActorClipRequest {
    pub actor_id: ActorId,
    pub intent: ActorIntent,
    pub facing: Facing8,
    pub movement: Tl,
    pub queue: QueuePolicy,
    pub compression: Option<u32>,
    pub snap_to: Option<Tl>,
}

pub struct ActorSystem {
    tile_size: f32,
    loaded: HashMap<ActorArchetype, LoadedActorArchetype>,
    actors: HashMap<ActorId, ActorInstance>,
    next_id: ActorId,
    validation_log: Vec<String>,
}

struct LoadedActorArchetype {
    scale: f32,
    registry: AnimationTemplateRegistry,
    tilesets: TilesetCatalogue,
    overlays: HashMap<OverlayKind, LoadedOverlay>,
}

struct LoadedOverlay {
    registry: AnimationTemplateRegistry,
    tilesets: TilesetCatalogue,
}

struct ActorInstance {
    archetype: ActorArchetype,
    controller: AnimationController,
    facing: Facing8,
    overlays: Vec<OverlayKind>,
}

impl ActorSystem {
    pub async fn load(tile_size: f32) -> Result<Self> {
        let mut loaded = HashMap::new();
        let mut validation_log = vec![];

        for &archetype in ALL_ACTOR_ARCHETYPES {
            let loaded_archetype = load_archetype(archetype, &mut validation_log).await?;
            validation_log.push(format!(
                "validated actor {}: {}",
                archetype,
                actor_why(archetype)
            ));
            loaded.insert(archetype, loaded_archetype);
        }

        Ok(Self {
            tile_size,
            loaded,
            actors: HashMap::new(),
            next_id: 1,
            validation_log,
        })
    }

    pub fn validation_log(&self) -> &[String] {
        &self.validation_log
    }

    pub fn spawn(&mut self, spec: ActorSpawnSpec) -> Result<ActorId> {
        let loaded = self
            .loaded
            .get(&spec.archetype)
            .with_context(|| format!("actor archetype {} not loaded", spec.archetype))?;

        let mut controller = AnimationController::new_at_pos(spec.position);
        let idle_clip = actor_clip(spec.archetype, ActorIntent::Idle, spec.facing)
            .ok_or_else(|| anyhow!("no idle clip for {}", spec.archetype))?;
        controller
            .set_start_frame_from_registry(idle_clip, &loaded.registry)
            .with_context(|| format!("failed to set start frame {idle_clip} for {}", spec.archetype))?;

        let actor_id = self.next_id;
        self.next_id += 1;

        self.actors.insert(
            actor_id,
            ActorInstance {
                archetype: spec.archetype,
                controller,
                facing: spec.facing,
                overlays: spec.overlays,
            },
        );

        Ok(actor_id)
    }

    pub fn remove_now(&mut self, actor_id: ActorId) {
        self.actors.remove(&actor_id);
    }

    pub fn set_overlays(&mut self, actor_id: ActorId, overlays: Vec<OverlayKind>) {
        let Some(actor) = self.actors.get_mut(&actor_id) else {
            return;
        };
        actor.overlays = overlays;
    }

    pub fn current_position(&self, actor_id: ActorId) -> Option<Tl> {
        self.actors
            .get(&actor_id)
            .map(|actor| actor.controller.get_current_position(CoarseInstant::recent()))
    }

    pub fn clip_duration_ms(
        &self,
        archetype: ActorArchetype,
        intent: ActorIntent,
        facing: Facing8,
    ) -> Option<u64> {
        let clip = actor_clip(archetype, intent, facing)?;
        let loaded = self.loaded.get(&archetype)?;
        loaded.registry.get_template(clip).map(|template| {
            template
                .frames
                .iter()
                .map(|frame| frame.duration.as_millis() as u64)
                .sum()
        })
    }

    pub fn apply(&mut self, request: ActorClipRequest) -> Result<()> {
        let Some(actor) = self.actors.get_mut(&request.actor_id) else {
            return Ok(());
        };

        let loaded = self
            .loaded
            .get(&actor.archetype)
            .with_context(|| format!("actor archetype {} not loaded", actor.archetype))?;

        actor.facing = request.facing;
        if let Some(snap_to) = request.snap_to {
            actor.controller.set_start_pos(snap_to);
        }

        if matches!(request.queue, QueuePolicy::Interrupt) {
            actor.controller.clear_at_current_pos();
        }

        let Some(clip_name) = actor_clip(actor.archetype, request.intent, request.facing) else {
            return Ok(());
        };

        if request.intent == ActorIntent::Idle {
            actor.controller
                .set_start_frame_from_registry(clip_name, &loaded.registry)
                .with_context(|| {
                    format!(
                        "failed to set idle clip {clip_name} for actor {}",
                        actor.archetype
                    )
                })?;
            return Ok(());
        }

        match request.compression {
            Some(compression) => actor
                .controller
                .add_animation_from_registry_with_compression(
                    &loaded.registry,
                    clip_name,
                    request.movement,
                    0,
                    compression,
                )
                .with_context(|| {
                    format!(
                        "failed to add compressed clip {clip_name} for actor {}",
                        actor.archetype
                    )
                })?,
            None => actor
                .controller
                .add_animation_from_registry(&loaded.registry, clip_name, request.movement, 0)
                .with_context(|| {
                    format!("failed to add clip {clip_name} for actor {}", actor.archetype)
                })?,
        }

        Ok(())
    }

    pub fn update(&mut self) {
        for actor in self.actors.values_mut() {
            actor.controller.update(CoarseInstant::recent());
        }
    }

    pub fn draw(&self) {
        let now = CoarseInstant::recent();
        let mut order: Vec<_> = self
            .actors
            .iter()
            .map(|(id, actor)| (*id, actor.controller.get_current_position(now).y))
            .collect();
        order.sort_by(|lhs, rhs| lhs.1.partial_cmp(&rhs.1).unwrap_or(std::cmp::Ordering::Equal));

        for (actor_id, _) in order {
            let Some(actor) = self.actors.get(&actor_id) else {
                continue;
            };
            let Some(loaded) = self.loaded.get(&actor.archetype) else {
                continue;
            };
            let Some(frame) = actor.controller.get_frame(now) else {
                continue;
            };

            if let Some(tileset) = loaded.tilesets.get_by_frame(&frame.frame) {
                draw_frame(self.tile_size, loaded.scale, tileset, frame.frame.tile_id, frame.position);
            }

            for overlay in &actor.overlays {
                let Some(loaded_overlay) = loaded.overlays.get(overlay) else {
                    continue;
                };

                let overlay_frame = loaded_overlay
                    .registry
                    .get_frame(&frame.frame.animation_name, frame.frame.tile_number)
                    .or_else(|| {
                        let fallback_clip = overlay_fallback_clip(actor.archetype, actor.facing);
                        loaded_overlay.registry.get_static_frame(fallback_clip)
                    });

                let Some(overlay_frame) = overlay_frame else {
                    continue;
                };

                let Some(tileset) = loaded_overlay.tilesets.get(&overlay_frame.tileset) else {
                    continue;
                };
                draw_frame(
                    self.tile_size,
                    loaded.scale,
                    tileset,
                    overlay_frame.tile_id,
                    frame.position,
                );
            }
        }
    }
}

async fn load_archetype(
    archetype: ActorArchetype,
    validation_log: &mut Vec<String>,
) -> Result<LoadedActorArchetype> {
    let (registry, tilesets) = load_registry(actor_base_paths(archetype)).await?;

    for intent in actor_supported_intents(archetype) {
        for facing in [
            Facing8::North,
            Facing8::NorthEast,
            Facing8::East,
            Facing8::SouthEast,
            Facing8::South,
            Facing8::SouthWest,
            Facing8::West,
            Facing8::NorthWest,
        ] {
            if let Some(clip) = actor_clip(archetype, *intent, facing) {
                if registry.get_template(clip).is_none() {
                    return Err(anyhow!(
                        "actor {} is missing clip {} for {} {}",
                        archetype,
                        clip,
                        intent,
                        facing
                    ));
                }
            }
        }
    }

    let mut overlays = HashMap::new();
    for &overlay in supported_overlays(archetype) {
        let Some(paths) = overlay_paths(archetype, overlay) else {
            continue;
        };
        let (overlay_registry, overlay_tilesets) = load_registry(paths).await?;
        validation_log.push(format!(
            "validated overlay {} for {}",
            overlay,
            archetype
        ));
        overlays.insert(
            overlay,
            LoadedOverlay {
                registry: overlay_registry,
                tilesets: overlay_tilesets,
            },
        );
    }

    Ok(LoadedActorArchetype {
        scale: actor_scale(archetype),
        registry,
        tilesets,
        overlays,
    })
}

async fn load_registry(paths: &[&str]) -> Result<(AnimationTemplateRegistry, TilesetCatalogue)> {
    let mut registry = AnimationTemplateRegistry::new();
    let mut tilesets = TilesetCatalogue::default();

    for rel_path in paths {
        let path = resolve_repo_path(rel_path);
        if !path.exists() {
            return Err(anyhow!("missing tileset {}", path.display()));
        }

        let tileset = load_tile_set(&path)
            .await
            .with_context(|| format!("failed to load tileset {}", path.display()))?;
        registry.add_tileset(&tileset.tileset);
        tilesets.add(tileset);
    }

    Ok((registry, tilesets))
}

fn draw_frame(tile_size: f32, scale: f32, tileset: &TileSet, tile_id: u32, position: Tl) {
    let source = Some(tileset.sprite_rect(tile_id));
    let base = vec2(tile_size, tile_size);
    let dest_size = base * scale;
    let pos = position.into_pixels_vec2(base) - (dest_size - base) / 2.0;

    draw_texture_ex(
        &tileset.texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(dest_size),
            source,
            ..Default::default()
        },
    );
}
