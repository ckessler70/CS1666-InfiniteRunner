use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

const FPS: f64 = 60.0;
const FRAME_TIME: f64 = 1.0 / FPS as f64;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct Instruction;

impl Game for Instruction {
    fn init() -> Result<Self, String> {
        Ok(Instruction {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = core.wincan.texture_creator();

        core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
        core.wincan.clear();

        let tex_control = texture_creator.load_texture("assets/instructions/controls.png")?;
        let tex_collect = texture_creator.load_texture("assets/instructions/collectables.png")?;
        let tex_interact = texture_creator.load_texture("assets/instructions/interactable.png")?;
        let tex_over = texture_creator.load_texture("assets/instructions/over.png")?;

        core.wincan.present();

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        let mut next_status = GameStatus::Main;

        let mut timer = 0;
        let mut which = 0;

        'gameloop: loop {
            last_raw_time = Instant::now(); // FPS tracking
            for event in core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        next_status = GameStatus::Credits;
                        break 'gameloop;
                    }
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => match k {
                        Keycode::Escape | Keycode::Q => {
                            next_status = GameStatus::Main;
                            break 'gameloop;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            if timer > 180 {
                timer = 0;
                which += 1;
            }

            match (which) {
                0 => {
                    core.wincan.copy_ex(
                        &tex_control,
                        rect!(0, 0, CAM_W, CAM_H),
                        rect!(0, 0, CAM_W, CAM_H),
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                }
                1 => {
                    core.wincan.copy_ex(
                        &tex_collect,
                        rect!(0, 0, CAM_W, CAM_H),
                        rect!(0, 0, CAM_W, CAM_H),
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                }
                2 => {
                    core.wincan.copy_ex(
                        &tex_interact,
                        rect!(0, 0, CAM_W, CAM_H),
                        rect!(0, 0, CAM_W, CAM_H),
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                }
                3 => {
                    core.wincan.copy_ex(
                        &tex_over,
                        rect!(0, 0, CAM_W, CAM_H),
                        rect!(0, 0, CAM_W, CAM_H),
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                }
                _ => {
                    next_status = GameStatus::Main;
                    break 'gameloop;
                }
            }

            timer += 1;

            core.wincan.present();

            /* ~~~~~~ FPS Calculation ~~~~~~ */
            // Time taken to display the last frame
            let raw_frame_time = last_raw_time.elapsed().as_secs_f64();
            let delay = FRAME_TIME - raw_frame_time;
            // If the amount of time to display the last frame was less than expected, sleep
            // until the expected amount of time has passed
            if delay > 0.0 {
                // Using sleep to delay will always cause slightly more delay than intended due
                // to CPU scheduling; possibly find a better way to delay
                sleep(Duration::from_secs_f64(delay));
            }
            all_frames += 1;
            let time_since_last_measurement = last_measurement_time.elapsed();
            // Measures the FPS once per second
            if time_since_last_measurement > Duration::from_secs(1) {
                //println!("{} FPS", all_frames);
                // println!(
                //     "Average FPS: {:.2}",
                //     (all_frames as f64) / time_since_last_measurement.as_secs_f64()
                // );
                all_frames = 0;
                last_measurement_time = Instant::now();
            }
            /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
        }

        // Out of game loop, return Ok
        Ok(GameState {
            status: Some(next_status),
            score: 0,
        })
    }
}
