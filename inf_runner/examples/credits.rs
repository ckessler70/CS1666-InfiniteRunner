extern crate inf_runner;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

use inf_runner::Game;
use inf_runner::SDLCore;

const TITLE: &str = "Credit scene - Dane Halle";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const MOVE_PER_FRAME: u32 = 1;

pub struct Credits {
    core: SDLCore,
}

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl Game for Credits {
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
        Ok(Credits { core })
    }

    fn run(&mut self) -> Result<(), String> {
        let mut count = CAM_H;

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = self.core.wincan.texture_creator();

        let surface = font
            .render("Caleb Kessler")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_caleb = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let caleb_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/caleb_hs.jpg")?,
        );

        let surface = font
            .render("Dane Halle")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_dane = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let dane_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/dane_hs.jpg")?,
        );

        let surface = font
            .render("Andrew Wiesen")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_andrew = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let andrew_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/dane_hs.jpg")?,
        );

        let surface = font
            .render("Benjamin Ungar")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_benjamin = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let benjamin_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/dane_hs.jpg")?,
        );

        let surface = font
            .render("Dominic Karras")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_dominic = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let dominic_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/dane_hs.jpg")?,
        );

        let surface = font
            .render("Mateen Kasim")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_mateen = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let mateen_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/dane_hs.jpg")?,
        );

        let surface = font
            .render("Elliot Snitzer")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_elliot = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let elliot_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/elliot_hs.jpg")?,
        );

        let surface = font
            .render("Michael Daley")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_michael = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let michael_hs = Headshot::new(
            rect!((CAM_W / 2 - 400 / 2), 0, 400, 400),
            texture_creator.load_texture("assets/dane_hs.jpg")?,
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

        let mut index = 0;

        'gameloop: loop {
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'gameloop,
                    _ => {}
                }
            }
            let mut i = 0;
            while i < 120 {
                i += 1;
                if count <= MOVE_PER_FRAME + 1 {
                    count = MOVE_PER_FRAME + 1;
                    count = self.credit_demo_text(&count, &team[index], &200, &hs[index])?;
                } else {
                    count = self.credit_demo_text(&count, &team[index], &200, &hs[index])?;
                    break;
                }
            }
            if i == 120 {
                count = CAM_H;
                index += 1;
                if index == team.len() {
                    index = 0;
                }
            } else {
                continue;
            }
        }
        Ok(())
    }
}

impl Credits {
    fn credit_demo_text(
        &mut self,
        count: &u32,
        texture: &sdl2::render::Texture,
        padding: &u32,
        image: &Headshot,
    ) -> Result<u32, String> {
        let m_count = count - MOVE_PER_FRAME;
        //Removal of this and changing instances to just `padding` causes it to break for some reason
        let m_padding = padding;

        // Background wipe
        self.core
            .wincan
            .set_draw_color(Color::RGBA(3, 252, 206, 255));
        self.core.wincan.clear();

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
        self.core
            .wincan
            .copy(&texture, None, Some(rect!(cx, m_count, w, h)))?;

        // Image drawing
        if m_count + m_padding <= CAM_H {
            self.core.wincan.copy(
                image.texture(),
                image.src(),
                rect!(image.x(), m_count + m_padding, 400, 400),
            )?;
        }

        // Only one present needed per frame
        self.core.wincan.present();

        Ok(m_count)
    }
}

fn main() {
    inf_runner::runner(TITLE, Credits::init);
}
