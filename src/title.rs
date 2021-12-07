use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const FPS: f64 = 60.0;
const FRAME_TIME: f64 = 1.0 / FPS as f64;

pub struct Title;

impl Game for Title {
    fn init() -> Result<Self, String> {
        Ok(Title {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = core.wincan.texture_creator();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let mut player_angle = 0.0;

        let surface = font
            .render("Urban Odyssey")
            .blended(Color::RGBA(230, 150, 25, 255))
            .map_err(|e| e.to_string())?;
        let title_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("P/Space - Play")
            .blended(Color::RGBA(230, 150, 25, 255))
            .map_err(|e| e.to_string())?;
        let play_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("I - Instructions")
            .blended(Color::RGBA(230, 150, 25, 255))
            .map_err(|e| e.to_string())?;
        let instruction_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("C - Credits")
            .blended(Color::RGBA(230, 150, 25, 255))
            .map_err(|e| e.to_string())?;
        let credits_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Escape/Q - Quit game")
            .blended(Color::RGBA(230, 150, 25, 255))
            .map_err(|e| e.to_string())?;
        let quit_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let tex_player = texture_creator.load_texture("assets/player/player.png")?;

        let next_status;

        // FPS tracking
        let mut _all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        'gameloop: loop {
            last_raw_time = Instant::now(); // FPS tracking
            core.wincan.set_draw_color(Color::RGBA(119, 120, 123, 255));
            core.wincan.clear();

            // Draw text
            core.wincan
                .copy(&title_texture, None, Some(rect!(30, 40, 850, 200)))?;
            core.wincan
                .copy(&play_texture, None, Some(rect!(100, 230, 350, 100)))?;
            core.wincan
                .copy(&instruction_texture, None, Some(rect!(100, 340, 420, 100)))?;
            core.wincan
                .copy(&credits_texture, None, Some(rect!(100, 450, 300, 100)))?;
            core.wincan
                .copy(&quit_texture, None, Some(rect!(100, 560, 550, 100)))?;
            core.wincan.copy_ex(
                &tex_player,
                rect!(0, 0, 250, 250),
                rect!(900, 300, 250, 250),
                -player_angle,
                None,
                false,
                false,
            )?;

            core.wincan.present();
            player_angle = (player_angle + 5.0) % 360.0;

            for event in core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape | Keycode::Q),
                        ..
                    } => {
                        next_status = None;
                        break 'gameloop;
                    }
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => match k {
                        Keycode::P | Keycode::Space => {
                            next_status = Some(GameStatus::Game);
                            break 'gameloop;
                        }
                        Keycode::C => {
                            next_status = Some(GameStatus::Credits);
                            break 'gameloop;
                        }
                        Keycode::I => {
                            next_status = Some(GameStatus::Instruct);
                            break 'gameloop;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

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
            _all_frames += 1;
            let time_since_last_measurement = last_measurement_time.elapsed();
            // Measures the FPS once per second
            if time_since_last_measurement > Duration::from_secs(1) {
                //println!("{} FPS", _all_frames);
                // println!(
                //     "Average FPS: {:.2}",
                //     (_all_frames as f64) / time_since_last_measurement.as_secs_f64()
                // );
                _all_frames = 0;
                last_measurement_time = Instant::now();
            }
            /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
        }

        // Out of game loop, return Ok
        Ok(GameState {
            status: next_status,
            score: 0,
        })
    }
}
