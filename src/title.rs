use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct Title;

impl Game for Title {
    fn init() -> Result<Self, String> {
        Ok(Title {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = core.wincan.texture_creator();

        core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
        core.wincan.clear();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let surface = font
            .render("Urban Odyssey")
            .blended(Color::RGBA(0, 255, 0, 255))
            .map_err(|e| e.to_string())?;
        let title_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = title_texture.query();

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

        let surface = font
            .render("P/Space - Play")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let play_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("C - Credits")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let credits_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Escape/Q - Quit game")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let quit_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // Grey out screen
        core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 128));
        core.wincan.fill_rect(rect!(0, 0, CAM_W, CAM_H))?;

        // Draw text
        core.wincan
            .copy(&title_texture, None, Some(rect!(cx, 50, w, h)))?;
        core.wincan
            .copy(&play_texture, None, Some(rect!(125, 200, 600, 125)))?;
        core.wincan
            .copy(&credits_texture, None, Some(rect!(125, 350, 700, 125)))?;
        core.wincan
            .copy(&quit_texture, None, Some(rect!(125, 500, 1000, 125)))?;

        core.wincan.present();

        let mut next_status = Some(GameStatus::Main);

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
                        Keycode::B => {
                            next_status = Some(GameStatus::BezierSim);
                            break 'gameloop;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        // Out of game loop, return Ok
        Ok(GameState {
            status: next_status,
            score: 0,
        })
    }
}
