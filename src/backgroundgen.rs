use crate::rect;
// use crate::Physics;
use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::event::Event;
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

pub struct BackgroundGen;

impl Game for BackgroundGen {
    fn init() -> Result<Self, String> {
        Ok(BackgroundGen {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        let mut next_status = Some(GameStatus::Main);

        let mut ct = 0;
        let mut buff_1: usize = 0;
        let mut buff_2: usize = 0;

        let mut bg: [[f64; 720]; 160] = [[0.0; 720]; 160];

        let mut rng = rand::thread_rng();

        let freq_1: f64 = rng.gen::<f64>() * 150.0 + 64.0;
        let freq_2: f64 = rng.gen::<f64>() * 200.0 + 64.0;

        let amp_1: f64 = rng.gen::<f64>() + 2.0;
        let amp_2: f64 = rng.gen::<f64>() + amp_1;

        while ct < 80 {
            bg[ct] = main_image((ct + buff_1), freq_1, amp_1, 0.5);
            bg[ct + 80] = main_image((ct + buff_2), freq_2, amp_2, 1.0);
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

            if ct >= 80 {
                for i in 0..79 {
                    bg[i] = bg[i + 1];
                }

                if buff_1 % 3 == 1 {
                    for i in 0..79 {
                        bg[i + 80] = bg[i + 1 + 80];
                    }
                    buff_2 += 1;
                }
                buff_1 += 1;
                ct = 79
            }

            let chunk_1 = main_image((ct + buff_1), freq_1, amp_1, 0.5);
            bg[ct] = chunk_1;

            if buff_2 % 3 == 1 {
                let chunk_2 = main_image((ct + buff_2), freq_2, amp_2, 1.0);
                bg[ct + 80] = chunk_2;
            }

            for i in 0..ct {
                for j in 0..720 {
                    if bg[i][j] == 1.0 && bg[i + 80][j] == 1.0 {
                        core.wincan.set_draw_color(Color::RGBA(
                            255,
                            69,
                            0,
                            (((720.0 - j as f64) / 720.0) * 255.0) as u8,
                        ));
                    }
                    if bg[i + 80][j] == 0.01 {
                        core.wincan.set_draw_color(Color::RGBA(
                            (0.2 * 255.0) as u8,
                            (0.2 * 255.0) as u8,
                            0,
                            255,
                        ));
                    }
                    if bg[i][j] == 0.01 {
                        core.wincan.set_draw_color(Color::RGBA(
                            (bg[i][j] * 255.0).floor() as u8,
                            (bg[i][j] * 255.0).floor() as u8,
                            (bg[i][j] * 255.0).floor() as u8,
                            255,
                        ));
                    }

                    // if bg_1[i][j] == 1.0 {
                    // } else if bg_1[i][j] == 0.2 {
                    // } else if bg_1[i][j] == 0.01 {
                    // } else {
                    // core.wincan.set_draw_color(Color::RGBA(
                    //     255,
                    //     69,
                    //     0,
                    //     ((j / 720) * 255) as u8,
                    // ));
                    // }

                    core.wincan
                        .fill_rect(rect!((16 * i), (720 - j - 1), 16, 1))?;
                }
            }

            ct += 1;

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

fn main_image(i: usize, freq: f64, amp: f64, modifier: f64) -> [f64; 720] {
    let mut out = [0.0; 720];

    for j in 0..720 {
        let cord = (i, j);

        let n = modifier
            * (noise(cord.0 as f64 * (1.0 / freq)) * amp
                + noise(cord.0 as f64 * (1.0 / freq / 2.0)) * amp / 2.0
                + noise(cord.0 as f64 * (1.0 / freq / 4.0)) * amp / 4.0
                + noise(cord.0 as f64 * (1.0 / freq / 8.0)) * amp / 8.0);

        let y = 2.0 * (cord.1 as f64 / 256.0) - 1.0;
        out[j] = 1.0;
        if n > y {
            out[j] = 0.01;
        }
    }
    return out;
}

fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

fn grad(p: f64) -> f64 {
    let random = [0.0; 256];
    let v = random[p.floor() as usize];

    return if v > 0.5 { 1.0 } else { -1.0 };
}

fn noise(p: f64) -> f64 {
    let p0 = p.floor();
    let p1 = p0 + 1.0;

    let t = p - p0;
    let fade_t = fade(t);

    let g0 = grad(p0);
    let g1 = grad(p1);

    return ((1.0 - fade_t) * g0 * (p - p0) + fade_t * g1 * (p - p1));
}
