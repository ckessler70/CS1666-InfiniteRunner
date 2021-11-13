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

const TIMEOUT: u64 = 50000;

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

        /*
        let p0: (f64, f64) = (100.0, 100.0);
        let p1: (f64, f64) = (200.0, 200.0);
        let p2: (f64, f64) = (400.0, 150.0);

        //Print points for reference
        core.wincan.set_draw_color(b);
        core.wincan
            .fill_rect(Rect::new(p0.0 as i32, p0.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p1.0 as i32, p1.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p2.0 as i32, p2.1 as i32, 30, 30))?;

        let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
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
        let p0: (f64, f64) = (400.0, 150.0);
        let p1: (f64, f64) = (500.0, 30.0);
        let p2: (f64, f64) = (600.0, 150.0);

        //Print points for reference
        core.wincan.set_draw_color(Color::RGBA(255, 235, 4, 255));
        core.wincan
            .fill_rect(Rect::new(p0.0 as i32, p0.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p1.0 as i32, p1.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(p2.0 as i32, p2.1 as i32, 30, 30))?;

        let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
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
        */

        //Cubic first points
        /*
        let prev_p0: (f64, f64) = (50.0, 650.0);
        let prev_p1: (f64, f64) = (250.0, 200.0);
        let prev_p2: (f64, f64) = (450.0, 600.0);
        let prev_p3: (f64, f64) = (650.0, 400.0);

        //Print points for reference
        core.wincan.set_draw_color(Color::RGBA(255, 0, 0, 255));
        core.wincan
            .fill_rect(Rect::new(prev_p0.0 as i32, prev_p0.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(prev_p1.0 as i32, prev_p1.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(prev_p2.0 as i32, prev_p2.1 as i32, 30, 30))?;
        core.wincan
            .fill_rect(Rect::new(prev_p3.0 as i32, prev_p3.1 as i32, 30, 30))?;

        let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
            proceduralgen::gen_cubic_bezier_curve_points(prev_p0, prev_p1, prev_p2, prev_p3);

        core.wincan.set_draw_color(g);
        for t in 0..BUFF_LENGTH {
            core.wincan.fill_rect(Rect::new(
                group_of_points[t].0 as i32,
                group_of_points[t].1 as i32,
                10,
                10,
            ))?;
        }

        //Cubic second points
        //let p0: (f64, f64) = (650.0, 400.0);
        //let p1: (f64, f64) = (850.0, 200.0);
        let p2: (f64, f64) = (1000.0, 690.0);
        let p3: (f64, f64) = (1200.0, 50.0);

        //Print points for reference
        core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
        /*core.wincan
            .fill_rect(Rect::new(p0.0 as i32, p0.1 as i32, 15, 15))?;
        core.wincan
            .fill_rect(Rect::new(p1.0 as i32, p1.1 as i32, 15, 15))?;
            */
        core.wincan
            .fill_rect(Rect::new(p2.0 as i32, p2.1 as i32, 15, 15))?;
        core.wincan
            .fill_rect(Rect::new(p3.0 as i32, p3.1 as i32, 15, 15))?;

        /*
        let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
            proceduralgen::gen_cubic_bezier_curve_points(p0, p1, p2, p3);
            */

        let temp = group_of_points;
        let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
            proceduralgen::extend_cubic_bezier_curve(prev_p3, prev_p2, p2, p3);

        core.wincan.set_draw_color(b);
        for t in 0..BUFF_LENGTH {
            core.wincan.fill_rect(Rect::new(
                group_of_points[t].0 as i32,
                group_of_points[t].1 as i32,
                10,
                10,
            ))?;
        }
        */

        //core.wincan.set_draw_color(Color::RGBA(255, 0, 255, 255));
        let mut rng = rand::thread_rng();

        //start at (0, 640)
        let mut p0: (f64, f64) = (0.0, 640.0);
        let mut p2: (f64, f64) = (0.0, 0.0); //just instantiate
        let mut p3: (f64, f64) = (0.0, 0.0); //just instantiate
        let mut width_index: f64 = 0.0;
        let mut height_index: f64 = 0.0;

        let mut first_curve: bool = true;

        while (width_index < 1280.0) {
            //set random color
            let temp_color = Color::RGBA(
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                255,
            );
            core.wincan.set_draw_color(temp_color);
            let mut prev_p2: (f64, f64) = p2;
            let mut prev_p3: (f64, f64) = p3;

            let rand_width = rng.gen_range(width_index + 10.0..width_index + 100.0);
            let rand_height = rng.gen_range(height_index + 10.0..height_index + 705.0);
            let p0 = (rand_width, rand_height);

            println!("Width: {}\nHeight: {}\n", width_index, height_index);

            width_index = rand_width;

            let rand_width = rng.gen_range(width_index + 10.0..width_index + 100.0);
            let rand_height = rng.gen_range(height_index + 10.0..height_index + 705.0);
            let mut p1 = (rand_width, rand_height);

            width_index = rand_width;

            let rand_width = rng.gen_range(width_index + 10.0..width_index + 100.0);
            let rand_height = rng.gen_range(height_index + 10.0..height_index + 705.0);
            p2 = (rand_width, rand_height);

            width_index = rand_width;

            let rand_width = rng.gen_range(width_index + 10.0..width_index + 100.0);
            let rand_height = rng.gen_range(height_index + 10.0..height_index + 705.0);
            p3 = (rand_width, rand_height);

            width_index = rand_width;

            if (first_curve) {
                let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
                    proceduralgen::gen_cubic_bezier_curve_points(p0, p1, p2, p3);

                //DRAW
                for t in 0..BUFF_LENGTH {
                    core.wincan.fill_rect(Rect::new(
                        group_of_points[t].0 as i32,
                        group_of_points[t].1 as i32,
                        10,
                        10,
                    ))?;
                }
            } else {
                let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
                    proceduralgen::extend_cubic_bezier_curve(prev_p3, prev_p2, p2, p3);

                //DRAW
                for t in 0..BUFF_LENGTH {
                    core.wincan.fill_rect(Rect::new(
                        group_of_points[t].0 as i32,
                        group_of_points[t].1 as i32,
                        10,
                        10,
                    ))?;
                }
            }

            first_curve = false;
        }

        /*
        let mut rand_height = rng.gen_range(0.0..705.0);
        let mut rand_width = rng.gen_range(0.0..1265.0);
        core.wincan
            .fill_rect(Rect::new(rand_width as i32, rand_height as i32, 15, 15))?;

        rand_height = rng.gen_range(0.0..705.0);
        rand_width = rng.gen_range(0.0..1265.0);
        core.wincan
            .fill_rect(Rect::new(rand_width as i32, rand_height as i32, 15, 15))?;
        rand_height = rng.gen_range(0.0..705.0);
        rand_width = rng.gen_range(0.0..1265.0);
        core.wincan
            .fill_rect(Rect::new(rand_width as i32, rand_height as i32, 15, 15))?;
            */

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
