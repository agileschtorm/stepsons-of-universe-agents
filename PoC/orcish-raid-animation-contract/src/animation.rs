#![allow(dead_code)]

use std::time::{Duration, Instant};
use macroquad::color::Color;
use macroquad::math::Vec2;

pub trait WeightedAverage: Copy {
    /// * weight: between 0 and 1.
    fn between(self, other: Self, fraction: f32) -> Self;
}

impl WeightedAverage for f32 {
    fn between(self, other: Self, fraction: f32) -> Self {
        self * (1.0 - fraction) + other * fraction
    }
}

impl WeightedAverage for Vec2 {
    fn between(self, other: Self, fraction: f32) -> Self {
        self * (1.0 - fraction) + other * fraction
    }
}

impl WeightedAverage for Color {
    fn between(self, other: Self, fraction: f32) -> Self {
        Color::new(
            self.r * (1.0 - fraction) + other.r * fraction,
            self.g * (1.0 - fraction) + other.g * fraction,
            self.b * (1.0 - fraction) + other.b * fraction,
            self.a * (1.0 - fraction) + other.a * fraction,
        )
    }
}

pub trait TimedTransition<TResult=Self> {
    fn get_at(self, fraction: f32) -> TResult;
    fn last(self) -> TResult;
}

#[derive(Copy, Clone)]
pub enum Either<T: Copy> {
    Static(T),
    Transition(T, T),
}

impl<T: Copy + WeightedAverage> TimedTransition<T> for Either<T> {
    fn get_at(self, fraction: f32) -> T {
        match self {
            Either::Static(val) => val,
            Either::Transition(a, b) => a.between(b, fraction)
        }
    }

    fn last(self) -> T {
        match self {
            Either::Static(val) => val,
            Either::Transition(_, b) => b,
        }
    }
}


#[derive(Copy, Clone)]
pub struct TransitionInstant {
    pub scale: f32,
    pub color: Color,
    pub rotation: f32,
    pub position: Vec2,
}

impl WeightedAverage for TransitionInstant {
    fn between(self, other: Self, fraction: f32) -> Self {
        Self {
            scale: self.scale.between(other.scale, fraction),
            color: self.color.between(other.color, fraction),
            rotation: self.rotation.between(other.rotation, fraction),
            position: self.position.between(other.position, fraction),
        }
    }
}

#[derive(Copy, Clone)]
pub struct AsciiTransition {
    pub duration: Duration,
    pub ab: Either<TransitionInstant>,
}

impl TimedTransition<TransitionInstant> for AsciiTransition {
    fn get_at(self, fraction: f32) -> TransitionInstant {
        self.ab.get_at(fraction)
    }

    fn last(self) -> TransitionInstant {
        self.ab.last()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Loop {
    None,
    Simple,
    BackForth,
}

#[derive(Copy, Clone)]
pub struct AsciiAnimation {
    pub start: Instant,
    pub looped: Loop,
    pub glyph: char,
    pub transition: AsciiTransition,
}

impl AsciiAnimation {
    #[allow(dead_code)]
    pub fn new(start: Instant, glyph: char, transition: AsciiTransition) -> Self {
        Self { start, looped: Loop::None, glyph, transition, }
    }

    #[allow(dead_code)]
    pub fn new_looped(start: Instant, glyph: char, transition: AsciiTransition, looped: Loop) -> Self {
        Self { start, looped, glyph, transition, }
    }

    pub fn with_now(&self) -> Self {
        let mut r = self.clone();
        r.start = Instant::now();
        r
    }

    pub fn with_pos(&self, pos: Vec2) -> Self {
        let mut r = self.clone();
        match &mut r.transition.ab {
            Either::Static(ti) => ti.position = pos,
            Either::Transition(a, b) => {
                a.position = pos;
                b.position = pos;
            }
        }
        r
    }

    pub fn is_ended(&self, when: Instant) -> bool {
        when - self.start > self.transition.duration
    }

    pub fn get_at(&self, when: Instant) -> Option<TransitionInstant> {
        let mut t = (when - self.start).as_secs_f32() / self.transition.duration.as_secs_f32();

        match self.looped {
            Loop::None => {}
            Loop::Simple => {
                if t > 1.0 {
                    t -= t.floor();
                }
            }
            Loop::BackForth => {
                if t > 2.0 {
                    t -= (t / 2.0).floor() * 2.0;
                }
                if t > 1.0 {
                    t = 2.0 - t;
                }
            }
        }

        if t < 0.0 || t > 1.0 {
            return None;
        }

        Some(self.transition.ab.get_at(t))
    }

    pub fn last(&self) -> TransitionInstant {
        self.transition.ab.last()
    }
}
