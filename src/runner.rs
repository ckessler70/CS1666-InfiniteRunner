use crate::proceduralgen;
use crate::rect;
// use crate::Physics;

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

const TILE_SIZE: u32 = 100;

const LEVEL_LEN: u32 = CAM_W * 3;
const SPEED_LIMIT: i32 = 5;

// Flipping bounds
// Roughly anything larger than 30 will not complete flip in jump's time
const FLIP_INCREMENT: f64 = 360.0 / 30.0;

// Ensure that SIZE is not a decimal
// 1, 2, 4, 5, 8, 10, 16, 20, 32, 40, 64, 80, 128, 160, 256, 320, 640
const SIZE: usize = CAM_W as usize / 8;

const FRONT_HILL_INDEX: usize = 0;
const BACK_HILL_INDEX: usize = 1;
const GROUND_INDEX: usize = 2;

pub struct Runner;

struct Player<'a> {
    pos: Rect,
    texture: Texture<'a>,
}

impl<'a> Player<'a> {
    fn new(pos: Rect, texture: Texture<'a>) -> Player {
        Player { pos, texture }
    }

    fn x(&self) -> i32 {
        self.pos.x()
    }

    fn y(&self) -> i32 {
        self.pos.y()
    }

    fn update_pos(&mut self, x_vel: i32, y_vel: i32, grounds: &[i16; SIZE]) {
        self.pos.set_x((self.pos.x() + x_vel).clamp(
            CAM_W as i32 / 2 - TILE_SIZE as i32 / 2,
            CAM_W as i32 / 2 - TILE_SIZE as i32 / 2 + 1,
        ));

        let clamp_bounds = CAM_H as i32
            - grounds[(self.pos.x() as usize) / (CAM_W / SIZE as u32) as usize] as i32
            - TILE_SIZE as i32;

        self.pos
            .set_y((self.pos.y() + y_vel).clamp(0, clamp_bounds));
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }
}

fn resist(vel: i32, deltav: i32) -> i32 {
    if deltav == 0 {
        if vel > 0 {
            -1
        } else if vel < 0 {
            1
        } else {
            deltav
        }
    } else {
        deltav
    }
}

impl Game for Runner {
    fn init() -> Result<Self, String> {
        Ok(Runner {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = core.wincan.texture_creator();
        let tex_bg = texture_creator.load_texture("assets/bg.png")?;
        let tex_sky = texture_creator.load_texture("assets/sky.png")?;
        let mut bg_buff = 0;

        // Create player at default position
        let mut player = Player::new(
            rect!(
                CAM_W as i32 / 2 - TILE_SIZE as i32 / 2,
                CAM_H as i32 / 2,
                TILE_SIZE,
                TILE_SIZE
            ),
            texture_creator.load_texture("assets/player.png")?,
        );

        // Used to keep track of animation status
        let mut frames: i32 = 0;
        let mut src_x: i32 = 0;
        let mut flip: bool = false;

        let mut x_vel: i32 = 0;
        let mut y_vel: i32 = 0;

        let mut jump: bool = false;
        let mut jump_ct: i32 = 0;

        //For rotational flip (maybe not the best variable names)
        let mut r_flip: bool = false;
        let mut r_flip_spot: f64 = 0.0;

        let mut score: i32 = 0;

        let mut game_paused: bool = false;
        let mut initial_pause: bool = false;
        let mut game_over: bool = false;
        let mut ct: i32 = 0;

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        let mut next_status = GameStatus::Main;

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

        let freq: f32 = rng.gen::<f32>() * 1000.0 + 100.0;

        let amp_1: f32 = rng.gen::<f32>() * 4.0 + 1.0;
        let amp_2: f32 = rng.gen::<f32>() * 2.0 + amp_1;
        let amp_3: f32 = rng.gen::<f32>() * 2.0 + 1.0;

        while ct < SIZE as usize {
            bg[FRONT_HILL_INDEX][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_1), freq, amp_1, 0.5, 600.0);
            bg[BACK_HILL_INDEX][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_2), freq, amp_2, 1.0, 820.0);
            bg[GROUND_INDEX][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_3), freq, amp_3, 1.5, 256.0);
            ct += 1;
        }

        'gameloop: loop {
            // FPS tracking
            last_raw_time = Instant::now();

            if game_paused {
                // Game paused handler
                for event in core.event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Q),
                            ..
                        } => {
                            next_status = GameStatus::Credits;
                            break 'gameloop;
                        }
                        Event::KeyDown {
                            keycode: Some(k), ..
                        } => match k {
                            Keycode::Escape | Keycode::Space => {
                                game_paused = false;
                            }
                            Keycode::R => {
                                next_status = GameStatus::Game;
                                break 'gameloop;
                            }
                            Keycode::M => {
                                next_status = GameStatus::Main;
                                break 'gameloop;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                // Draw it to screen once and then wait due to BlendMode
                if initial_pause {
                    let resume_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("Escape/Space - Resume Play")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let restart_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("R - Restart game")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let main_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("M - Main menu")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let quit_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("Q - Quit game")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    // Grey out screen
                    core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 128));
                    core.wincan.fill_rect(rect!(0, 0, CAM_W, CAM_H))?;

                    // Draw text
                    core.wincan
                        .copy(&resume_texture, None, Some(rect!(100, 100, 1000, 125)))?;
                    core.wincan
                        .copy(&restart_texture, None, Some(rect!(100, 250, 700, 125)))?;
                    core.wincan
                        .copy(&main_texture, None, Some(rect!(100, 400, 600, 125)))?;
                    core.wincan
                        .copy(&quit_texture, None, Some(rect!(100, 550, 600, 125)))?;

                    core.wincan.present();

                    initial_pause = false;
                }
            } else if game_over {
                if initial_pause {
                    ct = 0;
                    let game_over_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("GAME OVER")
                                .blended(Color::RGBA(255, 0, 0, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let TextureQuery { width, height, .. } = game_over_texture.query();

                    let padding = 64;

                    let wr = width as f32 / (CAM_W - padding) as f32;
                    let hr = height as f32 / (CAM_H - padding) as f32;

                    let (w, h) = if wr > 1f32 || hr > 1f32 {
                        if wr > hr {
                            let h = (height as f32 / wr) as i32;
                            ((CAM_W - padding) as i32, h)
                        } else {
                            let w = (width as f32 / hr) as i32;
                            (w, (CAM_H - padding) as i32)
                        }
                    } else {
                        (width as i32, height as i32)
                    };

                    let cx = (CAM_W as i32 - w) / 2;
                    let cy = (CAM_H as i32 - h) / 2;

                    core.wincan
                        .copy(&game_over_texture, None, Some(rect!(cx, cy, w, h)))?;

                    core.wincan.present();

                    initial_pause = false;
                }

                ct += 1;
                if ct == 120 {
                    break 'gameloop;
                }
            } else {
                let mut x_deltav: i32 = 1;
                let mut y_deltav: i32 = 1;
                for event in core.event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'gameloop,
                        Event::KeyDown {
                            keycode: Some(k), ..
                        } => match k {
                            Keycode::W | Keycode::Up | Keycode::Space => {
                                if !jump && jump_ct == 0 {
                                    jump = true;
                                }
                                if jump_ct != 0 {
                                    r_flip = true;
                                }
                            }
                            Keycode::Escape => {
                                game_paused = true;
                                initial_pause = true;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                // Boing
                if jump {
                    jump_ct += 1;
                    y_deltav = -1;
                }

                // Airtime
                if jump_ct > 30 {
                    jump = false;
                    y_deltav = 1;
                }

                // Jump cooldown
                if !jump && jump_ct > 0 {
                    jump_ct -= 1;
                }

                // Landed on head, GAME OVER
                if jump_ct == 0 && r_flip_spot != 0.0 {
                    game_over = true;
                    initial_pause = true;
                    continue;
                }

                core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
                core.wincan.clear();

                // Every tick, build a new ground segment
                if tick % 1 == 0 {
                    for i in 0..(SIZE as usize - 1) {
                        bg[GROUND_INDEX][i] = bg[GROUND_INDEX][i + 1];
                    }
                    buff_3 += 1;
                    let chunk_3 = proceduralgen::gen_perlin_hill_point(
                        ((SIZE - 1) as usize + buff_3),
                        freq,
                        amp_3,
                        1.5,
                        256.0,
                    );
                    bg[GROUND_INDEX][(SIZE - 1) as usize] = chunk_3;
                }

                // Every 3 ticks, build a new front mountain segment
                if tick % 3 == 0 {
                    for i in 0..(SIZE as usize - 1) {
                        bg[FRONT_HILL_INDEX][i] = bg[FRONT_HILL_INDEX][i + 1];
                    }
                    buff_1 += 1;
                    let chunk_1 = proceduralgen::gen_perlin_hill_point(
                        ((SIZE - 1) as usize + buff_1),
                        freq,
                        amp_1,
                        0.5,
                        600.0,
                    );
                    bg[FRONT_HILL_INDEX][(SIZE - 1) as usize] = chunk_1;
                }

                // Every 5 ticks, build a new back mountain segment
                if tick % 5 == 0 {
                    for i in 0..(SIZE as usize - 1) {
                        bg[BACK_HILL_INDEX][i] = bg[BACK_HILL_INDEX][i + 1];
                    }
                    buff_2 += 1;
                    let chunk_2 = proceduralgen::gen_perlin_hill_point(
                        ((SIZE - 1) as usize + buff_2),
                        freq,
                        amp_2,
                        1.0,
                        820.0,
                    );
                    bg[BACK_HILL_INDEX][(SIZE - 1) as usize] = chunk_2;
                }
                if tick % 10 == 0 {
                    bg_buff -= 1;
                }

                //Background gradient

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

                for i in 0..bg[FRONT_HILL_INDEX].len() - 1 {
                    // Furthest back mountains
                    core.wincan.set_draw_color(Color::RGBA(128, 51, 6, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                        CAM_H as i16 - bg[BACK_HILL_INDEX][i],
                        CAM_W as usize / SIZE,
                        CAM_H as i16
                    ))?;

                    // Closest mountains
                    core.wincan.set_draw_color(Color::RGBA(96, 161, 152, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                        CAM_H as i16 - bg[FRONT_HILL_INDEX][i],
                        CAM_W as usize / SIZE,
                        CAM_H as i16
                    ))?;

                    // Ground
                    core.wincan.set_draw_color(Color::RGBA(13, 66, 31, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                        CAM_H as i16 - bg[GROUND_INDEX][i],
                        CAM_W as usize / SIZE,
                        CAM_H as i16
                    ))?;
                }

                tick += 1;

                score += 1;

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

                x_deltav = resist(x_vel, x_deltav);
                y_deltav = resist(y_vel, y_deltav);
                x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
                y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

                player.update_pos(x_vel, y_vel, &bg[2]);

                //ADDITION: Hey lizard, do a flip
                r_flip_spot = if r_flip && flip {
                    //going left
                    r_flip_spot + FLIP_INCREMENT
                } else if r_flip && !flip {
                    //going right
                    r_flip_spot - FLIP_INCREMENT
                } else {
                    0.0
                };

                //going right backlfip
                //if r_flip_spot.approx_eq(-360.0, (0.0, 2)) {
                if r_flip_spot.floor() == -360.0 {
                    //flip complete
                    r_flip = false;
                    r_flip_spot = 0.0; //reset flip_spot
                }
                //Going left backflip
                //if r_flip_spot.approx_eq(360.0, (0.0, 2)) {

                if r_flip_spot.floor() == 360.0 {
                    //flip complete
                    r_flip = false;
                    r_flip_spot = 0.0; //reset flip_spot
                }

                // Draw player
                //NOTE: i added 10 toplayer. y()
                core.wincan.copy_ex(
                    player.texture(),
                    rect!(src_x, 0, TILE_SIZE, TILE_SIZE),
                    rect!(player.x(), player.y() + 10, TILE_SIZE, TILE_SIZE),
                    r_flip_spot,
                    None,
                    flip,
                    false,
                )?;

                let surface = font
                    .render(&format!("{:08}", score))
                    .blended(Color::RGBA(255, 0, 0, 100))
                    .map_err(|e| e.to_string())?;
                let score_texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                core.wincan
                    .copy(&score_texture, None, Some(rect!(10, 10, 100, 50)))?;

                core.wincan.present();
            }

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
            status: Some(next_status),
            score: score,
        })
    }
}
