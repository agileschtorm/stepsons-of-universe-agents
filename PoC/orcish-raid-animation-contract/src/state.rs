use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use macroquad::color::{
    Color, BLACK, DARKGRAY, DARKGREEN, DARKPURPLE, GRAY, GREEN, LIGHTGRAY, ORANGE, RED, YELLOW,
};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_text_ex, TextParams};
use macroquad::shapes::{draw_line, draw_rectangle, draw_rectangle_lines};
use macroquad::text::draw_text;
use macroquad::window::{clear_background, screen_width};
use macroquad_tiled_redux::{MapDirection, MapPoint};
use rand::Rng;
use step_combat::combat_ai::{AiAction, CombatAi, CombatIntel, Combatant};
use step_combat::dumbest_ai::DumbAggro;
use step_combat::hit_probability::HitProbability;
use step_combat::json_store::JsonValueMap;
use step_combat::map_utils::{pt, LocalMapDirection};
use step_combat::pathfinding::{AStar, LevelMapBracket};
use step_engine::items::{MeleeWeapon, RangedPrecision, RangedWeapon, Weapon};
use step_percentage::percentage::Percentage;
use thin_walls::level_map::LevelMap;
use thin_walls::seeds::{ObstacleShape, ShapeObject};
use thin_walls::Walkability;
use thin_walls_visibility::visibility::{field_of_view_set, ToObstacles};

use crate::animation::{AsciiAnimation, AsciiTransition, Either, Loop, TransitionInstant};
use crate::actors::ActorSpawnSpec;
use crate::contract::{
    facing_from_map_direction, facing_from_points, projectile_travel_ms, ActorArchetype, ActorId,
    ActorIntent, OverlayKind, QueuePolicy, PLAYER_DEFAULT_OVERLAYS,
};
use crate::map_gen::{demo_map, HEIGHT, WIDTH};
use crate::runtime::{ActorRequest, AnimationRequest, AnimationRuntime, EffectRequest, EffectRequestKind};
use macroquad_tiled_redux::Tl;

pub const TILE_SIZE: f32 = 24.0;
const HP_SIZE: f32 = 4.0;
const VISION_RANGE: u32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatantTag {
    Player,
    DemoOrcJuno,
    DemoOrcTusk,
    DemoHumanRanger,
}

lazy_static! {
    static ref CHARACTER: AsciiAnimation = AsciiAnimation {
        start: Instant::now(),
        looped: Loop::BackForth,
        glyph: '@',
        transition: AsciiTransition {
            duration: Duration::from_secs(1),
            ab: Either::Transition(
                TransitionInstant {
                    scale: 1.0,
                    color: DARKPURPLE,
                    rotation: 0.3,
                    position: Vec2 { x: 0.0, y: 0.0 },
                },
                TransitionInstant {
                    scale: 1.0,
                    color: DARKPURPLE,
                    rotation: -0.3,
                    position: Vec2 { x: 0.0, y: 0.0 },
                }
            )
        }
    };
}

pub struct AniCombatant {
    who: Combatant,
    archetype: ActorArchetype,
    visual: ActorId,
    tag: CombatantTag,
    #[allow(dead_code)]
    graphic: AsciiAnimation,
}

impl AniCombatant {
    pub fn new(
        who: Combatant,
        archetype: ActorArchetype,
        visual: ActorId,
        graphic: AsciiAnimation,
        tag: CombatantTag,
    ) -> Self {
        Self {
            who,
            archetype,
            visual,
            tag,
            graphic,
        }
    }
}

fn p2tl(p: MapPoint) -> Tl {
    Tl::new(p.x as f32, p.y as f32)
}

#[derive(Default)]
pub struct UiState {
    hit_prob: Option<Percentage>,
    /// It's actually opposite of cover, visibility%
    cover: Option<Percentage>,
}

pub struct State {
    level_map: LevelMap,
    // player: AniCombatant,
    selected_enemy: Option<usize>,
    mobs: Vec<AniCombatant>,
    mob_memory: Vec<JsonValueMap>,
    focus_memory: HashMap<CombatantTag, Tl>,

    vision: Option<HashSet<MapPoint>>,
    presentation: AnimationRuntime,
    player_overlay_state: Vec<OverlayKind>,

    ui: Option<UiState>,
}

type CombatantId = usize;

impl State {
    pub async fn demo_scene() -> Self {
        let player_pos = pt(11, 12);
        let orc_juno_pos = pt(14, 12);
        let orc_tusk_pos = pt(24, 16);
        let human_ranger_pos = pt(20, 10);

        let mut player_ani = CHARACTER.with_now();
        player_ani.transition.ab = Either::Transition(
            TransitionInstant {
                scale: 1.0,
                color: DARKGREEN,
                rotation: 0.0,
                position: Default::default(),
            },
            TransitionInstant {
                scale: 1.2,
                color: DARKGREEN,
                rotation: 0.0,
                position: Default::default(),
            },
        );

        let walls_map = demo_map(WIDTH as usize, HEIGHT as usize);

        let mut presentation = AnimationRuntime::load(TILE_SIZE)
            .await
            .expect("failed to load Orcish Raid PoC animation runtime");
        for line in presentation.validation_log() {
            println!("{line}");
        }

        let player_overlays = PLAYER_DEFAULT_OVERLAYS.to_vec();
        let player_visual = presentation
            .spawn_actor(ActorSpawnSpec {
                archetype: ActorArchetype::HumanMale,
                position: p2tl(player_pos),
                facing: facing_from_map_direction(MapDirection::East),
                overlays: player_overlays.clone(),
            })
            .expect("failed to spawn player visual");

        let mut player = Combatant::new(
            player_pos,
            Weapon::Ranged(RangedWeapon::simple(5, RangedPrecision::new(7, 95.into()))),
        );
        player.hp = 20;

        let mut orc_juno = Combatant::new(orc_juno_pos, Weapon::Melee(MeleeWeapon { attack: 3 }));
        orc_juno.hp = 5;

        let mut orc_tusk = Combatant::new(orc_tusk_pos, Weapon::Melee(MeleeWeapon { attack: 3 }));
        orc_tusk.hp = 5;

        let mut human_ranger = Combatant::new(
            human_ranger_pos,
            Weapon::Ranged(RangedWeapon::simple(2, RangedPrecision::new(7, 90.into()))),
        );
        human_ranger.hp = 4;

        let mut enemies = vec![AniCombatant::new(
            player,
            ActorArchetype::HumanMale,
            player_visual,
            player_ani,
            CombatantTag::Player,
        )];

        for (who, archetype, facing, overlays, tag) in [
            (
                orc_juno,
                ActorArchetype::OrcJuno,
                facing_from_map_direction(MapDirection::West),
                vec![],
                CombatantTag::DemoOrcJuno,
            ),
            (
                orc_tusk,
                ActorArchetype::OrcTusk,
                facing_from_map_direction(MapDirection::West),
                vec![],
                CombatantTag::DemoOrcTusk,
            ),
            (
                human_ranger,
                ActorArchetype::HumanMale,
                facing_from_map_direction(MapDirection::West),
                vec![OverlayKind::Tunic],
                CombatantTag::DemoHumanRanger,
            ),
        ] {
            let visual = presentation
                .spawn_actor(ActorSpawnSpec {
                    archetype,
                    position: p2tl(who.location),
                    facing,
                    overlays,
                })
                .expect("failed to spawn demo visual");
            enemies.push(AniCombatant::new(
                who,
                archetype,
                visual,
                CHARACTER.with_now(),
                tag,
            ));
        }

        let mut r = Self {
            level_map: LevelMap::new_init(
                "demo",
                walls_map.map(|opt| opt.map(|shape| shape.into()).unwrap_or_default()),
                walls_map,
                None,
                HashMap::default(),
                HashMap::default(),
            ),
            selected_enemy: None,
            mobs: enemies,
            mob_memory: vec![Default::default(); 4],
            focus_memory: HashMap::default(),
            vision: None,
            presentation,
            player_overlay_state: player_overlays,
            ui: Default::default(),
        };

        r.update_visions();
        r
    }

    /// Wrong, but will last for some time.
    /// Ideally, we need to draw text by its center coordinate.
    fn text_position_correction(scale: f32) -> Vec2 {
        let font_scale_correction = vec2(-0.35, 0.35) * TILE_SIZE * (scale - 1.0);
        let text_position_correction = vec2(0.18, -0.4) * TILE_SIZE;
        font_scale_correction + text_position_correction
    }

    #[allow(dead_code)]
    fn draw_animation(graphic: &AsciiAnimation, pos: Vec2, when: Instant) {
        let animation = graphic.get_at(when).unwrap_or(graphic.last());

        let mut buf = [0; 4];
        let text = graphic.glyph.encode_utf8(&mut buf);

        let pos = pos + animation.position + Self::text_position_correction(animation.scale);

        draw_text_ex(
            text,
            pos.x,
            pos.y,
            TextParams {
                font_size: 36,
                font_scale: animation.scale,
                color: animation.color,
                rotation: animation.rotation,
                ..Default::default()
            },
        );
    }

    fn draw_hp_bar(who: &Combatant, pos: Vec2) {
        let hp_fraction = (who.hp as f32) / 20.0;
        let mut hp_color = if hp_fraction >= 0.9 {
            GREEN
        } else if hp_fraction >= 0.5 {
            YELLOW
        } else {
            RED
        };
        hp_color.a = 0.4;

        let pos = pos + Self::text_position_correction(1.0);

        draw_rectangle(
            pos.x - TILE_SIZE * 0.18,
            pos.y - TILE_SIZE,
            TILE_SIZE * hp_fraction,
            HP_SIZE,
            hp_color,
        );
    }

    fn draw_character(&self, who: &AniCombatant, frame: Option<Color>) {
        let position = self
            .presentation
            .current_position(who.visual)
            .unwrap_or_else(|| p2tl(who.who.location));
        let p = vec2(position.x * TILE_SIZE, (position.y + 1.0) * TILE_SIZE);

        if let Some(frame) = frame {
            let frame_offset = 4.0;
            draw_rectangle_lines(
                p.x - frame_offset,
                p.y - frame_offset,
                TILE_SIZE + 2.0 * frame_offset,
                -(TILE_SIZE + 2.0 * frame_offset),
                4.0,
                frame,
            );
        }

        Self::draw_hp_bar(&who.who, p);
    }

    pub fn focus_for_tag(&self, tag: CombatantTag) -> Option<Vec2> {
        let position = if let Some(who) = self.mobs.iter().find(|who| who.tag == tag) {
            self.presentation
                .current_position(who.visual)
                .unwrap_or_else(|| p2tl(who.who.location))
        } else {
            *self.focus_memory.get(&tag)?
        };
        Some(vec2(
            (position.x + 0.5) * TILE_SIZE,
            (position.y + 0.5) * TILE_SIZE,
        ))
    }

    #[allow(dead_code)]
    fn draw_tile_walls(&self, pos: MapPoint, w: Walkability) {
        for (d, (p1, p2)) in LocalMapDirection::sides_iter() {
            let p1 = pos + p1;
            let p2 = pos + p2;
            if w & d.into() == d.into() {
                draw_line(
                    p1.x as f32 * TILE_SIZE,
                    p1.y as f32 * TILE_SIZE,
                    p2.x as f32 * TILE_SIZE,
                    p2.y as f32 * TILE_SIZE,
                    3.0,
                    DARKGRAY,
                );
            }
        }
    }

    fn draw_walls(&self) {
        for x in 0..self.level_map.opacity.width {
            for y in 0..self.level_map.opacity.height {
                let w = self.level_map.opacity.get_at(pt(x, y));
                let Some(w) = w else {
                    continue;
                };
                let pos = pt(x, y);

                // let tile_center = (Vec2::from(pos) + vec2(0.5, 0.5)) * TILE_SIZE;
                //
                // match w {
                //     ObstacleShape::Tile => self.draw_tile_walls(pos, Walkability::SOLID),
                //     ObstacleShape::EdgeWall(w) => self.draw_tile_walls(pos, w),
                //     ObstacleShape::BigCircle =>
                //         draw_circle(
                //             tile_center.x,
                //             tile_center.y,
                //             TILE_SIZE * 0.5,
                //             DARKGRAY),
                //     ObstacleShape::SmallCircle =>
                //     draw_circle(
                //         tile_center.x,
                //         tile_center.y,
                //         TILE_SIZE * 0.2,
                //         DARKGRAY),
                //     ObstacleShape::CenterHalfWall(_) => todo!(),
                // }

                let obstacles = w.to_obstacles(Percentage::HUNDRED, pos.into());
                for o in obstacles {
                    draw_line(
                        o.line.start.x * TILE_SIZE,
                        o.line.start.y * TILE_SIZE,
                        o.line.end.x * TILE_SIZE,
                        o.line.end.y * TILE_SIZE,
                        TILE_SIZE * 0.2,
                        DARKGRAY);
                }

            }
        }
    }

    fn draw_hud(&self) -> Option<()> {
        let ui = self.ui.as_ref()?;
        let right_column_width = 200.0;
        if self.selected_enemy.is_some() {
            draw_text(
                "Target: Orc",
                screen_width() - right_column_width,
                TILE_SIZE,
                36.0,
                ORANGE,
            );
        }
        if let Some(hit_prob) = ui.hit_prob {
            draw_text(
                &format!("To hit: {}", hit_prob),
                screen_width() - right_column_width,
                TILE_SIZE * 2.0,
                36.0,
                ORANGE,
            );
        }
        if let Some(cover) = ui.cover {
            draw_text(
                &format!("My cover: {}", Percentage::HUNDRED - cover),
                screen_width() - right_column_width,
                TILE_SIZE * 3.0,
                36.0,
                ORANGE,
            );
        }

        None
    }

    pub fn draw(&mut self) {
        clear_background(GRAY);

        for x in 0..self.level_map.tiles.width {
            for y in 0..self.level_map.tiles.height {
                let pos = pt(x, y);

                if self.vision.is_some() && self.vision.as_ref().unwrap().contains(&pos) {
                    draw_rectangle(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        LIGHTGRAY,
                    );
                }
            }
        }

        self.draw_walls();

        self.presentation.draw_underlays();
        self.presentation.draw_actors();

        self.draw_character(&self.mobs[0], None);

        for (i, enemy) in self.mobs.iter().enumerate().skip(1) {
            let frame = if self.selected_enemy == Some(i) {
                // TODO: Use hit% to choose frame color.
                // Some(Color::new(0.06, 1.0, 0.4, 0.4))
                Some(DARKGREEN)
            } else {
                None
            };
            self.draw_character(enemy, frame);
        }

        self.presentation.draw_overlays();

        draw_text(
            &format!("{}", self.mobs[0].who.location),
            0.0,
            24.0,
            24.0,
            BLACK,
        );

        self.draw_hud();
    }

    // No, we don't need it in minimal example. Just missed, we won't be hitting furniture yet.
    // /// Returns: the `MapPoint` where the bullet hits. You decide if it flies further.
    // fn shot_hit<T>()

    fn get_target(&self) -> Option<&Combatant> {
        self.mobs.get(self.selected_enemy?).map(|ac| &ac.who)
    }

    fn calc_player_hit_probability(&self) -> Option<(Percentage, Percentage)> {
        self.calc_hit_probability(&self.mobs[0].who, self.get_target()?)
    }

    fn calc_hit_probability(
        &self,
        source: &Combatant,
        target: &Combatant,
    ) -> Option<(Percentage, Percentage)> {
        let other_obstacles = self
            .mobs
            .iter()
            .filter(|loc| {
                loc.who.location != target.location && loc.who.location != source.location
            })
            .map(|e| (e.who.location, ObstacleShape::SmallCircle))
            .collect();

        let bracket_map = LevelMapBracket::new(&self.level_map, other_obstacles);
        bracket_map.calc_hit_probability(source, target)
    }

    fn movement_tl(direction: MapDirection) -> Tl {
        match direction {
            MapDirection::North => Tl::new(0.0, -1.0),
            MapDirection::East => Tl::new(1.0, 0.0),
            MapDirection::South => Tl::new(0.0, 1.0),
            MapDirection::West => Tl::new(-1.0, 0.0),
        }
    }

    fn queue_actor(
        &mut self,
        who: CombatantId,
        intent: ActorIntent,
        facing: crate::contract::Facing8,
        movement: Tl,
        queue: QueuePolicy,
        delay_ms: u64,
    ) {
        self.presentation.submit(AnimationRequest::Actor(ActorRequest {
            actor_id: self.mobs[who].visual,
            intent,
            facing,
            movement,
            queue,
            compression: None,
            snap_to: None,
            delay_ms,
        }));
    }

    fn queue_effect(&mut self, kind: EffectRequestKind, delay_ms: u64) {
        self.presentation
            .submit(AnimationRequest::Effect(EffectRequest { kind, delay_ms }));
    }

    fn queue_walk(&mut self, who: CombatantId, direction: MapDirection) {
        self.queue_actor(
            who,
            ActorIntent::Walk,
            facing_from_map_direction(direction),
            Self::movement_tl(direction),
            QueuePolicy::Append,
            0,
        );
    }

    fn queue_ranged_attack(&mut self, shooter: CombatantId, target: CombatantId) -> u64 {
        let target_location = self.mobs[target].who.location;
        let shooter_location = self.mobs[shooter].who.location;
        let facing = facing_from_points(shooter_location, target_location);
        let travel_ms = projectile_travel_ms(p2tl(shooter_location), p2tl(target_location));
        let projectile_delay_ms = 80;
        let hit_delay_ms = projectile_delay_ms + travel_ms;

        self.queue_actor(
            shooter,
            ActorIntent::RangedAttack,
            facing,
            Tl::new(0.0, 0.0),
            QueuePolicy::Interrupt,
            0,
        );
        self.queue_effect(
            EffectRequestKind::ProjectileTravel {
                from: p2tl(shooter_location),
                to: p2tl(target_location),
            },
            projectile_delay_ms,
        );
        self.queue_effect(
            EffectRequestKind::ProjectileImpact {
                at: p2tl(target_location),
            },
            hit_delay_ms,
        );

        hit_delay_ms
    }

    fn queue_melee_attack(&mut self, attacker: CombatantId, target: CombatantId) -> u64 {
        let attacker_location = self.mobs[attacker].who.location;
        let target_location = self.mobs[target].who.location;
        let facing = facing_from_points(attacker_location, target_location);
        let hit_delay_ms = 110;

        self.queue_actor(
            attacker,
            ActorIntent::MeleeAttack,
            facing,
            Tl::new(0.0, 0.0),
            QueuePolicy::Interrupt,
            0,
        );
        self.queue_effect(
            EffectRequestKind::MeleeSwing {
                from: p2tl(attacker_location),
                to: p2tl(target_location),
            },
            70,
        );

        hit_delay_ms
    }

    fn queue_damage_feedback(
        &mut self,
        attacker: CombatantId,
        target: CombatantId,
        hit_delay_ms: u64,
        will_die: bool,
    ) {
        let attacker_location = self.mobs[attacker].who.location;
        let target_location = self.mobs[target].who.location;
        let facing = facing_from_points(attacker_location, target_location);

        self.queue_actor(
            target,
            ActorIntent::Hurt,
            facing,
            Tl::new(0.0, 0.0),
            QueuePolicy::Interrupt,
            hit_delay_ms,
        );
        self.queue_effect(
            EffectRequestKind::BloodHit {
                at: p2tl(target_location),
            },
            hit_delay_ms,
        );

        if will_die {
            let die_delay_ms = hit_delay_ms + 120;
            self.queue_actor(
                target,
                ActorIntent::Die,
                facing,
                Tl::new(0.0, 0.0),
                QueuePolicy::Interrupt,
                die_delay_ms,
            );

            let die_duration_ms = self
                .presentation
                .clip_duration_ms(self.mobs[target].archetype, ActorIntent::Die, facing)
                .unwrap_or(500);

            self.queue_effect(
                EffectRequestKind::Corpse {
                    at: p2tl(target_location),
                },
                die_delay_ms + die_duration_ms.saturating_sub(120),
            );
            self.presentation.submit(AnimationRequest::RemoveActor {
                actor_id: self.mobs[target].visual,
                delay_ms: die_delay_ms + die_duration_ms,
            });
        }
    }

    /// returns: remaining HP.
    fn hurt(&mut self, who: CombatantId, weapon: Weapon) -> i16 {
        self.mobs[who].who.hp -= weapon.attack();

        self.mobs[who].who.hp
    }

    fn kill_enemy(&mut self, enemy_index: usize) {
        if let Some(enemy) = self.mobs.get(enemy_index) {
            let focus = self
                .presentation
                .current_position(enemy.visual)
                .unwrap_or_else(|| p2tl(enemy.who.location));
            self.focus_memory.insert(enemy.tag, focus);
        }
        self.mobs.remove(enemy_index);
        self.mob_memory.remove(enemy_index);
        self.vision = None;
        self.selected_enemy = None;
        self.update_visions();
    }

    fn toggle_player_overlay(&mut self, overlay: OverlayKind) {
        let enabled = !self.player_overlay_state.contains(&overlay);
        self.set_player_overlay_enabled(overlay, enabled);
    }

    pub fn set_player_overlay_enabled(&mut self, overlay: OverlayKind, enabled: bool) {
        if enabled {
            if !self.player_overlay_state.contains(&overlay) {
                self.player_overlay_state.push(overlay);
            }
        } else {
            self.player_overlay_state.retain(|it| *it != overlay);
        }
        self.presentation.submit(AnimationRequest::SetOverlays {
            actor_id: self.mobs[0].visual,
            overlays: self.player_overlay_state.clone(),
        });
    }

    fn combatant_index(&self, tag: CombatantTag) -> Option<usize> {
        self.mobs.iter().position(|who| who.tag == tag)
    }

    fn move_actor_without_ai(&mut self, who: CombatantId, direction: MapDirection) -> bool {
        if !self.level_map.can_walk(self.mobs[who].who.location, direction) {
            return false;
        }

        let next_location = self.mobs[who].who.location + direction.into();
        let enemy_index = self
            .mobs
            .iter()
            .enumerate()
            .find(|(index, it)| *index != who && it.who.location == next_location)
            .map(|(index, _)| index);

        match enemy_index {
            Some(enemy_index) => {
                let hit_delay_ms = self.queue_melee_attack(who, enemy_index);
                let hp_left = self.hurt(enemy_index, self.mobs[who].who.weapon);
                self.queue_damage_feedback(who, enemy_index, hit_delay_ms, hp_left <= 0);
                if hp_left <= 0 {
                    self.kill_enemy(enemy_index);
                }
                true
            }
            None => {
                self.queue_walk(who, direction);
                self.mobs[who].who.location = next_location;
                if who == 0 {
                    self.vision = None;
                }
                true
            }
        }
    }

    fn ranged_attack_without_ai(&mut self, shooter: CombatantId, target: CombatantId, is_hit: bool) -> bool {
        let hit_delay_ms = self.queue_ranged_attack(shooter, target);
        if is_hit {
            let hp_left = self.hurt(target, self.mobs[shooter].who.weapon);
            self.queue_damage_feedback(shooter, target, hit_delay_ms, hp_left <= 0);
            if hp_left <= 0 {
                self.kill_enemy(target);
            }
        }
        true
    }

    fn melee_attack_without_ai(&mut self, attacker: CombatantId, target: CombatantId, is_hit: bool) -> bool {
        let hit_delay_ms = self.queue_melee_attack(attacker, target);
        if is_hit {
            let hp_left = self.hurt(target, self.mobs[attacker].who.weapon);
            self.queue_damage_feedback(attacker, target, hit_delay_ms, hp_left <= 0);
            if hp_left <= 0 {
                self.kill_enemy(target);
            }
        }
        true
    }

    pub fn scripted_move(&mut self, who: CombatantTag, direction: MapDirection) {
        let Some(who) = self.combatant_index(who) else {
            return;
        };
        if self.move_actor_without_ai(who, direction) {
            self.ui = None;
            self.update_visions();
        }
    }

    pub fn scripted_ranged_attack(&mut self, shooter: CombatantTag, target: CombatantTag) {
        let Some(shooter) = self.combatant_index(shooter) else {
            return;
        };
        let Some(target) = self.combatant_index(target) else {
            return;
        };
        if self.ranged_attack_without_ai(shooter, target, true) {
            self.ui = None;
            self.update_visions();
        }
    }

    pub fn scripted_melee_attack(&mut self, attacker: CombatantTag, target: CombatantTag) {
        let Some(attacker) = self.combatant_index(attacker) else {
            return;
        };
        let Some(target) = self.combatant_index(target) else {
            return;
        };
        if self.melee_attack_without_ai(attacker, target, true) {
            self.ui = None;
            self.update_visions();
        }
    }

    fn get_visible_enemies(&self) -> Option<Vec<usize>> {
        if let Some(vision) = &self.vision {
            Some(
                self.mobs
                    .iter()
                    .enumerate()
                    .skip(1)
                    .filter_map(|(i, e)| {
                        if vision.contains(&e.who.location) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        } else {
            println!("Error: vision is None");
            None
        }
    }

    fn update_selected(&mut self) {
        let visible_enemies = self.get_visible_enemies();
        if visible_enemies.is_none() {
            println!("error: visibility shall not be None");
            self.ui = None;
            return;
        }
        let visible_enemies = visible_enemies.unwrap();
        if let Some(selected) = self.selected_enemy {
            if !visible_enemies.contains(&selected) {
                self.selected_enemy = visible_enemies.get(0).cloned();
                self.ui = None;
            }
        } else {
            self.selected_enemy = visible_enemies.get(0).cloned();
            self.ui = None;
        }
    }

    fn next_enemy(&self, direction: isize) -> Option<usize> {
        let visible_enemies = self.get_visible_enemies();
        if visible_enemies.is_none() {
            println!("error: visibility shall not be None");
            return None;
        }

        let visible_enemies = visible_enemies.unwrap();

        match self.selected_enemy {
            None => visible_enemies.get(0).cloned(),
            Some(selected) => match visible_enemies.iter().position(|it| *it == selected) {
                None => visible_enemies.get(0).cloned(),
                Some(pos) => {
                    let next_index = (pos as isize + direction) % visible_enemies.len() as isize;
                    visible_enemies.get(next_index as usize).cloned()
                }
            },
        }
    }

    pub fn input(&mut self) {
        let mut movement = None;
        let mut turn_used = false;
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            movement = Some(MapDirection::South);
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            movement = Some(MapDirection::North);
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            movement = Some(MapDirection::West);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            movement = Some(MapDirection::East);
        }
        if is_key_pressed(KeyCode::Period) || is_key_pressed(KeyCode::Space) {
            turn_used = true;
        }

        if is_key_pressed(KeyCode::Tab) {
            let shift_down = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
            let direction = if shift_down { -1 } else { 1 };
            self.selected_enemy = self.next_enemy(direction);
            println!("Next enemy: {:?}", self.selected_enemy);
            self.ui = None;
        }

        if is_key_pressed(KeyCode::Key1) {
            self.toggle_player_overlay(OverlayKind::Cape);
        }
        if is_key_pressed(KeyCode::Key2) {
            self.toggle_player_overlay(OverlayKind::Tunic);
        }

        if is_key_pressed(KeyCode::F) {
            if let Some(selected_enemy) = self.selected_enemy {
                if self.mobs.get(selected_enemy).is_some() {
                    if let Some((hit_prob, _cover)) = self.calc_player_hit_probability() {
                        let mut rng = rand::rng();
                        let is_hit = rng.random_range(0..100) < hit_prob.percent;

                        let hit_delay_ms = self.queue_ranged_attack(0, selected_enemy);
                        if is_hit {
                            let hp_left = self.hurt(selected_enemy, self.mobs[0].who.weapon);
                            self.queue_damage_feedback(0, selected_enemy, hit_delay_ms, hp_left <= 0);
                            if hp_left <= 0 {
                                self.kill_enemy(selected_enemy);
                                println!("Guess you are not coming back soon.");
                            } else {
                                println!("How does the bullet taste?");
                            }
                        } else {
                            println!("Miss!");
                        }

                        turn_used = true;
                    }
                } else {
                    println!("Shoot who exactly?");
                }
            }
        }

        if let Some(movement) = movement {
            if self.level_map.can_walk(self.mobs[0].who.location, movement) {
                turn_used = true;

                let next_location = self.mobs[0].who.location + movement.into();

                let enemy_index = self
                    .mobs
                    .iter_mut()
                    .position(|it| it.who.location == next_location);

                match enemy_index {
                    Some(enemy_index) => {
                        let hit_delay_ms = self.queue_melee_attack(0, enemy_index);
                        let hp_left = self.hurt(enemy_index, self.mobs[0].who.weapon);
                        self.queue_damage_feedback(0, enemy_index, hit_delay_ms, hp_left <= 0);
                        if hp_left <= 0 {
                            self.kill_enemy(enemy_index);
                            println!("Good night, baby!");
                        } else {
                            println!("Take that");
                        }
                    }
                    None => {
                        self.queue_walk(0, movement);
                        self.mobs[0].who.location = (&self.mobs[0]).who.location + movement.into();
                        self.vision = None;
                    }
                }
            } else {
                println!("Ouch!");
            }
        }

        if turn_used {
            self.ui = None;
            self.move_mobs();
            self.update_visions();
        }
    }

    pub fn update_visions(&mut self) {
        let enemies_iter = self
            .mobs
            .iter()
            .skip(1)
            .map(|e| ShapeObject::new(e.who.location, ObstacleShape::SmallCircle));

        self.vision = Some(field_of_view_set(
            self.mobs[0].who.location,
            VISION_RANGE as isize,
            &self.level_map,
            enemies_iter,
        ));

        self.update_selected();

        if self.ui.is_none() {
            let Some(target) = self.get_target() else {
                return;
            };

            let (_me_prob, my_cover) = self.calc_hit_probability(target, &self.mobs[0].who).unzip();

            let (hit_prob, _enemy_cover) =
                self.calc_hit_probability(&self.mobs[0].who, target).unzip();

            self.ui = Some(UiState {
                hit_prob,
                cover: my_cover,
            });
        }
    }

    fn move_mob(&mut self, mob: CombatantId, path: Vec<MapPoint>) -> Option<MapPoint> {
        if let Some(&next_pos) = path.get(1) {
            if next_pos != self.mobs[0].who.location {
                // Otherwise they can step into the same tile
                let direction = next_pos - self.mobs[mob].who.location;
                let map_direction = if direction.x > 0 {
                    MapDirection::East
                } else if direction.x < 0 {
                    MapDirection::West
                } else if direction.y > 0 {
                    MapDirection::South
                } else {
                    MapDirection::North
                };
                self.queue_walk(mob, map_direction);
                (&mut self.mobs[mob]).who.location = next_pos;
                return Some(next_pos);
            } else {
                let hit_delay_ms = self.queue_melee_attack(mob, 0);
                let hp_left = self.hurt(0, self.mobs[mob].who.weapon);
                self.queue_damage_feedback(mob, 0, hit_delay_ms, false);
                if hp_left <= 0 {
                    println!("Oh no, I'm too snarky to die!");
                    // TODO: Game over mode.
                    self.mobs[0].who.hp = 2;
                } else {
                    println!("Damn, that hurt.");
                }
            }
        }

        None
    }

    pub fn move_mobs(&mut self) {
        let orcs: Vec<_> = self.mobs.iter().skip(1).map(|ac| ac.who).collect();
        let mut orcs_positions: HashSet<MapPoint> = orcs.iter().map(|e| e.location).collect();
        let orcs_shapes: HashSet<ShapeObject> = orcs
            .iter()
            .map(|e| ShapeObject::new(e.location, ObstacleShape::SmallCircle))
            .collect();
        let player_location_iter = Some(ShapeObject::new(
            self.mobs[0].who.location,
            ObstacleShape::SmallCircle,
        ));

        for mob_index in 1..self.mobs.len() {
            let mob_location = self.mobs[mob_index].who.location;
            let vision = field_of_view_set(
                mob_location,
                VISION_RANGE as isize,
                &self.level_map,
                orcs_shapes.iter().cloned().chain(player_location_iter),
            );

            let mut enemies = vec![];
            if vision.contains(&self.mobs[0].who.location) {
                enemies.push(self.mobs[0].who)
            }

            let intel = CombatIntel {
                map: &self.level_map,
                me: self.mobs[mob_index].who,
                enemies: &enemies,
                allies: &orcs
                    .iter()
                    .filter(|it| it.location != mob_location && vision.contains(&it.location))
                    .cloned()
                    .collect(),
                extra_objects: &vec![],
            };

            let action = DumbAggro::next_action(&mut self.mob_memory[mob_index], intel);

            match action {
                AiAction::Move(to) => {
                    let path = self
                        .level_map
                        .find_path(mob_location, to, orcs_positions.clone());
                    if let Some(path) = path {
                        if let Some(next_pos) = self.move_mob(mob_index, path) {
                            orcs_positions.remove(&mob_location);
                            orcs_positions.insert(next_pos);
                        }
                    }
                }

                AiAction::Attack(weapon, target) => {
                    if let Some((hit_prob, _cover)) =
                        self.calc_hit_probability(&self.mobs[mob_index].who, &self.mobs[0].who)
                    {
                        if hit_prob <= 1.into() {
                            println!(
                                "Wow, why did I shoot them again at {} probability?",
                                hit_prob
                            );
                        }

                        let mut rng = rand::rng();
                        let is_hit = rng.random_range(0..100) < hit_prob.percent;
                        // FIXME: Something's wrong here, sometimes orcs shoot through the walls.

                        // So far, the code is only ready to attack the player, but it doesn't have
                        // to be that way.
                        assert_eq!(target, self.mobs[0].who.location);

                        let hit_delay_ms = match weapon {
                            Weapon::Melee(_) | Weapon::Unarmed(_) => {
                                self.queue_melee_attack(mob_index, 0)
                            }
                            Weapon::Ranged(_) => self.queue_ranged_attack(mob_index, 0),
                        };

                        if is_hit {
                            let hp_left = self.hurt(0, self.mobs[mob_index].who.weapon);
                            self.queue_damage_feedback(mob_index, 0, hit_delay_ms, false);
                            if hp_left <= 0 {
                                println!("Oh noes, I'm too snarky to die!");
                                // TODO: Game over mode.
                                self.mobs[0].who.hp = 2;
                            } else {
                                println!("Damn, that hurts.");
                            }
                        } else {
                            println!("Lucky break.");
                        }
                    } else {
                        println!("You thought you can hit me?");
                    }
                }

                // Nothing to do here, until we can Aim.
                AiAction::Wait(_) => {}
                AiAction::Reload => println!("Reload not implemented for bows"),
            }
        }
    }

    pub fn gc(&mut self) {
        self.presentation.update();
    }
}
