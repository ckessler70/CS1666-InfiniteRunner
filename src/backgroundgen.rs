use crate::proceduralgen;
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
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

use rand::Rng;

const FPS: f64 = 60.0;
const FRAME_TIME: f64 = 1.0 / FPS as f64;

const CAM_H: u32 = 720;
const CAM_W: u32 = 1280;

// Ensure that SIZE is not a decimal
// 2, 4, 5, 8, 10, 16, 20
const SIZE: usize = CAM_W as usize / 8;

pub struct BackgroundGen;

impl Game for BackgroundGen {
    fn init() -> Result<Self, String> {
        Ok(BackgroundGen {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = core.wincan.texture_creator();
        let tex_bg = texture_creator.load_texture("assets/bg.png")?;
        let tex_sky = texture_creator.load_texture("assets/sky.png")?;
        let mut bg_buff = 0;

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        let mut next_status = Some(GameStatus::Main);

        let mut ct: usize = 0;
        let mut tick = 0;
        let mut buff_1: usize = 0;
        let mut buff_2: usize = 0;
        let mut buff_3: usize = 0;

        // bg[0] = Front hills
        // bg[1] = Back hills
        // bg[2] = Ground
        let mut bg: [[i16; SIZE]; 3] = [[0; SIZE]; 3];

        let mut rng = rand::thread_rng();

        let freq: f64 = rng.gen::<f64>() * 1000.0 + 100.0;

        let amp_1: f64 = rng.gen::<f64>() * 4.0 + 1.0;
        let amp_2: f64 = rng.gen::<f64>() * 2.0 + amp_1;
        let amp_3: f64 = rng.gen::<f64>() * 2.0 + 1.0;

        while ct < SIZE as usize {
            bg[0][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_1), freq, amp_1, 0.5, 600.0);
            bg[1][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_2), freq, amp_2, 1.0, 820.0);
            bg[2][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_3), freq, amp_3, 1.5, 256.0);
            ct += 1;
        }

        'gameloop: loop {
            // FPS tracking
            last_raw_time = Instant::now();

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
                        Keycode::T => {
                            next_status = Some(GameStatus::Test);
                            break 'gameloop;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
            core.wincan.clear();

            // Every tick, build a new ground segment
            if tick % 1 == 0 {
                for i in 0..(SIZE as usize - 1) {
                    bg[2][i] = bg[2][i + 1];
                }
                buff_3 += 1;
                let chunk_3 = proceduralgen::gen_perlin_hill_point(
                    ((SIZE - 1) as usize + buff_3),
                    freq,
                    amp_3,
                    1.5,
                    256.0,
                );
                bg[2][(SIZE - 1) as usize] = chunk_3;
            }

            // Every 3 ticks, build a new front mountain segment
            if tick % 3 == 0 {
                for i in 0..(SIZE as usize - 1) {
                    bg[0][i] = bg[0][i + 1];
                }
                buff_1 += 1;
                let chunk_1 = proceduralgen::gen_perlin_hill_point(
                    ((SIZE - 1) as usize + buff_1),
                    freq,
                    amp_1,
                    0.5,
                    600.0,
                );
                bg[0][(SIZE - 1) as usize] = chunk_1;
            }

            // Every 5 ticks, build a new back mountain segment
            if tick % 5 == 0 {
                for i in 0..(SIZE as usize - 1) {
                    bg[1][i] = bg[1][i + 1];
                }
                buff_2 += 1;
                let chunk_2 = proceduralgen::gen_perlin_hill_point(
                    ((SIZE - 1) as usize + buff_2),
                    freq,
                    amp_2,
                    1.0,
                    820.0,
                );
                bg[1][(SIZE - 1) as usize] = chunk_2;
            }
            if tick % 10 == 0 {
                bg_buff -= 1;
            }

            //Background gradient
            for i in 0..CAM_H {
                core.wincan.set_draw_color(Color::RGBA(
                    255,
                    69,
                    0,
                    ((i as f64 / CAM_H as f64 / 40.0) * 255.0) as u8,
                ));
                core.wincan.fill_rect(rect!(0, i, CAM_W, CAM_H));
            }

            core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
            core.wincan.fill_rect(rect!(0, 470, CAM_W, CAM_H));

            // Draw background
            core.wincan
                .copy(&tex_bg, None, rect!(bg_buff, -150, CAM_W, CAM_H))?;
            core.wincan.copy(
                &tex_bg,
                None,
                rect!(bg_buff + (CAM_W as i32), -150, CAM_W, CAM_H),
            )?;

            //Draw sky in background
            core.wincan
                .copy(&tex_sky, None, rect!(bg_buff, 0, CAM_W, CAM_H / 3))?;
            core.wincan.copy(
                &tex_sky,
                None,
                rect!(CAM_W as i32 + bg_buff, 0, CAM_W, CAM_H / 3),
            )?;

            for i in 0..bg[0].len() - 1 {
                // Furthest back mountains
                core.wincan.set_draw_color(Color::RGBA(
                    (0.2 * 255.0) as u8,
                    (0.2 * 255.0) as u8,
                    0,
                    255,
                ));
                core.wincan.fill_rect(rect!(
                    i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                    CAM_H as i16 - bg[1][i],
                    CAM_W as usize / SIZE,
                    CAM_H as i16
                ));

                // Closest mountains
                core.wincan.set_draw_color(Color::RGBA(12, 102, 133, 255));
                core.wincan.fill_rect(rect!(
                    i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                    CAM_H as i16 - bg[0][i],
                    CAM_W as usize / SIZE,
                    CAM_H as i16
                ));

                // Ground
                core.wincan.set_draw_color(Color::RGBA(13, 66, 31, 255));
                core.wincan.fill_rect(rect!(
                    i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                    CAM_H as i16 - bg[2][i],
                    CAM_W as usize / SIZE,
                    CAM_H as i16
                ));
            }

            tick += 1;

            if tick % 3 == 0 && tick % 5 == 0 {
                tick = 0;
            }

            if -bg_buff == CAM_W as i32 {
                bg_buff = 0;
            }

            // There should be some way to determine where it repeats but I can't figure it out
            // if buff_1 as f64 % (freq * amp_1) == 0 {
            //     println!("{:?}", buff_1);
            //     buff_1 = 0;
            // }
            // if buff_2 as f64 % (freq * amp_2) == 0 {
            //     println!("{:?}", buff_2);
            //     buff_2 = 0;
            // }
            // if buff_3 as f64 % (freq * amp_3) == 0 {
            //     println!("{:?}", buff_3);
            //     buff_3 = 0;
            // }

            core.wincan.present();

            // FPS Calculation
            // the time taken to display the last frame
            let raw_frame_time = last_raw_time.elapsed().as_secs_f64();
            let delay = FRAME_TIME - raw_frame_time;
            // if the amount of time to display the last frame was less than expected, sleep
            // until the expected amount of time has passed
            if delay > 0.0 {
                // using sleep to delay will always cause slightly more delay than intended due
                // to CPU scheduling; possibly find a better way to delay
                sleep(Duration::from_secs_f64(delay));
            }
            // let adjusted_frame_time = last_adjusted_time.elapsed().as_secs_f64();
            all_frames += 1;
            let time_since_last_measurement = last_measurement_time.elapsed();
            // measure the FPS once every second
            if time_since_last_measurement > Duration::from_secs(1) {
                all_frames = 0;
                last_measurement_time = Instant::now();
            }
        }

        Ok(GameState {
            status: next_status,
            score: 0,
        })
    }
}
