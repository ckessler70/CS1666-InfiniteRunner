use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
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

        let mut next_status = Some(GameStatus::Main);

        'gameloop: loop {
            core.wincan.set_draw_color(Color::RGBA(119, 120, 123, 255));
            core.wincan.clear();

            // Grey out screen
            core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 128));
            core.wincan.fill_rect(rect!(0, 0, CAM_W, CAM_H))?;

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
        }

        // Out of game loop, return Ok
        Ok(GameState {
            status: next_status,
            score: 0,
        })
    }
}
