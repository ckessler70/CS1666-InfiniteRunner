use crate::rect;
// use crate::Physics;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use rand::Rng;

pub struct ProceduralGen;

#[allow(dead_code)]
pub struct TerrainSegment<'a> {
    pos: Rect,
    // curve: Bezier Curve,
    texture: &'a Texture<'a>,
}

#[allow(dead_code)]
impl<'a> TerrainSegment<'a> {
    pub fn new(pos: Rect, texture: &'a Texture<'a>) -> TerrainSegment {
        TerrainSegment { pos, texture }
    }

    pub fn x(&self) -> i32 {
        self.pos.x()
    }

    pub fn y(&self) -> i32 {
        self.pos.y()
    }

    pub fn w(&self) -> i32 {
        self.pos.width() as i32
    }

    pub fn h(&self) -> i32 {
        self.pos.height() as i32
    }

    pub fn pos(&self) -> &Rect {
        &self.pos
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn update_pos(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.set_x(self.pos.x() + x_adj);
        self.pos.set_y(self.pos.y() + y_adj);
    }
}

#[allow(dead_code)]
impl ProceduralGen {
    pub fn init() -> Result<Self, String> {
        Ok(ProceduralGen {})
    }

    pub fn init_terrain<'a>(
        cam_w: i32,
        cam_h: i32,
        texture: &'a Texture<'a>,
    ) -> TerrainSegment<'a> {
        TerrainSegment::new(rect!(0, cam_h * 2 / 3, cam_w, cam_h / 3), &texture)
    }

    pub fn gen_land<'a>(
        prev_segment: &TerrainSegment,
        cam_w: i32,
        cam_h: i32,
        _is_pit: bool,
        _is_flat: bool,
        _is_cliff: bool,
        texture: &'a Texture<'a>,
    ) -> TerrainSegment<'a> {
        //TODO
        //prev_point - Last point of the previouly generated bit of land
        //length - length of next batch of generated land
        //is_pit - binary tick, next batch of land will have a pit in it
        //is_flat - binary tick, next batch of land will be flat or mostly flat
        // (shallow curve) is_cliff - binary tick, next batch of land
        // will have a point where it drops down into a cliff face
        TerrainSegment::new(
            rect!(
                prev_segment.x() + prev_segment.w(),
                prev_segment.y(),
                cam_w,
                cam_h / 3
            ),
            &texture,
        )
    }

    fn gen_noise() -> bool {
        //TODO
        //Perlin noise generation
        false
    }

    fn gen_curve() -> bool {
        //TODO
        //Bezier curve
        false
    }

    pub fn main_runner(&mut self) -> Result<(), String> {
        let mut out = [[0; 32]; 160];

        let amp1 = rand::thread_rng().gen::<f64>();
        let amp2 = rand::thread_rng().gen::<f64>();
        let amp3 = rand::thread_rng().gen::<f64>();
        let amp4 = rand::thread_rng().gen::<f64>();

        for i in 1..160 {
            for j in 1..32 {
                let cord = (i, j);

                let n = noise(cord.0 as f64 * (1.0 / 300.0)) * amp1
                    + noise(cord.0 as f64 * (1.0 / 150.0)) * amp2
                    + noise(cord.0 as f64 * (1.0 / 75.0)) * amp3
                    + noise(cord.0 as f64 * (1.0 / 37.5)) * amp4;
                let y = 2.0 * (cord.1 as f64 / 32.0) - 1.0;

                out[i][j] = if n > y { 1 } else { 0 };
            }
        }
        for i in 1..32 {
            for j in 1..160 {
                let print = if out[j][i] == 1 { '+' } else { '.' };
                print!("{}", print);
            }
            println!("");
        }
        Ok(())
    }
}

pub fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

pub fn grad(p: f64) -> f64 {
    let mut random = [0.0; 256];
    for mut i in random {
        i = rand::thread_rng().gen();
    }
    let v = random[p.floor() as usize];

    return if v > 0.5 { 1.0 } else { -1.0 };
}

pub fn noise(p: f64) -> f64 {
    let p0 = p.floor();
    let p1 = p0 + 1.0;

    let t = p - p0;
    let fade_t = fade(t);

    let g0 = grad(p0);

    let g1 = grad(p1);

    return ((1.0 - fade_t) * g0 * (p - p0) + fade_t * g1 * (p - p1));
}
