// use crate::physics::Player;
use crate::proceduralgen::ProceduralGen;
use crate::proceduralgen::TerrainSegment;
use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use std::collections::HashSet;
use std::collections::LinkedList;
use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

const FPS: f64 = 60.0;
const FRAME_TIME: f64 = 1.0 / FPS as f64;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const TILE_SIZE: u32 = 100;

// Bounds we want to keep the player within
const PLAYER_BOUNDS_H: (i32, i32) = (0, (CAM_W - TILE_SIZE) as i32);
const PLAYER_BOUNDS_V: (i32, i32) = (0, (CAM_H - TILE_SIZE) as i32);
//const LTHIRD: i32 = ((CAM_W as i32) / 3) - (TILE_SIZE as i32) / 2;
//const RTHIRD: i32 = ((CAM_W as i32) * 2 / 3) - (TILE_SIZE as i32) / 2;

const SPEED_LIMIT: i32 = 5;

// Flipping bounds
// Roughly anything larger than 30 will not complete flip in jump's time
const FLIP_INCREMENT: f64 = 360.0 / 30.0;

// const LEVEL_LEN: u32 = CAM_W * 3;

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

    fn update_pos(&mut self, x_vel: i32, y_vel: i32) {
        self.pos
            .set_x((self.pos.x() + x_vel).clamp(PLAYER_BOUNDS_H.0, PLAYER_BOUNDS_H.1));
        self.pos
            .set_y((self.pos.y() + y_vel).clamp(PLAYER_BOUNDS_V.0, PLAYER_BOUNDS_V.1));
    }
    /*
    fn update_pos(
        &mut self,
        vel: (i32, i32),
        x_bounds: (i32, i32),
        y_bounds: (i32, i32),
        scroll_offset: i32,
    ) {
        self.pos
            .set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
        self.pos.set_y((self.pos.y() + vel.1).clamp(
            y_bounds.0,
            ground_pos(self.x() - scroll_offset) - (TILE_SIZE as i32),
        ));
    }
    */

    fn texture(&self) -> &Texture {
        &self.texture
    }
}

// What is this?
// fn resist(vel: i32, deltav: i32) -> i32 {
//     if deltav == 0 {
//         if vel > 0 {
//             -1
//         } else if vel < 0 {
//             1
//         } else {
//             deltav
//         }
//     } else {
//         deltav
//     }
// }

/*
// y = -0.05x + 100
fn ground_pos(x: i32) -> i32 {
    let res = (-0.05 * (x as f64) + 100.0) as i32;
    // println!("ground: {}", res);
    (CAM_H as i32) - res
}
*/

impl Game for Runner {
    fn init() -> Result<Self, String> {
        Ok(Runner {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        // ???
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = core.wincan.texture_creator();

        core.wincan.set_draw_color(Color::RGBA(3, 252, 206, 255));
        core.wincan.clear();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        // Textures
        let tex_bg = texture_creator.load_texture("assets/bg.png")?;
        let tex_terrain = texture_creator.load_texture("assets/rolling_hills.png")?;
        let tex_sky = texture_creator.load_texture("assets/sky.png")?;

        // ???
        // let mut scroll_offset = 0;

        // Create player at default position
        let mut player = Player::new(
            rect!(PLAYER_BOUNDS_H.0, PLAYER_BOUNDS_V.0, TILE_SIZE, TILE_SIZE),
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

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        // Score tracking
        let mut score: i32 = 0;

        let mut game_paused: bool = false;
        let mut initial_pause: bool = false;
        let mut game_over: bool = false;
        let mut ct: i32 = 0;

        let mut next_status = GameStatus::Main;

        // Terrain Initialization
        let init_terrain = ProceduralGen::init_terrain(CAM_W as i32, CAM_H as i32, &tex_terrain);
        let mut terrain: LinkedList<TerrainSegment> = LinkedList::new();
        terrain.push_back(init_terrain);

        // Total offset of terrain, also used for background
        let mut OFFSET: i32 = 0;

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
                    break;
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

                // If we want to use keystates instead of events...
                let keystate: HashSet<Keycode> = core
                    .event_pump
                    .keyboard_state()
                    .pressed_scancodes()
                    .filter_map(Keycode::from_scancode)
                    .collect();

                if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
                    //x_deltav = -1;
                    x_vel = -SPEED_LIMIT;
                }
                if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
                    //x_deltav = 1;
                    x_vel = SPEED_LIMIT;
                }

                /*
                x_deltav = resist(x_vel, x_deltav);
                y_deltav = resist(y_vel, y_deltav);
                x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
                y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
                */

                player.update_pos(0, y_vel);
                OFFSET = (OFFSET + x_vel) % CAM_W as i32;
                let bg_offset = -OFFSET;
                /*
                p.update_pos(
                    (x_vel, y_vel),
                    (0, (LEVEL_LEN - TILE_SIZE) as i32),
                    (0, (CAM_H - 2 * TILE_SIZE) as i32),
                    scroll_offset,
                );
                */

                /*
                // Check if we need to updated scroll offset
                scroll_offset = if p.x() > scroll_offset + RTHIRD {
                    (p.x() - RTHIRD).clamp(0, (LEVEL_LEN - CAM_W) as i32)
                } else if p.x() < scroll_offset + LTHIRD {
                    (p.x() - LTHIRD).clamp(0, (LEVEL_LEN - CAM_W) as i32)
                } else {
                    scroll_offset
                };

                // If scroll offest is 0, set it CAM_W and update player pos to account for this
                // update
                if scroll_offset == 0 {
                    scroll_offset = CAM_W as i32;
                    p.update_pos(
                        (CAM_W as i32, y_vel),
                        (0, (LEVEL_LEN - TILE_SIZE) as i32),
                        (0, (CAM_H - 2 * TILE_SIZE) as i32),
                        scroll_offset,
                    );
                }

                // If scroll offest is 2x CAM_W, set it CAM_W and update player pos to account
                // for this update
                if scroll_offset / (CAM_W as i32) == 2 {
                    scroll_offset = CAM_W as i32;
                    p.update_pos(
                        (-(CAM_W as i32), y_vel),
                        (0, (LEVEL_LEN - TILE_SIZE) as i32),
                        (0, (CAM_H - 2 * TILE_SIZE) as i32),
                        scroll_offset,
                    );

                    score += 100;
                }

                let bg_offset = -(scroll_offset % (CAM_W as i32));
                */

                //MODIFIED: G 252 -> 120 (so I could see sky images better)
                core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
                core.wincan.clear();

                // Check if we need to update anything for animation
                flip = if x_vel > 0 && flip {
                    false
                } else if x_vel < 0 && !flip {
                    true
                } else {
                    flip
                };

                src_x = if x_vel != 0 {
                    frames = if (frames + 1) / 6 > 3 { 0 } else { frames + 1 };

                    (frames / 6) * 100
                } else {
                    src_x
                };

                // Draw background
                core.wincan
                    .copy(&tex_bg, None, rect!(bg_offset, 0, CAM_W, CAM_H))?;
                core.wincan.copy(
                    &tex_bg,
                    None,
                    rect!(bg_offset + (CAM_W as i32), 0, CAM_W, CAM_H),
                )?;

                /*** Terrain Section ***/
                // Update all segment postitions
                for segment in terrain.iter_mut() {
                    segment.update_pos(-x_vel, 0);
                }

                // Generate new segment if current tail is visible
                if terrain.back().unwrap().x() <= CAM_W as i32 {
                    let new_segment = ProceduralGen::gen_land(
                        terrain.back().unwrap(),
                        CAM_W as i32,
                        CAM_H as i32,
                        false,
                        false,
                        false,
                        &tex_terrain,
                    );
                    terrain.push_back(new_segment);
                }

                // Delete head segment if invisible
                if terrain.front().unwrap().x() + terrain.front().unwrap().w() <= 0 {
                    terrain.pop_front();
                }

                // Draw all segments
                for segment in terrain.iter() {
                    core.wincan
                        .copy(&(segment.texture()), None, *segment.pos())?;
                }
                /*** End Terrain Section ***/

                //Draw sky in background
                core.wincan
                    .copy(&tex_sky, None, rect!(bg_offset, 0, CAM_W, CAM_H / 3))?;
                core.wincan.copy(
                    &tex_sky,
                    None,
                    rect!(CAM_W as i32 + bg_offset, 0, CAM_W, CAM_H / 3),
                )?;

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
                //NOTE: i added 10 to p.y()
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

        // Out of game loop, return Ok
        Ok(GameState {
            status: Some(next_status),
            score: score,
        })
    }
}
