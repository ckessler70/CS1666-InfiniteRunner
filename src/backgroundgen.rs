use crate::rect;
// use crate::Physics;
use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

use rand::Rng;

const CAM_H: u32 = 720;
const CAM_W: u32 = 1280;

pub struct BackgroundGen;

impl Game for BackgroundGen {
    fn init() -> Result<Self, String> {
        Ok(BackgroundGen {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = core.wincan.texture_creator();

        core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
        core.wincan.clear();

        let mut next_status = Some(GameStatus::Main);

        let mut ct = 0;
        let mut i = 0;
        while ct < 1 {
            let chunk = main_image();
            while i < 160 {
                for j in 0..720 {
                    if chunk[i][j] == 1.0 {
                        core.wincan.set_draw_color(Color::RGBA(
                            255,
                            69,
                            0,
                            (((720.0 - j as f64) / 720.0) * 255.0) as u8,
                        ));
                    } else if chunk[i][j] == 0.2 {
                        core.wincan.set_draw_color(Color::RGBA(
                            (chunk[i][j] * 255.0).floor() as u8,
                            (chunk[i][j] * 255.0).floor() as u8,
                            0,
                            255,
                        ));
                    } else if chunk[i][j] == 0.01 {
                        core.wincan.set_draw_color(Color::RGBA(
                            (chunk[i][j] * 255.0).floor() as u8,
                            (chunk[i][j] * 255.0).floor() as u8,
                            (chunk[i][j] * 255.0).floor() as u8,
                            255,
                        ));
                    } else {
                        // core.wincan.set_draw_color(Color::RGBA(
                        //     255,
                        //     69,
                        //     0,
                        //     ((j / 720) * 255) as u8,
                        // ));
                    }

                    core.wincan
                        .fill_rect(rect!((8 * i) + 160 * ct, (720 - j), 8, 1))?;
                }
                i += 1;
            }
            i = 0;
            ct += 1;
        }

        core.wincan.present();

        'gameloop: loop {
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
        }

        Ok(GameState {
            status: next_status,
            score: 0,
        })
    }
}

fn main_image() -> [[f64; 720]; 160] {
    let mut out = [[0.0; 720]; 160];

    let mut rng = rand::thread_rng();

    let freq_n1: f64 = rng.gen::<f64>() * 300.0 + 32.0;
    let amp_n1: f64 = rng.gen::<f64>() + 0.5;

    let freq_n2: f64 = rng.gen::<f64>() * 300.0 + 32.0;
    let amp_n2: f64 = rng.gen::<f64>() + 0.5;

    for i in 0..160 {
        for j in 0..720 {
            let cord = (i, j);

            let n1 = 0.5
                * (noise(cord.0 as f64 * (1.0 / freq_n1)) * amp_n1
                    + noise(cord.0 as f64 * (1.0 / freq_n1 / 2.0)) * amp_n1 / 2.0
                    + noise(cord.0 as f64 * (1.0 / freq_n1 / 4.0)) * amp_n1 / 4.0
                    + noise(cord.0 as f64 * (1.0 / freq_n1 / 8.0)) * amp_n1 / 8.0);

            let n2 = noise(cord.0 as f64 * (1.0 / freq_n2)) * amp_n2
                + noise(cord.0 as f64 * (1.0 / freq_n2 / 2.0)) * amp_n2 / 2.0
                + noise(cord.0 as f64 * (1.0 / freq_n2 / 4.0)) * amp_n2 / 4.0
                + noise(cord.0 as f64 * (1.0 / freq_n2 / 8.0)) * amp_n2 / 8.0;
            let y = 2.0 * (cord.1 as f64 / 720.0) - 1.0;
            out[i][j] = 1.0;
            if n2 > y {
                out[i][j] = 0.2;
            }
            if n1 > y {
                out[i][j] = 0.01;
            }
        }
    }
    for i in 0..720 {
        for j in 0..160 {
            let print = if out[j][i] == 1.0 {
                '_'
            } else if out[j][i] == 0.2 {
                '.'
            } else if out[j][i] == 0.01 {
                '+'
            } else {
                ' '
            };
            print!("{}", print);
        }
        println!("");
    }
    return out;
}

fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

fn grad(p: f64) -> f64 {
    let mut random = [0.0; 256];
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
