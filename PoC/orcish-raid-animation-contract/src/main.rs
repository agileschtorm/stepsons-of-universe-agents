use macroquad::color::BLACK;

use frame_counter::{FrameCounter, StdTimer};

mod state;
mod map_gen;
mod animation;
mod actors;
mod contract;
mod demo;
mod fx;
mod runtime;

use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::prelude::draw_text;
use macroquad::window::{Conf, next_frame};
use crate::demo::{draw_play_hint, DemoController, DemoFlow};
use crate::state::{State, TILE_SIZE};
use crate::map_gen::{WIDTH, HEIGHT};

enum GameMode {
    Demo(DemoController),
    Interactive,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Orcish Raid Animation Contract PoC".to_owned(),
        // fullscreen: false,
        window_width: (WIDTH * TILE_SIZE) as i32,
        window_height: (HEIGHT * TILE_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!(
        "{} extracted from {}",
        contract::MODULE_NAME,
        contract::MODULE_SOURCE
    );
    println!("{}", contract::ACTOR_POLICY);
    println!("Demo starts automatically.");
    println!("Controls: arrows/WASD move, F shoot, Tab cycle targets, 1/2 toggle cape/tunic, Q quit.");
    println!("Extra controls: any key starts play mode, R restarts the scene, F1 returns to demo mode.");

    let mut gs: State = State::demo_scene().await;
    let mut mode = GameMode::Demo(DemoController::new());

    let mut frame_counter: FrameCounter<StdTimer> = FrameCounter::default();

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        frame_counter.tick();

        match &mut mode {
            GameMode::Demo(controller) => {
                if controller.should_take_over() {
                    gs = State::demo_scene().await;
                    mode = GameMode::Interactive;
                } else if matches!(controller.update(&mut gs), DemoFlow::Restart) {
                    gs = State::demo_scene().await;
                    mode = GameMode::Demo(DemoController::new());
                }
            }
            GameMode::Interactive => {
                if is_key_pressed(KeyCode::R) {
                    gs = State::demo_scene().await;
                } else if is_key_pressed(KeyCode::F1) {
                    gs = State::demo_scene().await;
                    mode = GameMode::Demo(DemoController::new());
                } else {
                    gs.input();
                }
            }
        }

        gs.gc();
        gs.draw();

        match &mode {
            GameMode::Demo(controller) => controller.draw_overlay(&gs),
            GameMode::Interactive => draw_play_hint(),
        }

        draw_text(
            &format!("fps: {}", frame_counter),
            100.0,
            24.0,
            24.0,
            BLACK,
        );

        frame_counter.sleep_until_framerate(40.0);

        next_frame().await
    };
}
