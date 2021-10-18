use crate::rect;
use inf_runner::Game;
use inf_runner::SDLCore;

use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
// use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const TILE_SIZE: u32 = 100;

// Bounds we want to keep the player within
const LTHIRD: i32 = ((CAM_W as i32) / 3) - (TILE_SIZE as i32) / 2;
const RTHIRD: i32 = ((CAM_W as i32) * 2 / 3) - (TILE_SIZE as i32) / 2;

const SPEED_LIMIT: i32 = 5;

// Flipping bounds
// Roughly anything larger than 30 will not complete flip in jump's time
const FLIP_INCREMENT: f64 = 360.0/30.0; 

pub struct Demo;

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

// y = -0.05x + 100
fn ground_pos(x: i32) -> i32 {
    let res = (-0.05 * (x as f64) + 100.0) as i32;
    // println!("ground: {}", res);
    (CAM_H as i32) - res
}

impl Game for Demo {
    fn init() -> Result<Self, String> {
        Ok(Demo {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<(), String> {
        let texture_creator = core.wincan.texture_creator();

        core.wincan.set_draw_color(Color::RGBA(3, 252, 206, 255));
        core.wincan.clear();

        // BG is the same size and window, but will scroll as the user moves
        let bg = texture_creator.load_texture("assets/bg.png")?;

        //ADDN
        let rh = texture_creator.load_texture("assets/rolling_hills.png")?;
        let sky = texture_creator.load_texture("assets/sky.png")?;
        let mut scroll_offset = 0;

        let mut level_len: u32 = CAM_W * 2;

        let mut p = Player::new(
            rect!(TILE_SIZE as i32, (CAM_H) as i32, TILE_SIZE, TILE_SIZE),
            texture_creator.load_texture("assets/player.png")?,
        );

        // Used to keep track of animation status
        let mut frames = 0;
        let mut src_x = 0;
        let mut flip = false;

        let mut x_vel = 0;
        let mut y_vel = 0;

        let mut jump = false;
        let mut jump_ct = 0;

        //For rotational flip (maybe not the best variable names)
        let mut r_flip = false;
        let mut r_flip_spot: f64 = 0.0;

        'gameloop: loop {
            let mut x_deltav = 1;
            let mut y_deltav = 1;
            for event in core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'gameloop,
                    _ => {}
                }
                match event {
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => match k {
                        Keycode::W => {
                            if !jump && jump_ct == 0 {
                                jump = true;
                            }
                            if jump && jump_ct != 0{
                                r_flip = true;
                            }
                        }
                        Keycode::Up => {
                            if !jump && jump_ct == 0 {
                                jump = true;
                            }
                            if jump && jump_ct != 0{
                                r_flip = true;
                            }
                        }
                        Keycode::Space => {
                            if !jump && jump_ct == 0 {
                                jump = true;
                            }
                            if jump && jump_ct != 0{
                                r_flip = true;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            if (scroll_offset + RTHIRD) % CAM_W as i32 == 0 {
                level_len = level_len + CAM_W*2;
            }
            if (scroll_offset - LTHIRD) % CAM_W as i32 == 0 {
                level_len = level_len - CAM_W*2;
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
                break;
            }

            // If we want to use keystates instead of events...
            let keystate: HashSet<Keycode> = core
                .event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
                x_deltav = -1;
            }

            x_deltav = resist(x_vel, x_deltav);
            y_deltav = resist(y_vel, y_deltav);
            x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
            y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

            p.update_pos(
                (x_vel, y_vel),
                (0, (level_len - TILE_SIZE) as i32),
                (0, (CAM_H - 2 * TILE_SIZE) as i32),
                scroll_offset,
            );

            // Check if we need to updated scroll offset
            scroll_offset = if p.x() > scroll_offset + RTHIRD {
                (p.x() - RTHIRD).clamp(0, (level_len - CAM_W) as i32)
            } else if p.x() < scroll_offset + LTHIRD {
                (p.x() - LTHIRD).clamp(0, (level_len - CAM_W) as i32)
            } else {
                scroll_offset
            };

            let bg_offset = -(scroll_offset % (CAM_W as i32));
            // let mut brick_offset = -(scroll_offset % (TILE_SIZE as i32));

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
                .copy(&bg, None, rect!(bg_offset, 0, CAM_W, CAM_H))?;
            core.wincan.copy(
                &bg,
                None,
                rect!(bg_offset + (CAM_W as i32), 0, CAM_W, CAM_H),
            )?;

            //Draw rolling hills on top of background
            core.wincan.copy(&rh, None, rect!(0, CAM_H*2/3, CAM_W, CAM_H/3))?;

            //Draw sky in background
            core.wincan.copy(&sky, None, rect!(bg_offset, 0, CAM_W, CAM_H/3))?;
            core.wincan.copy(&sky, None, rect!(CAM_W as i32+bg_offset, 0, CAM_W, CAM_H/3))?;

            //ADDITION: Hey lizard, do a flip
            r_flip_spot = if r_flip && flip{ //going left
                r_flip_spot + FLIP_INCREMENT 

            }else if r_flip && !flip{        //going right
                r_flip_spot - FLIP_INCREMENT

            }else{
                0.0
            };

            //going right backlfip
            if r_flip_spot == -360.0{ //flip complete
                r_flip = false;
                r_flip_spot = 0.0; //reset flip_spot
            }
            //Going left backflip
            if r_flip_spot == 360.0{ //flip complete
                r_flip = false;
                r_flip_spot = 0.0; //reset flip_spot
            }

            // Draw player
            //NOTE: i added 10 to p.y()
            core.wincan.copy_ex(
                p.texture(),
                rect!(src_x, 0, TILE_SIZE, TILE_SIZE),
                rect!(p.x() - scroll_offset, p.y()+10, TILE_SIZE, TILE_SIZE),
                r_flip_spot,
                None,
                flip,
                false,
            )?;

            core.wincan.present();
        }

        // Out of game loop, return Ok
        Ok(())
    }
}
