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

const FPS: f64 = 60.0;
const FRAME_TIME: f64 = 1.0 / FPS as f64;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const MOVE_PER_FRAME: u32 = 2;
const ORANGE: Color = Color::RGBA(230, 150, 25, 255);
const SILVER: Color = Color::RGBA(119, 120, 123, 255);

pub struct Credits;

struct Headshot<'a> {
    pos: Rect,
    src: Rect,
    texture: Texture<'a>,
}

impl<'a> Headshot<'a> {
    fn new(pos: Rect, texture: Texture<'a>) -> Headshot {
        let src = rect!(0, 0, 400, 400);
        Headshot { pos, src, texture }
    }

    fn x(&self) -> i32 {
        self.pos.x()
    }

    fn src(&self) -> Rect {
        self.src
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }
}

impl Game for Credits {
    fn init() -> Result<Self, String> {
        Ok(Credits {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        let mut count = CAM_H;

        /********************* TEXTURES AND HEADSHOTS ******************/

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = core.wincan.texture_creator();

        let surface = font
            .render("Caleb Kessler")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_caleb = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let caleb_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/caleb_hs.jpg")?,
        );

        let surface = font
            .render("Dane Halle")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_dane = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let dane_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/dane_hs.jpg")?,
        );

        let surface = font
            .render("Andrew Wiesen")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_andrew = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let andrew_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/andrew_hs.png")?,
        );

        let surface = font
            .render("Benjamin Ungar")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_benjamin = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let benjamin_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/benjamin_hs.jpg")?,
        );

        let surface = font
            .render("Dominic Karras")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_dominic = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let dominic_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/dominic_hs.jpg")?,
        );

        let surface = font
            .render("Mateen Kasim")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_mateen = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let mateen_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/mateen_hs.jpg")?,
        );

        let surface = font
            .render("Elliot Snitzer")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_elliot = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let elliot_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/elliot_hs.jpg")?,
        );

        let surface = font
            .render("Michael Daley")
            .blended(ORANGE)
            .map_err(|e| e.to_string())?;
        let texture_michael = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let michael_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/headshots/michael_hs.jpg")?,
        );

        let team = [
            texture_caleb,
            texture_dane,
            texture_andrew,
            texture_benjamin,
            texture_dominic,
            texture_mateen,
            texture_elliot,
            texture_michael,
        ];

        let hs = [
            caleb_hs,
            dane_hs,
            andrew_hs,
            benjamin_hs,
            dominic_hs,
            mateen_hs,
            elliot_hs,
            michael_hs,
        ];

        /********************************************************************/

        let mut index = 0;
        let mut next_status = GameStatus::Main;

        // FPS tracking
        let mut _all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        'gameloop: loop {
            last_raw_time = Instant::now(); // FPS tracking
            for event in core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape | Keycode::Q),
                        ..
                    } => break 'gameloop,
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        next_status = GameStatus::Game;
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
            let mut i = 0;
            while i < 120 {
                i += 1;
                if count <= MOVE_PER_FRAME + 1 {
                    count = MOVE_PER_FRAME + 1;
                    count = self.credit_text(core, &count, &team[index], &200, &hs[index])?;
                } else {
                    count = self.credit_text(core, &count, &team[index], &200, &hs[index])?;
                    break;
                }
            }
            if i == 120 {
                count = CAM_H;
                index += 1;
                if index == team.len() {
                    break;
                }
            } else {
                continue;
            }

            /* ~~~~~~ FPS Calculation ~~~~~~ */
            // Time taken to display the last frame
            let raw_frame_time = last_raw_time.elapsed().as_secs_f64();
            let delay = FRAME_TIME - raw_frame_time;
            // If the amount of time to display the last frame was less than expected, sleep
            // until the expected amount of time has passed
            if delay > 0.0 {
                // Using sleep to delay will always cause slightly more delay than intended due
                // to CPU scheduling; possibly find a better way to delay
                sleep(Duration::from_secs_f64(delay));
            }
            _all_frames += 1;
            let time_since_last_measurement = last_measurement_time.elapsed();
            // Measures the FPS once per second
            if time_since_last_measurement > Duration::from_secs(1) {
                //println!("{} FPS", _all_frames);
                // println!(
                //     "Average FPS: {:.2}",
                //     (_all_frames as f64) / time_since_last_measurement.as_secs_f64()
                // );
                _all_frames = 0;
                last_measurement_time = Instant::now();
            }
            /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
        }

        Ok(GameState {
            status: Some(next_status),
            score: 0,
        })
    }
}

impl Credits {
    fn credit_text(
        &mut self,
        core: &mut SDLCore,
        count: &u32,
        texture: &sdl2::render::Texture,
        padding: &u32,
        image: &Headshot,
    ) -> Result<u32, String> {
        let m_count = count - MOVE_PER_FRAME;
        //Removal of this and changing instances to just `padding` causes it to break
        // for some reason
        let m_padding = padding;

        // Background wipe
        core.wincan.set_draw_color(SILVER);
        core.wincan.clear();

        let TextureQuery { width, height, .. } = texture.query();

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

        // Print out the name
        core.wincan
            .copy(texture, None, Some(rect!(cx, m_count, w, h)))?;

        // Image drawing
        if m_count + m_padding <= CAM_H {
            core.wincan.copy(
                image.texture(),
                image.src(),
                rect!(image.x(), m_count + m_padding, 400, 400),
            )?;
        }

        // Only one present needed per frame
        core.wincan.present();

        Ok(m_count)
    }
}
