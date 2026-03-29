use std::time::{Duration, Instant};

use macroquad::color::{Color, BLACK, RED, WHITE};
use macroquad::input::get_last_key_pressed;
use macroquad::math::Vec2;
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_rectangle};
use macroquad::text::{draw_text, measure_text};
use macroquad::window::{screen_height, screen_width};
use macroquad_tiled_redux::MapDirection;

use crate::contract::OverlayKind;
use crate::state::{CombatantTag, State};

pub enum DemoFlow {
    Continue,
    Restart,
}

pub struct DemoController {
    steps: Vec<DemoStep>,
    step_index: usize,
    next_step_at: Instant,
}

struct DemoStep {
    duration_ms: u64,
    callout: DemoCallout,
    action: DemoAction,
}

struct DemoCallout {
    title: &'static str,
    body: &'static str,
    focuses: Vec<DemoFocus>,
}

enum DemoFocus {
    Combatant(CombatantTag),
}

enum DemoAction {
    None,
    SetPlayerOverlay {
        overlay: OverlayKind,
        enabled: bool,
    },
    Move {
        who: CombatantTag,
        direction: MapDirection,
    },
    RangedAttack {
        shooter: CombatantTag,
        target: CombatantTag,
    },
    MeleeAttack {
        attacker: CombatantTag,
        target: CombatantTag,
    },
}

impl DemoController {
    pub fn new() -> Self {
        let steps = demo_script();
        let next_step_at = Instant::now() + Duration::from_millis(steps[0].duration_ms);
        Self {
            steps,
            step_index: 0,
            next_step_at,
        }
    }

    pub fn should_take_over(&self) -> bool {
        get_last_key_pressed().is_some()
    }

    pub fn update(&mut self, state: &mut State) -> DemoFlow {
        if Instant::now() < self.next_step_at {
            return DemoFlow::Continue;
        }

        if self.step_index + 1 >= self.steps.len() {
            return DemoFlow::Restart;
        }

        self.step_index += 1;
        self.steps[self.step_index].action.run(state);
        self.next_step_at = Instant::now() + Duration::from_millis(self.steps[self.step_index].duration_ms);
        DemoFlow::Continue
    }

    pub fn draw_overlay(&self, state: &State) {
        let step = &self.steps[self.step_index];

        for focus in &step.callout.focuses {
            let Some(center) = focus.center(state) else {
                continue;
            };
            draw_glow_circle(center, 26.0);
        }

        draw_callout_box(
            "Demo mode. Press any key to play this same scene.",
            step.callout.title,
            step.callout.body,
        );
    }
}

impl DemoAction {
    fn run(&self, state: &mut State) {
        match self {
            Self::None => {}
            Self::SetPlayerOverlay { overlay, enabled } => {
                state.set_player_overlay_enabled(*overlay, *enabled);
            }
            Self::Move { who, direction } => state.scripted_move(*who, *direction),
            Self::RangedAttack { shooter, target } => state.scripted_ranged_attack(*shooter, *target),
            Self::MeleeAttack { attacker, target } => state.scripted_melee_attack(*attacker, *target),
        }
    }
}

impl DemoFocus {
    fn center(&self, state: &State) -> Option<Vec2> {
        match self {
            Self::Combatant(tag) => state.focus_for_tag(*tag),
        }
    }
}

pub fn draw_play_hint() {
    draw_callout_box(
        "Play mode.",
        "You are in control now.",
        "WASD move, F shoot, Tab target, 1/2 toggle overlays, R restart, F1 demo, Q quit.",
    );
}

fn draw_callout_box(mode_line: &str, title: &str, body: &str) {
    let box_x = 16.0;
    let box_w = (screen_width() - 32.0).min(760.0);
    let text_x = box_x + 14.0;
    let text_width = box_w - 28.0;
    let body_lines = wrap_text(body, text_width, 22, 1.0);
    let lines = body_lines.len() as f32;
    let box_h = 88.0 + lines * 24.0;
    let box_y = screen_height() - box_h - 20.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.05, 0.02, 0.02, 0.84));
    draw_rectangle(box_x + 4.0, box_y + 4.0, box_w - 8.0, 34.0, Color::new(0.35, 0.05, 0.05, 0.55));

    draw_text(mode_line, text_x, box_y + 24.0, 22.0, WHITE);
    draw_text(title, text_x, box_y + 56.0, 28.0, WHITE);

    for (index, line) in body_lines.iter().enumerate() {
        draw_text(
            line,
            text_x,
            box_y + 88.0 + index as f32 * 22.0,
            22.0,
            WHITE,
        );
    }
}

fn wrap_text(text: &str, max_width: f32, font_size: u16, font_scale: f32) -> Vec<String> {
    let mut wrapped = vec![];

    for paragraph in text.split('\n') {
        let mut current = String::new();

        for word in paragraph.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };

            if measure_text(&candidate, None, font_size, font_scale).width <= max_width {
                current = candidate;
            } else {
                if !current.is_empty() {
                    wrapped.push(current);
                }
                current = word.to_string();
            }
        }

        if !current.is_empty() {
            wrapped.push(current);
        } else {
            wrapped.push(String::new());
        }
    }

    wrapped
}

fn draw_glow_circle(center: Vec2, radius: f32) {
    for (index, alpha) in [0.16, 0.11, 0.07].iter().enumerate() {
        draw_circle(
            center.x,
            center.y,
            radius + index as f32 * 9.0,
            Color::new(1.0, 0.1, 0.1, *alpha),
        );
    }

    for (index, alpha) in [0.9, 0.7, 0.45].iter().enumerate() {
        draw_circle_lines(
            center.x,
            center.y,
            radius + index as f32 * 6.0,
            2.0,
            Color::new(1.0, 0.35, 0.35, *alpha),
        );
    }

    draw_circle(center.x, center.y, 3.5, RED);
    draw_circle(center.x, center.y, 1.5, BLACK);
}

fn demo_script() -> Vec<DemoStep> {
    vec![
        DemoStep {
            duration_ms: 1700,
            callout: DemoCallout {
                title: "This is a real scripted run.",
                body: "The demo uses the same game state and animation bus as normal play.\nThe red circles show where to look.",
                focuses: vec![
                    DemoFocus::Combatant(CombatantTag::Player),
                    DemoFocus::Combatant(CombatantTag::DemoOrcJuno),
                    DemoFocus::Combatant(CombatantTag::DemoHumanRanger),
                ],
            },
            action: DemoAction::None,
        },
        DemoStep {
            duration_ms: 1300,
            callout: DemoCallout {
                title: "Overlays are layered on top.",
                body: "The player starts with a cape. The demo adds a tunic on top of the same actor.",
                focuses: vec![DemoFocus::Combatant(CombatantTag::Player)],
            },
            action: DemoAction::SetPlayerOverlay {
                overlay: OverlayKind::Tunic,
                enabled: true,
            },
        },
        DemoStep {
            duration_ms: 1100,
            callout: DemoCallout {
                title: "Walk is a gameplay intent.",
                body: "The human actor gets a Walk request and answers it with its own walk clip.",
                focuses: vec![DemoFocus::Combatant(CombatantTag::Player)],
            },
            action: DemoAction::Move {
                who: CombatantTag::Player,
                direction: MapDirection::East,
            },
        },
        DemoStep {
            duration_ms: 1100,
            callout: DemoCallout {
                title: "The orc gets the same Walk intent.",
                body: "Same request, different actor set, different clip mapping.",
                focuses: vec![DemoFocus::Combatant(CombatantTag::DemoOrcJuno)],
            },
            action: DemoAction::Move {
                who: CombatantTag::DemoOrcJuno,
                direction: MapDirection::West,
            },
        },
        DemoStep {
            duration_ms: 1800,
            callout: DemoCallout {
                title: "Ranged attack is a timed sequence.",
                body: "Attack clip, projectile travel, impact, and hurt happen in order.\nThe player hurt clip has overlay fallback, so cape and tunic stay visible.",
                focuses: vec![
                    DemoFocus::Combatant(CombatantTag::DemoHumanRanger),
                    DemoFocus::Combatant(CombatantTag::Player),
                ],
            },
            action: DemoAction::RangedAttack {
                shooter: CombatantTag::DemoHumanRanger,
                target: CombatantTag::Player,
            },
        },
        DemoStep {
            duration_ms: 1500,
            callout: DemoCallout {
                title: "Melee uses the same semantic request too.",
                body: "The orc attacks with the same MeleeAttack intent, but its own clip mapping.",
                focuses: vec![
                    DemoFocus::Combatant(CombatantTag::DemoOrcJuno),
                    DemoFocus::Combatant(CombatantTag::Player),
                ],
            },
            action: DemoAction::MeleeAttack {
                attacker: CombatantTag::DemoOrcJuno,
                target: CombatantTag::Player,
            },
        },
        DemoStep {
            duration_ms: 2000,
            callout: DemoCallout {
                title: "The player answers the same melee request differently.",
                body: "Now the human actor uses its own melee clip.\nThe orc plays die, then the corpse stays behind.",
                focuses: vec![
                    DemoFocus::Combatant(CombatantTag::Player),
                    DemoFocus::Combatant(CombatantTag::DemoOrcJuno),
                ],
            },
            action: DemoAction::MeleeAttack {
                attacker: CombatantTag::Player,
                target: CombatantTag::DemoOrcJuno,
            },
        },
        DemoStep {
            duration_ms: 2000,
            callout: DemoCallout {
                title: "The same bus drives another actor set too.",
                body: "This shot kills the human ranger.\nDeath and corpse timing are sequenced, not hardcoded all over the game.",
                focuses: vec![
                    DemoFocus::Combatant(CombatantTag::Player),
                    DemoFocus::Combatant(CombatantTag::DemoHumanRanger),
                ],
            },
            action: DemoAction::RangedAttack {
                shooter: CombatantTag::Player,
                target: CombatantTag::DemoHumanRanger,
            },
        },
        DemoStep {
            duration_ms: 1800,
            callout: DemoCallout {
                title: "Looping back to the start.",
                body: "Press any key if you want to take over and play this same scene yourself.",
                focuses: vec![DemoFocus::Combatant(CombatantTag::Player)],
            },
            action: DemoAction::None,
        },
    ]
}
