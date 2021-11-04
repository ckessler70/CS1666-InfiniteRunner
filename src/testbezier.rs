// use crate::physics::Physics;
// use crate::physics::Body;
// use crate::physics::Collider;
use crate::physics::Dynamic;
use crate::physics::Entity;
use crate::physics::Player as PhysPlayer;

use crate::proceduralgen;
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

const TITLE: &str = "Testing Bezier";

const TIMEOUT: u64 = 10000;

pub struct TestBezier;

/*
Modified from Farnan example code, intended for testing purposes only
*/

impl Game for TestBezier {
    fn init() -> Result<Self, String> {
        //let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
        Ok(TestBezier {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        let g = Color::RGBA(0, 255, 0, 255);
        let b = Color::RGBA(0, 0, 255, 255);

        core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        core.wincan.clear();

        /*
        core.wincan.set_draw_color(g);
        core.wincan.draw_point(Point::new(100, 100))?;
        core.wincan.draw_points(
            &[
                Point::new(75, 75),
                Point::new(75, 125),
                Point::new(125, 75),
                Point::new(125, 125),
            ][..],
        )?;

        */

        /*
        core.wincan
            .draw_line(Point::new(500, 300), Point::new(400, 400))?;

        core.wincan.set_draw_color(b);

        core.wincan.draw_lines(
            &[
                Point::new(150, 150),
                Point::new(200, 150),
                Point::new(200, 200),
                Point::new(375, 375),
                Point::new(375, 300),
            ][..],
        )?;

        */

        /*
        core.wincan.set_draw_color(g);
        core.wincan.draw_rect(Rect::new(400, 10, 100, 100))?;

        // Outline overwritten by blue fill_rect() call
        core.wincan.draw_rect(Rect::new(400, 110, 100, 100))?;
        */

        /*
        core.wincan.set_draw_color(b);
        core.wincan.fill_rect(Rect::new(400, 110, 100, 100))?;

        core.wincan.set_draw_color(g);
        core.wincan.fill_rect(Rect::new(400, 300, 100, 100))?;
        */

        //
        //
        //ACTUAL BEZIER TEST IMPLEMENTATION
        //
        //

        //set points is kind of arbitrary
        //p0 = (100,500)
        //p1 = (500,200)
        //p2 = (900, 500)

        //First points
        let p0: (f64, f64) = (20.0, 150.0);
        let p1: (f64, f64) = (100.0, 75.0);
        let p2: (f64, f64) = (180.0, 150.0);

        //Print points for reference
        core.wincan.set_draw_color(b);
        core.wincan
            .fill_rect(Rect::new(p0.0 as i32, p0.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p1.0 as i32, p1.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p2.0 as i32, p2.1 as i32, 30, 30))?;

        let group_of_points: [(f64, f64); BUFF_LENGTH] =
            proceduralgen::gen_quadratic_bezier_curve_points(p0, p1, p2);

        core.wincan.set_draw_color(g);
        for t in 0..BUFF_LENGTH {
            core.wincan.fill_rect(Rect::new(
                group_of_points[t].0 as i32,
                group_of_points[t].1 as i32,
                10,
                10,
            ))?;
        }

        //Second points
        let p0: (f64, f64) = (200.0, 300.0);
        let p1: (f64, f64) = (400.0, 500.0);
        let p2: (f64, f64) = (600.0, 400.0);

        //Print points for reference
        core.wincan.set_draw_color(Color::RGBA(255, 235, 4, 255));
        core.wincan
            .fill_rect(Rect::new(p0.0 as i32, p0.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p1.0 as i32, p1.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p2.0 as i32, p2.1 as i32, 30, 30))?;

        let group_of_points: [(f64, f64); BUFF_LENGTH] =
            proceduralgen::gen_quadratic_bezier_curve_points(p0, p1, p2);

        core.wincan.set_draw_color(g);

        for t in 0..BUFF_LENGTH {
            core.wincan.fill_rect(Rect::new(
                group_of_points[t].0 as i32,
                group_of_points[t].1 as i32,
                10,
                10,
            ))?;
        }

        //Third points
        let p0: (f64, f64) = (400.0, 600.0);
        let p1: (f64, f64) = (600.0, 200.0);
        let p2: (f64, f64) = (800.0, 690.0);
        let p3: (f64, f64) = (1200.0, 150.0);

        //Print points for reference
        core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
        core.wincan
            .fill_rect(Rect::new(p0.0 as i32, p0.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p1.0 as i32, p1.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p2.0 as i32, p2.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p3.0 as i32, p3.1 as i32, 30, 30))?;

        let group_of_points: [(f64, f64); BUFF_LENGTH] =
            proceduralgen::gen_cubic_bezier_curve_points(p0, p1, p2, p3);

        core.wincan.set_draw_color(g);
        for t in 0..BUFF_LENGTH {
            core.wincan.fill_rect(Rect::new(
                group_of_points[t].0 as i32,
                group_of_points[t].1 as i32,
                10,
                10,
            ))?;
        }

        /*
        let chunk_3 = proceduralgen::gen_perlin_hill_point(
                        ((SIZE - 1) as usize + buff_3),
                        freq,
                        amp_3,
                        1.5,
                        256.0,
                    );
                    */

        /*
                pub fn gen_quadratic_bezier_curve_point(
            p0: (f64, f64),
            p1: (f64, f64),
            p2: (f64, f64),
        ) -> [(f64, f64); 32]
        */

        /*

        // Uncomment for red outline
        //self.core.wincan.set_draw_color(g);
        //self.core.wincan.draw_rect(Rect::new(400, 110, 100, 100))?;

        // I <3 Rust iterators
        let rs: Vec<_> = (0..5)
            .map(|i| i * 25)
            .map(|i| Rect::new(225 + i, 225 + i, 25, 25))
            .collect();

        // Up until now, should have been BlendMode::None
        core.wincan.set_blend_mode(BlendMode::Blend);

        core.wincan.set_draw_color(Color::RGBA(0, 255, 0, 128));
        core.wincan.fill_rects(&rs[..])?;

        */

        core.wincan.present();
        thread::sleep(Duration::from_millis(TIMEOUT));

        Ok(GameState {
            status: Some(GameStatus::Main),
            score: 0,
        })
    }
}
