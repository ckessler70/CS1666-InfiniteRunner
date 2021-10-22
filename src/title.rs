use crate::rect;

use inf_runner::Game;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct Title;

impl Game for Title {
    fn init() -> Result<Self, String> {
        Ok(Title {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameStatus, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = core.wincan.texture_creator();

        core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
        core.wincan.clear();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

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
            .copy(&play_texture, None, Some(rect!(100, 100, 600, 125)))?;
        core.wincan
            .copy(&credits_texture, None, Some(rect!(100, 250, 700, 125)))?;
        core.wincan
            .copy(&quit_texture, None, Some(rect!(100, 400, 1000, 125)))?;

        core.wincan.present();

        let mut game: bool = false;
        let mut credits: bool = false;

        'gameloop: loop {
            for event in core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape | Keycode::Q),
                        ..
                    } => break 'gameloop,
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => match k {
                        Keycode::P | Keycode::Space => {
                            game = true;
                            credits = false;
                            break 'gameloop;
                        }
                        Keycode::C => {
                            game = false;
                            credits = true;
                            break 'gameloop;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        // Out of game loop, return Ok
        Ok(GameStatus {
            main: false,
            game: game,
            credits: credits,
            score: 0,
        })
    }
}
