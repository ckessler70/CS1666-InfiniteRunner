use crate::rect;
use inf_runner::Game;
use inf_runner::SDLCore;

use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const LEVEL_LEN: u32 = 525600;

const TILE_SIZE: u32 = 100;

// Bounds we want to keep the player within
const LTHIRD: i32 = ((CAM_W as i32) / 3) - (TILE_SIZE as i32) / 2;
const RTHIRD: i32 = ((CAM_W as i32) * 2 / 3) - (TILE_SIZE as i32) / 2;

const SPEED_LIMIT: i32 = 5;

pub struct Demo;

enum PlayerType {
    Lizard,
}

struct Player<'a> {
    pos: Rect,
    src: Rect,
    texture: Texture<'a>,
}

impl<'a> Player<'a> {
    fn new(t: PlayerType, pos: Rect, texture: Texture<'a>) -> Player {
        let (x, y) = match t {
            PlayerType::Lizard => (0, 0),
        };

        let src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
        Player { pos, src, texture }
    }

    fn x(&self) -> i32 {
        self.pos.x()
    }

    fn y(&self) -> i32 {
        self.pos.y()
    }

    fn update_pos(&mut self, vel: (i32, i32), x_bounds: (i32, i32), y_bounds: (i32, i32)) {
        self.pos
            .set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
        self.pos
            .set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
    }

    fn src(&self) -> Rect {
        self.src
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
        let mut scroll_offset = 0;

        // Also drawing bricks again
        let brick_sheet = texture_creator.load_texture("assets/road.png")?;

        let mut p = Player::new(
            PlayerType::Lizard,
            Rect::new(
                (CAM_W / 2 - TILE_SIZE / 2) as i32,
                (CAM_H / 2 - TILE_SIZE / 2) as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
            texture_creator.load_texture("assets/player.png")?,
        );

        let mut x_vel = 0;
        let mut y_vel = 0;

        let mut jump = false;
        let mut jump_ct = 0;

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
                        }
                        Keycode::Up => {
                            if !jump && jump_ct == 0 {
                                jump = true;
                            }
                        }
                        Keycode::Space => {
                            if !jump && jump_ct == 0 {
                                jump = true;
                            }
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

            // If we want to use keystates instead of events...
            // let keystate: HashSet<Keycode> = core
            //     .event_pump
            //     .keyboard_state()
            //     .pressed_scancodes()
            //     .filter_map(Keycode::from_scancode)
            //     .collect();

            // if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) || keystate.contains(&Keycode::Space) {
            //     y_deltav = -1;
            // }

            x_deltav = resist(x_vel, x_deltav);
            y_deltav = resist(y_vel, y_deltav);
            x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
            y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

            p.update_pos(
                (x_vel, y_vel),
                (0, (LEVEL_LEN - TILE_SIZE) as i32),
                (0, (CAM_H - 2 * TILE_SIZE) as i32),
            );

            // Check if we need to updated scroll offset
            scroll_offset = if p.x() > scroll_offset + RTHIRD {
                (p.x() - RTHIRD).clamp(0, (LEVEL_LEN - CAM_W) as i32)
            } else if p.x() < scroll_offset + LTHIRD {
                (p.x() - LTHIRD).clamp(0, (LEVEL_LEN - CAM_W) as i32)
            } else {
                scroll_offset
            };

            let bg_offset = -(scroll_offset % (CAM_W as i32));
            let mut brick_offset = -(scroll_offset % (TILE_SIZE as i32));

            core.wincan.set_draw_color(Color::RGBA(3, 252, 206, 255));
            core.wincan.clear();

            // Draw background
            core.wincan
                .copy(&bg, None, Rect::new(bg_offset, 0, CAM_W, CAM_H))?;
            core.wincan.copy(
                &bg,
                None,
                Rect::new(bg_offset + (CAM_W as i32), 0, CAM_W, CAM_H),
            )?;

            // Draw bricks
            // Why not i = 0 here?
            let mut i = (scroll_offset % ((TILE_SIZE as i32) * 4)) / (TILE_SIZE as i32);
            // What happens if we use `while (brick_offset as u32) < CAM_W {` instead?
            while brick_offset < (CAM_W as i32) {
                let src = Rect::new((i % 4) * (TILE_SIZE as i32), 0, TILE_SIZE, TILE_SIZE);
                let pos = Rect::new(
                    brick_offset,
                    (CAM_H - TILE_SIZE) as i32,
                    TILE_SIZE,
                    TILE_SIZE,
                );

                core.wincan.copy(&brick_sheet, src, pos)?;

                i += 1;
                brick_offset += TILE_SIZE as i32;
            }

            // Draw player
            core.wincan.copy(
                p.texture(),
                p.src(),
                Rect::new(p.x() - scroll_offset, p.y(), TILE_SIZE, TILE_SIZE),
            )?;

            core.wincan.present();
        }

        // Out of game loop, return Ok
        Ok(())
    }
}
