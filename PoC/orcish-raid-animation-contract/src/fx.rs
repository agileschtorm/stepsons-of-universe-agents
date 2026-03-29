use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use coarsetime::Instant as CoarseInstant;
use macroquad::color::WHITE;
use macroquad::math::{vec2, Vec2};
use macroquad::texture::{draw_texture_ex, load_texture, DrawTextureParams, FilterMode, Texture2D};
use macroquad_tiled_redux::animation_controller::AnimationController;
use macroquad_tiled_redux::animation_template::AnimationTemplate;
use macroquad_tiled_redux::tile_set_catalogue::load_tile_set;
use macroquad_tiled_redux::{TileSet, Tl};

use crate::contract::{resolve_repo_path, EffectIntent, VisualSource, EFFECT_BINDINGS};

pub struct FxSystem {
    tile_size: f32,
    loaded: HashMap<EffectIntent, LoadedSource>,
    active: Vec<ActiveEffect>,
}

enum LoadedSource {
    StaticTile {
        tileset: Arc<TileSet>,
        tile_id: u32,
        scale: f32,
    },
    TiledClip {
        tileset: Arc<TileSet>,
        template: AnimationTemplate,
        scale: f32,
    },
    FrameDir {
        frames: Arc<Vec<Texture2D>>,
        frame_duration: Duration,
        looped: bool,
        scale: f32,
    },
}

enum ActiveEffect {
    MovingTile {
        tileset: Arc<TileSet>,
        tile_id: u32,
        from: Tl,
        to: Tl,
        start: Instant,
        duration: Duration,
        rotation: f32,
        scale: f32,
    },
    TiledClip {
        tileset: Arc<TileSet>,
        controller: AnimationController,
        scale: f32,
    },
    FrameDir {
        frames: Arc<Vec<Texture2D>>,
        frame_duration: Duration,
        looped: bool,
        position: Tl,
        start: Instant,
        rotation: f32,
        scale: f32,
    },
    StaticTile {
        tileset: Arc<TileSet>,
        tile_id: u32,
        position: Tl,
        rotation: f32,
        scale: f32,
    },
}

impl FxSystem {
    pub async fn load(tile_size: f32) -> Result<Self> {
        let mut loaded = HashMap::new();
        let mut tileset_cache: HashMap<PathBuf, Arc<TileSet>> = HashMap::new();

        for binding in EFFECT_BINDINGS {
            let source = match binding.source {
                VisualSource::StaticTile {
                    rel_path,
                    name,
                    scale,
                } => {
                    let tileset = load_tileset_cached(&mut tileset_cache, rel_path).await?;
                    let tile_id = tileset
                        .tile_by_name(name)
                        .with_context(|| format!("tile {name} not found in {}", tileset.tileset.name))?;
                    LoadedSource::StaticTile {
                        tileset,
                        tile_id,
                        scale,
                    }
                }
                VisualSource::TiledClip {
                    rel_path,
                    name,
                    scale,
                } => {
                    let tileset = load_tileset_cached(&mut tileset_cache, rel_path).await?;
                    let template = AnimationTemplate::get_from_tileset(&tileset.tileset, name)
                        .with_context(|| format!("clip {name} not found in {}", tileset.tileset.name))?;
                    LoadedSource::TiledClip {
                        tileset,
                        template,
                        scale,
                    }
                }
                VisualSource::FrameDir {
                    rel_path,
                    frame_ms,
                    looped,
                    scale,
                } => LoadedSource::FrameDir {
                    frames: load_frame_dir(rel_path).await?,
                    frame_duration: Duration::from_millis(frame_ms),
                    looped,
                    scale,
                },
            };

            loaded.insert(binding.intent, source);
        }

        Ok(Self {
            tile_size,
            loaded,
            active: vec![],
        })
    }

    pub fn spawn_projectile(&mut self, from: Tl, to: Tl) {
        let Some(LoadedSource::StaticTile {
            tileset,
            tile_id,
            scale,
        }) = self.loaded.get(&EffectIntent::ProjectileTravel)
        else {
            return;
        };

        let delta: Vec2 = (to - from).into();
        let rhs_hack = vec2(0.0, 1.0);
        let rotation = -delta.angle_between(rhs_hack);
        let distance = delta.length().max(1.0);
        let duration = Duration::from_millis((60.0 * distance) as u64);

        self.active.push(ActiveEffect::MovingTile {
            tileset: Arc::clone(tileset),
            tile_id: *tile_id,
            from,
            to,
            start: Instant::now(),
            duration,
            rotation,
            scale: *scale,
        });
    }

    pub fn spawn_impact(&mut self, at: Tl) {
        self.spawn_frame_dir(EffectIntent::ProjectileImpact, at, 0.0);
    }

    pub fn spawn_blood_hit(&mut self, at: Tl) {
        let Some(LoadedSource::TiledClip {
            tileset,
            template,
            scale,
        }) = self.loaded.get(&EffectIntent::BloodHit)
        else {
            return;
        };

        let mut controller = AnimationController::new_at_pos(at);
        controller.add_animation(CoarseInstant::now(), template, Tl::new(0.0, 0.0));

        self.active.push(ActiveEffect::TiledClip {
            tileset: Arc::clone(tileset),
            controller,
            scale: *scale,
        });
    }

    pub fn spawn_corpse(&mut self, at: Tl) {
        let Some(LoadedSource::StaticTile {
            tileset,
            tile_id,
            scale,
        }) = self.loaded.get(&EffectIntent::Corpse)
        else {
            return;
        };

        self.active.push(ActiveEffect::StaticTile {
            tileset: Arc::clone(tileset),
            tile_id: *tile_id,
            position: at,
            rotation: 0.0,
            scale: *scale,
        });
    }

    pub fn spawn_melee_swing(&mut self, from: Tl, to: Tl) {
        let delta: Vec2 = (to - from).into();
        let rhs_hack = vec2(0.0, 1.0);
        let rotation = -delta.angle_between(rhs_hack);
        self.spawn_frame_dir(EffectIntent::MeleeSwing, to, rotation);
    }

    pub fn gc(&mut self) {
        self.active.retain_mut(|effect| match effect {
            ActiveEffect::MovingTile {
                start, duration, ..
            } => start.elapsed() <= *duration,
            ActiveEffect::TiledClip { controller, .. } => {
                controller.update(CoarseInstant::recent());
                !controller.has_no_animations()
            }
            ActiveEffect::FrameDir {
                frames,
                frame_duration,
                looped,
                start,
                ..
            } => {
                if *looped {
                    true
                } else {
                    let total_ms = frame_duration.as_millis() as u64 * frames.len() as u64;
                    start.elapsed() <= Duration::from_millis(total_ms)
                }
            }
            ActiveEffect::StaticTile { .. } => true,
        });
    }

    pub fn draw_underlays(&mut self) {
        let tile_size = self.tile_size;
        for effect in &mut self.active {
            let ActiveEffect::StaticTile {
                tileset,
                tile_id,
                position,
                rotation,
                scale,
            } = effect
            else {
                continue;
            };
            draw_tile(tile_size, tileset, *tile_id, *position, *rotation, *scale);
        }
    }

    pub fn draw_overlays(&mut self) {
        let tile_size = self.tile_size;
        for effect in &mut self.active {
            match effect {
                ActiveEffect::StaticTile { .. } => {}
                ActiveEffect::MovingTile {
                    tileset,
                    tile_id,
                    from,
                    to,
                    start,
                    duration,
                    rotation,
                    scale,
                } => {
                    let total = duration.as_secs_f32().max(0.001);
                    let t = (start.elapsed().as_secs_f32() / total).clamp(0.0, 1.0);
                    let pos = Tl::new(
                        from.x * (1.0 - t) + to.x * t,
                        from.y * (1.0 - t) + to.y * t,
                    );
                    draw_tile(tile_size, tileset, *tile_id, pos, *rotation, *scale);
                }
                ActiveEffect::TiledClip {
                    tileset,
                    controller,
                    scale,
                } => {
                    controller.update(CoarseInstant::recent());
                    if let Some(frame) = controller.get_frame(CoarseInstant::recent()) {
                        draw_tile(
                            tile_size,
                            tileset,
                            frame.frame.tile_id,
                            frame.position,
                            0.0,
                            *scale,
                        );
                    }
                }
                ActiveEffect::FrameDir {
                    frames,
                    frame_duration,
                    looped,
                    position,
                    start,
                    rotation,
                    scale,
                } => {
                    let Some(frame) = current_frame(frames, *frame_duration, *looped, *start) else {
                        continue;
                    };
                    draw_texture(tile_size, frame, *position, *rotation, *scale);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw(&mut self) {
        self.draw_underlays();
        self.draw_overlays();
    }

    fn spawn_frame_dir(&mut self, intent: EffectIntent, at: Tl, rotation: f32) {
        let Some(LoadedSource::FrameDir {
            frames,
            frame_duration,
            looped,
            scale,
        }) = self.loaded.get(&intent)
        else {
            return;
        };

        self.active.push(ActiveEffect::FrameDir {
            frames: Arc::clone(frames),
            frame_duration: *frame_duration,
            looped: *looped,
            position: at,
            start: Instant::now(),
            rotation,
            scale: *scale,
        });
    }
}

fn draw_tile(tile_size: f32, tileset: &TileSet, tile_id: u32, position: Tl, rotation: f32, scale: f32) {
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
            rotation,
            ..Default::default()
        },
    );
}

fn draw_texture(tile_size: f32, texture: &Texture2D, position: Tl, rotation: f32, scale: f32) {
    let base = vec2(tile_size, tile_size);
    let dest_size = base * scale;
    let pos = position.into_pixels_vec2(base) - (dest_size - base) / 2.0;

    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(dest_size),
            rotation,
            ..Default::default()
        },
    );
}

async fn load_tileset_cached(
    cache: &mut HashMap<PathBuf, Arc<TileSet>>,
    rel_path: &str,
) -> Result<Arc<TileSet>> {
    let path = resolve_repo_path(rel_path);
    if let Some(found) = cache.get(&path) {
        return Ok(Arc::clone(found));
    }

    let tileset = Arc::new(
        load_tile_set(&path)
            .await
            .with_context(|| format!("failed to load tileset {}", path.display()))?,
    );
    cache.insert(path, Arc::clone(&tileset));
    Ok(tileset)
}

async fn load_frame_dir(rel_path: &str) -> Result<Arc<Vec<Texture2D>>> {
    let dir = resolve_repo_path(rel_path);
    let mut entries = frame_paths(&dir)?;
    entries.sort();

    let mut frames = Vec::with_capacity(entries.len());
    for entry in entries {
        let path = entry.to_string_lossy().to_string();
        let texture = load_texture(&path)
            .await
            .with_context(|| format!("failed to load frame {}", path))?;
        texture.set_filter(FilterMode::Nearest);
        frames.push(texture);
    }

    if frames.is_empty() {
        return Err(anyhow::anyhow!("no frames found in {}", dir.display()));
    }

    Ok(Arc::new(frames))
}

fn frame_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut result = vec![];
    for entry in fs::read_dir(dir)
        .with_context(|| format!("failed to read frame directory {}", dir.display()))?
    {
        let path = entry?.path();
        if path.extension().map(|e| e.eq_ignore_ascii_case("png")).unwrap_or(false) {
            result.push(path);
        }
    }
    Ok(result)
}

fn current_frame(
    frames: &[Texture2D],
    frame_duration: Duration,
    looped: bool,
    start: Instant,
) -> Option<&Texture2D> {
    if frames.is_empty() {
        return None;
    }

    let frame_ms = frame_duration.as_millis().max(1) as u64;
    let elapsed_ms = start.elapsed().as_millis() as u64;
    let mut index = (elapsed_ms / frame_ms) as usize;

    if looped {
        index %= frames.len();
    } else if index >= frames.len() {
        index = frames.len() - 1;
    }

    frames.get(index)
}
