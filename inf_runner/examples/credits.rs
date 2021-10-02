extern crate inf_runner;

use sdl2::event::Event;
use sdl2::pixels::Color;
// use sdl2::rect::Point;
use sdl2::rect::Rect;
// use sdl2::render::BlendMode;
use sdl2::render::TextureQuery;

use inf_runner::Game;
use inf_runner::SDLCore;

const TITLE: &str = "Credit scene - Dane Halle";
const CAM_W: u32 = 1920;
const CAM_H: u32 = 1080;

pub struct Credits {
    core: SDLCore,
}

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

impl Game for Credits {
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
        Ok(Credits { core })
    }

    fn run(&mut self) -> Result<(), String> {
        'gameloop: loop {
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'gameloop,
                    _ => {}
                }
            }
            self.credit_demo_text()?;
        }
        Ok(())
    }
}

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (CAM_W as i32 - w) / 2;
    let cy = (CAM_H as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

impl Credits {
    fn credit_demo_text(&mut self) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = self.core.wincan.texture_creator();

        let surface = font
            .render("Dane Halle - Debug Guru")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        self.core
            .wincan
            .set_draw_color(Color::RGBA(3, 252, 206, 255));
        self.core.wincan.clear();

        let TextureQuery { width, height, .. } = texture.query();

        let padding = 64;
        let target = get_centered_rect(width, height, CAM_W - padding, CAM_H - padding);

        self.core.wincan.copy(&texture, None, Some(target))?;
        self.core.wincan.present();

        Ok(())
    }
}

fn main() {
    inf_runner::runner(TITLE, Credits::init);
}
