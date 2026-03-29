use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use macroquad::math::Vec2;
use macroquad_tiled_redux::{MapDirection, MapPoint, Tl};

pub type ActorId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorArchetype {
    HumanMale,
    OrcJuno,
    OrcTusk,
}

impl Display for ActorArchetype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HumanMale => write!(f, "HumanMale"),
            Self::OrcJuno => write!(f, "OrcJuno"),
            Self::OrcTusk => write!(f, "OrcTusk"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OverlayKind {
    Cape,
    Tunic,
}

impl Display for OverlayKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cape => write!(f, "cape"),
            Self::Tunic => write!(f, "tunic"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorIntent {
    Idle,
    Walk,
    MeleeAttack,
    RangedAttack,
    Hurt,
    Die,
}

impl Display for ActorIntent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Walk => write!(f, "Walk"),
            Self::MeleeAttack => write!(f, "MeleeAttack"),
            Self::RangedAttack => write!(f, "RangedAttack"),
            Self::Hurt => write!(f, "Hurt"),
            Self::Die => write!(f, "Die"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Facing8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Facing8 {
    pub fn as_cardinal_suffix(self) -> &'static str {
        match self {
            Self::North | Self::NorthEast | Self::NorthWest => "n",
            Self::East => "e",
            Self::South | Self::SouthEast | Self::SouthWest => "s",
            Self::West => "w",
        }
    }

    pub fn as_octant_suffix(self) -> &'static str {
        match self {
            Self::North => "n",
            Self::NorthEast => "ne",
            Self::East => "e",
            Self::SouthEast => "se",
            Self::South => "s",
            Self::SouthWest => "sw",
            Self::West => "w",
            Self::NorthWest => "nw",
        }
    }
}

impl Display for Facing8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_octant_suffix())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueuePolicy {
    Append,
    Interrupt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectIntent {
    ProjectileTravel,
    ProjectileImpact,
    BloodHit,
    Corpse,
    MeleeSwing,
}

impl Display for EffectIntent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProjectileTravel => write!(f, "ProjectileTravel"),
            Self::ProjectileImpact => write!(f, "ProjectileImpact"),
            Self::BloodHit => write!(f, "BloodHit"),
            Self::Corpse => write!(f, "Corpse"),
            Self::MeleeSwing => write!(f, "MeleeSwing"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VisualSource {
    StaticTile {
        rel_path: &'static str,
        name: &'static str,
        scale: f32,
    },
    TiledClip {
        rel_path: &'static str,
        name: &'static str,
        scale: f32,
    },
    FrameDir {
        rel_path: &'static str,
        frame_ms: u64,
        looped: bool,
        scale: f32,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct EffectBinding {
    pub intent: EffectIntent,
    pub source: VisualSource,
    #[allow(dead_code)]
    pub why: &'static str,
}

pub const MODULE_NAME: &str = "orcish-raid-animation-contract";
pub const MODULE_SOURCE: &str = "step-combat/examples/orcish_raid";
pub const ACTOR_POLICY: &str =
    "This PoC separates actor semantics, combat FX, asset binding, attachment fallback, and timing policy so we can move the same split into rifrl later.";

const HUMAN_MALE_BASE_PATHS: &[&str] = &[
    "../../../rifrl/data/animations1/human/walk/walk.tsx",
    "../../../rifrl/data/animations1/human/attack-punch/attack-punch.tsx",
    "../../../rifrl/data/animations1/human/attack-handgun/attack-handgun.tsx",
    "../../../rifrl/data/animations1/human/hurt/hurt.tsx",
    "../../../rifrl/data/animations1/human/knockback/knockback.tsx",
];

const HUMAN_MALE_CAPE_PATHS: &[&str] = &[
    "../../../rifrl/data/animations1/human/walk/variant.male.overlay/clothes/cape.tsx",
    "../../../rifrl/data/animations1/human/attack-punch/variant.male.overlay/clothes/cape.tsx",
    "../../../rifrl/data/animations1/human/attack-handgun/variant.male.overlay/clothes/cape.tsx",
];

const HUMAN_MALE_TUNIC_PATHS: &[&str] = &[
    "../../../rifrl/data/animations1/human/walk/variant.male.overlay/clothes/tunic.tsx",
    "../../../rifrl/data/animations1/human/attack-punch/variant.male.overlay/clothes/tunic.tsx",
    "../../../rifrl/data/animations1/human/attack-handgun/variant.male.overlay/clothes/tunic.tsx",
];

const ORC_JUNO_BASE_PATHS: &[&str] = &[
    "../../../rifrl/data/animations1/LPC-Juno/LPC-Juno.tsx",
];

const ORC_TUSK_BASE_PATHS: &[&str] = &[
    "../../../rifrl/data/animations1/LPC-Tusk/LPC-Tusk.tsx",
];

pub const EFFECT_BINDINGS: &[EffectBinding] = &[
    EffectBinding {
        intent: EffectIntent::ProjectileTravel,
        source: VisualSource::StaticTile {
            rel_path: "../../../rifrl/resources/DoodleRogue/doodle-48.tsx",
            name: "arrow",
            scale: 1.0,
        },
        why: "Reuse an existing wrapped projectile tile for travel along the shot line.",
    },
    EffectBinding {
        intent: EffectIntent::ProjectileImpact,
        source: VisualSource::FrameDir {
            rel_path: "../../../rifrl/resources/40FXPack_NYKNCK/Impact/I101",
            frame_ms: 45,
            looped: false,
            scale: 1.2,
        },
        why: "Demonstrate direct use of a previously unused raw frame directory.",
    },
    EffectBinding {
        intent: EffectIntent::BloodHit,
        source: VisualSource::TiledClip {
            rel_path: "../../../rifrl/resources/bloodfx001_nyknck/bloodfx001.tsx",
            name: "splatter",
            scale: 1.2,
        },
        why: "Reuse an existing wrapped blood clip for hit feedback.",
    },
    EffectBinding {
        intent: EffectIntent::Corpse,
        source: VisualSource::StaticTile {
            rel_path: "../../../rifrl/resources/DoodleRogue/doodle-48.tsx",
            name: "body",
            scale: 1.0,
        },
        why: "Persist a body marker after death without inventing a new corpse system.",
    },
    EffectBinding {
        intent: EffectIntent::MeleeSwing,
        source: VisualSource::FrameDir {
            rel_path: "../../../rifrl/resources/40FXPack_NYKNCK/Slash and Swing/S0201",
            frame_ms: 40,
            looped: false,
            scale: 1.4,
        },
        why: "Demonstrate how raw frame folders can support melee feedback without a TSX conversion pass.",
    },
];

pub const ALL_ACTOR_ARCHETYPES: &[ActorArchetype] = &[
    ActorArchetype::HumanMale,
    ActorArchetype::OrcJuno,
    ActorArchetype::OrcTusk,
];

pub const PLAYER_DEFAULT_OVERLAYS: &[OverlayKind] = &[OverlayKind::Cape];

pub fn resolve_repo_path(rel_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel_path)
}

pub fn actor_why(archetype: ActorArchetype) -> &'static str {
    match archetype {
        ActorArchetype::HumanMale => {
            "Shows human walk, melee, ranged, hurt, overlay layering, and overlay fallback."
        }
        ActorArchetype::OrcJuno => {
            "Shows that one semantic contract can map onto a different clip vocabulary, including die."
        }
        ActorArchetype::OrcTusk => {
            "Shows a second orc actor set using the same semantic contract and die support."
        }
    }
}

pub fn actor_scale(archetype: ActorArchetype) -> f32 {
    match archetype {
        ActorArchetype::HumanMale => 1.25,
        ActorArchetype::OrcJuno | ActorArchetype::OrcTusk => 1.35,
    }
}

pub fn actor_base_paths(archetype: ActorArchetype) -> &'static [&'static str] {
    match archetype {
        ActorArchetype::HumanMale => HUMAN_MALE_BASE_PATHS,
        ActorArchetype::OrcJuno => ORC_JUNO_BASE_PATHS,
        ActorArchetype::OrcTusk => ORC_TUSK_BASE_PATHS,
    }
}

pub fn supported_overlays(archetype: ActorArchetype) -> &'static [OverlayKind] {
    match archetype {
        ActorArchetype::HumanMale => &[OverlayKind::Cape, OverlayKind::Tunic],
        ActorArchetype::OrcJuno | ActorArchetype::OrcTusk => &[],
    }
}

pub fn overlay_paths(
    archetype: ActorArchetype,
    overlay: OverlayKind,
) -> Option<&'static [&'static str]> {
    match (archetype, overlay) {
        (ActorArchetype::HumanMale, OverlayKind::Cape) => Some(HUMAN_MALE_CAPE_PATHS),
        (ActorArchetype::HumanMale, OverlayKind::Tunic) => Some(HUMAN_MALE_TUNIC_PATHS),
        _ => None,
    }
}

pub fn actor_clip(
    archetype: ActorArchetype,
    intent: ActorIntent,
    facing: Facing8,
) -> Option<&'static str> {
    match archetype {
        ActorArchetype::HumanMale => match intent {
            ActorIntent::Idle | ActorIntent::Walk => Some(match facing.as_cardinal_suffix() {
                "n" => "walk-n",
                "e" => "walk-e",
                "s" => "walk-s",
                "w" => "walk-w",
                _ => unreachable!("cardinal suffix"),
            }),
            ActorIntent::MeleeAttack => Some(match facing.as_octant_suffix() {
                "n" => "attack_punch-n",
                "ne" => "attack_punch-ne",
                "e" => "attack_punch-e",
                "se" => "attack_punch-se",
                "s" => "attack_punch-s",
                "sw" => "attack_punch-sw",
                "w" => "attack_punch-w",
                "nw" => "attack_punch-nw",
                _ => unreachable!("octant suffix"),
            }),
            ActorIntent::RangedAttack => Some(match facing.as_octant_suffix() {
                "n" => "attack_handgun-n",
                "ne" => "attack_handgun-ne",
                "e" => "attack_handgun-e",
                "se" => "attack_handgun-se",
                "s" => "attack_handgun-s",
                "sw" => "attack_handgun-sw",
                "w" => "attack_handgun-w",
                "nw" => "attack_handgun-nw",
                _ => unreachable!("octant suffix"),
            }),
            ActorIntent::Hurt => Some(match facing.as_octant_suffix() {
                "n" => "hurt-n",
                "ne" => "hurt-ne",
                "e" => "hurt-e",
                "se" => "hurt-se",
                "s" => "hurt-s",
                "sw" => "hurt-sw",
                "w" => "hurt-w",
                "nw" => "hurt-nw",
                _ => unreachable!("octant suffix"),
            }),
            ActorIntent::Die => Some(match facing.as_octant_suffix() {
                "n" => "knockback-n",
                "ne" => "knockback-ne",
                "e" => "knockback-e",
                "se" => "knockback-se",
                "s" => "knockback-s",
                "sw" => "knockback-sw",
                "w" => "knockback-w",
                "nw" => "knockback-nw",
                _ => unreachable!("octant suffix"),
            }),
        },
        ActorArchetype::OrcJuno | ActorArchetype::OrcTusk => match intent {
            ActorIntent::Idle | ActorIntent::Walk => Some(match facing.as_cardinal_suffix() {
                "n" => "walk-n",
                "e" => "walk-e",
                "s" => "walk-s",
                "w" => "walk-w",
                _ => unreachable!("cardinal suffix"),
            }),
            ActorIntent::MeleeAttack => Some(match facing.as_cardinal_suffix() {
                "n" => "slash-n",
                "e" => "slash-e",
                "s" => "slash-s",
                "w" => "slash-w",
                _ => unreachable!("cardinal suffix"),
            }),
            ActorIntent::RangedAttack => Some(match facing.as_cardinal_suffix() {
                "n" => "thrust-n",
                "e" => "thrust-e",
                "s" => "thrust-s",
                "w" => "thrust-w",
                _ => unreachable!("cardinal suffix"),
            }),
            ActorIntent::Hurt => None,
            ActorIntent::Die => Some("die"),
        },
    }
}

pub fn overlay_fallback_clip(
    _archetype: ActorArchetype,
    facing: Facing8,
) -> &'static str {
    match facing.as_cardinal_suffix() {
        "n" => "walk-n",
        "e" => "walk-e",
        "s" => "walk-s",
        "w" => "walk-w",
        _ => unreachable!("cardinal suffix"),
    }
}

pub fn actor_supported_intents(archetype: ActorArchetype) -> &'static [ActorIntent] {
    match archetype {
        ActorArchetype::HumanMale => &[
            ActorIntent::Idle,
            ActorIntent::Walk,
            ActorIntent::MeleeAttack,
            ActorIntent::RangedAttack,
            ActorIntent::Hurt,
            ActorIntent::Die,
        ],
        ActorArchetype::OrcJuno | ActorArchetype::OrcTusk => &[
            ActorIntent::Idle,
            ActorIntent::Walk,
            ActorIntent::MeleeAttack,
            ActorIntent::RangedAttack,
            ActorIntent::Die,
        ],
    }
}

pub fn facing_from_map_direction(direction: MapDirection) -> Facing8 {
    match direction {
        MapDirection::North => Facing8::North,
        MapDirection::East => Facing8::East,
        MapDirection::South => Facing8::South,
        MapDirection::West => Facing8::West,
    }
}

pub fn facing_from_points(from: MapPoint, to: MapPoint) -> Facing8 {
    let dx = (to.x - from.x) as isize;
    let dy = (to.y - from.y) as isize;
    match (dx.signum(), dy.signum()) {
        (0, -1) => Facing8::North,
        (1, -1) => Facing8::NorthEast,
        (1, 0) => Facing8::East,
        (1, 1) => Facing8::SouthEast,
        (0, 1) => Facing8::South,
        (-1, 1) => Facing8::SouthWest,
        (-1, 0) => Facing8::West,
        (-1, -1) => Facing8::NorthWest,
        _ => Facing8::South,
    }
}

pub fn projectile_travel_ms(from: Tl, to: Tl) -> u64 {
    let delta: Vec2 = (to - from).into();
    (60.0 * delta.length().max(1.0)) as u64
}
