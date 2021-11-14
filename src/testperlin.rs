// use crate::physics::Physics;
// use crate::physics::Body;
// use crate::physics::Collider;
use crate::physics::Dynamic;
use crate::physics::Entity;
use crate::physics::Player as PhysPlayer;

use crate::proceduralgen::noise_2d;
// use crate::proceduralgen::ProceduralGen;
// use crate::proceduralgen::TerrainSegment;

use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;
//use proceduralgen::StaticObject;

// use std::collections::HashSet;
//use std::collections::LinkedList;
use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
// use sdl2::render::Texture;
use sdl2::render::TextureQuery;

use sdl2::rect::Point;

//use std::time::Duration;
use sdl2::render::BlendMode;
use std::thread;

use rand::Rng;

const CAM_H: u32 = 720;
const CAM_W: u32 = 1280;
const TILE_SIZE: u32 = 100;

// Ensure that SIZE is not a decimal
// 1, 2, 4, 5, 8, 10, 16, 20, 32, 40, 64, 80, 128, 160, 256, 320, 640
const SIZE: usize = CAM_W as usize / 10;
const BUFF_LENGTH: usize = CAM_W as usize / 4;

const TITLE: &str = "Testing Perlin";

const TIMEOUT: u64 = 8000;

pub struct TestPerlin;

/*
Modified from Farnan example code, intended for testing purposes only
*/

impl Game for TestPerlin {
    fn init() -> Result<Self, String> {
        //let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
        Ok(TestPerlin {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        let g = Color::RGBA(0, 255, 0, 255);
        let b = Color::RGBA(0, 0, 255, 255);

        core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        core.wincan.clear();

        let x = 1280 / 2;
        let y = 720 / 2;

        let mut rng = rand::thread_rng();

        let freq = rng.gen_range(32.0..300.0);
        let amp = rng.gen_range(0.0..5.0);

        for i in 0..y {
            for j in 0..x {
                let cord = (i, j);

                let n = noise_2d((cord.0 as f64 / (freq), cord.1 as f64 / (freq))) * (amp)
                    + noise_2d((cord.0 as f64 / (freq / 2.0), cord.1 as f64 / (freq / 2.0)))
                        * (amp / 2.0)
                    + noise_2d((cord.0 as f64 / (freq / 4.0), cord.1 as f64 / (freq / 4.0)))
                        * (amp / 4.0)
                    + noise_2d((cord.0 as f64 / (freq / 8.0), cord.1 as f64 / (freq / 8.0)))
                        * (amp / 8.0);
                let modifier = n * 0.5 + 0.5;

                let rgb = 256.0 * modifier;

                core.wincan
                    .set_draw_color(Color::RGB(rgb as u8, rgb as u8, rgb as u8));
                core.wincan
                    .fill_rect(Rect::new(j as i32 * 1, i as i32 * 1, 1, 1));
            }
        }

        println!("{:?} {:?}", freq, amp);

        core.wincan.present();
        thread::sleep(Duration::from_millis(TIMEOUT));

        Ok(GameState {
            status: Some(GameStatus::Main),
            score: 0,
        })
    }
}
