use std::time::Instant as StdInstant;

use anyhow::Result;
use macroquad_tiled_redux::Tl;

use crate::actors::{ActorClipRequest, ActorSpawnSpec, ActorSystem};
use crate::contract::{
    ActorArchetype, ActorId, ActorIntent, Facing8, OverlayKind, QueuePolicy,
};
use crate::fx::FxSystem;

#[derive(Debug, Clone)]
pub enum AnimationRequest {
    Actor(ActorRequest),
    Effect(EffectRequest),
    SetOverlays {
        actor_id: ActorId,
        overlays: Vec<OverlayKind>,
    },
    RemoveActor {
        actor_id: ActorId,
        delay_ms: u64,
    },
}

#[derive(Debug, Clone)]
pub struct ActorRequest {
    pub actor_id: ActorId,
    pub intent: ActorIntent,
    pub facing: Facing8,
    pub movement: Tl,
    pub queue: QueuePolicy,
    pub compression: Option<u32>,
    pub snap_to: Option<Tl>,
    pub delay_ms: u64,
}

#[derive(Debug, Clone)]
pub struct EffectRequest {
    pub kind: EffectRequestKind,
    pub delay_ms: u64,
}

#[derive(Debug, Clone)]
pub enum EffectRequestKind {
    ProjectileTravel { from: Tl, to: Tl },
    ProjectileImpact { at: Tl },
    BloodHit { at: Tl },
    Corpse { at: Tl },
    MeleeSwing { from: Tl, to: Tl },
}

struct PendingRequest {
    execute_at: StdInstant,
    request: AnimationRequest,
}

pub struct AnimationRuntime {
    actors: ActorSystem,
    fx: FxSystem,
    pending: Vec<PendingRequest>,
    validation_log: Vec<String>,
}

impl AnimationRuntime {
    pub async fn load(tile_size: f32) -> Result<Self> {
        let actors = ActorSystem::load(tile_size).await?;
        let fx = FxSystem::load(tile_size).await?;

        let mut validation_log = actors.validation_log().to_vec();
        validation_log.push("validated effect bindings".to_string());

        Ok(Self {
            actors,
            fx,
            pending: vec![],
            validation_log,
        })
    }

    pub fn validation_log(&self) -> &[String] {
        &self.validation_log
    }

    pub fn spawn_actor(&mut self, spec: ActorSpawnSpec) -> Result<ActorId> {
        self.actors.spawn(spec)
    }

    pub fn clip_duration_ms(
        &self,
        archetype: ActorArchetype,
        intent: ActorIntent,
        facing: Facing8,
    ) -> Option<u64> {
        self.actors.clip_duration_ms(archetype, intent, facing)
    }

    pub fn current_position(&self, actor_id: ActorId) -> Option<Tl> {
        self.actors.current_position(actor_id)
    }

    pub fn submit(&mut self, request: AnimationRequest) {
        let delay_ms = match &request {
            AnimationRequest::Actor(request) => request.delay_ms,
            AnimationRequest::Effect(request) => request.delay_ms,
            AnimationRequest::SetOverlays { .. } => 0,
            AnimationRequest::RemoveActor { delay_ms, .. } => *delay_ms,
        };

        self.pending.push(PendingRequest {
            execute_at: StdInstant::now() + std::time::Duration::from_millis(delay_ms),
            request,
        });
    }

    pub fn update(&mut self) {
        let now = StdInstant::now();
        let mut ready = vec![];
        let mut pending = Vec::with_capacity(self.pending.len());

        for request in self.pending.drain(..) {
            if request.execute_at <= now {
                ready.push(request.request);
            } else {
                pending.push(request);
            }
        }
        self.pending = pending;

        for request in ready {
            self.apply(request);
        }

        self.actors.update();
        self.fx.gc();
    }

    pub fn draw_underlays(&mut self) {
        self.fx.draw_underlays();
    }

    pub fn draw_actors(&self) {
        self.actors.draw();
    }

    pub fn draw_overlays(&mut self) {
        self.fx.draw_overlays();
    }

    fn apply(&mut self, request: AnimationRequest) {
        match request {
            AnimationRequest::Actor(request) => {
                let _ = self.actors.apply(ActorClipRequest {
                    actor_id: request.actor_id,
                    intent: request.intent,
                    facing: request.facing,
                    movement: request.movement,
                    queue: request.queue,
                    compression: request.compression,
                    snap_to: request.snap_to,
                });
            }
            AnimationRequest::Effect(request) => match request.kind {
                EffectRequestKind::ProjectileTravel { from, to } => self.fx.spawn_projectile(from, to),
                EffectRequestKind::ProjectileImpact { at } => self.fx.spawn_impact(at),
                EffectRequestKind::BloodHit { at } => self.fx.spawn_blood_hit(at),
                EffectRequestKind::Corpse { at } => self.fx.spawn_corpse(at),
                EffectRequestKind::MeleeSwing { from, to } => self.fx.spawn_melee_swing(from, to),
            },
            AnimationRequest::SetOverlays { actor_id, overlays } => {
                self.actors.set_overlays(actor_id, overlays);
            }
            AnimationRequest::RemoveActor { actor_id, .. } => {
                self.actors.remove_now(actor_id);
            }
        }
    }
}
