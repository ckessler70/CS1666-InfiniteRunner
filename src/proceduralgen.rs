use crate::rect;
// use crate::Physics;

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

        let mut rng = rand::thread_rng();

        let flat_mod: f64 = 0.25;
        let cliff_min_mod: f64 = 2.0;
        let cliff_max_mod: f64 = 5.0;

        let freq = rng.gen::<f64>() * 256.0 + 32.0;
        let amp: f64 = if _is_flat {
            rng.gen::<f64>() * flat_mod
        } else if _is_cliff {
            rng.gen::<f64>() * cliff_max_mod.clamp(cliff_min_mod, cliff_max_mod)
        } else {
            rng.gen::<f64>()
        };

        // //Generates perlin noise map each terrain
        // let perlin_noise: [[f64; 128]; 128] = gen_perlin_noise(freq, amp);

        // // As mod is closer to 1, it should be higher. As it is closer to 0, it should be lower
        // let point_mod: f64 = perlin_noise
        //     [((rng.gen::<f64>() * (perlin_noise.len() - 1) as f64).floor()) as usize]
        //     [((rng.gen::<f64>() * (perlin_noise.len() - 1) as f64).floor()) as usize];

        // Generates perlin noise for random point instead of whole map
        let map_size = 128;
        let point_mod: f64 = gen_point_mod(
            (
                ((rng.gen::<f64>() * (map_size - 1) as f64).floor()) as i32,
                ((rng.gen::<f64>() * (map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );

        let _curve = gen_bezier_curve(point_mod);

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

    pub fn test_mapper(&mut self) -> Result<(), String> {
        let mut out = [[0.0; 128]; 128];
        let mut random = [[0.0; 64]; 64];

        let mut rng = rand::thread_rng();

        for i in 0..64 {
            for j in 0..64 {
                random[i][j] = rng.gen::<f64>();
            }
        }

        let freq = rng.gen::<f64>() * 256.0 + 32.0;
        let amp = rng.gen::<f64>();

        for i in 0..(out.len() - 1) {
            for j in 0..(out.len() - 1) {
                let cord = (i, j);

                let n = noise(&random, (cord.0 as f64 / 64.0, cord.1 as f64 / (freq))) * (amp)
                    + noise(
                        &random,
                        (cord.0 as f64 / 32.0, cord.1 as f64 / (freq / 2.0)),
                    ) * (amp / 2.0)
                    + noise(
                        &random,
                        (cord.0 as f64 / 16.0, cord.1 as f64 / (freq / 4.0)),
                    ) * (amp / 4.0)
                    + noise(&random, (cord.0 as f64 / 8.0, cord.1 as f64 / (freq / 8.0)))
                        * (amp / 8.0);
                let modifier = n * 0.5 + 0.5;

                out[i][j] = modifier;
            }
        }
        for i in 0..(out.len() - 1) {
            for j in 0..(out.len() - 1) {
                let print = if out[j][i] / 0.1 < 1.0 {
                    ' '
                } else if out[i][j] / 0.1 < 2.0 {
                    '.'
                } else if out[i][j] / 0.1 < 3.0 {
                    ','
                } else if out[i][j] / 0.1 < 4.0 {
                    '-'
                } else if out[i][j] / 0.1 < 5.0 {
                    '|'
                } else if out[i][j] / 0.1 < 6.0 {
                    '"'
                } else if out[i][j] / 0.1 < 7.0 {
                    '='
                } else if out[i][j] / 0.1 < 8.0 {
                    '+'
                } else if out[i][j] / 0.1 < 9.0 {
                    'o'
                } else {
                    'O'
                };
                print!("{}", print);
            }
            println!("");
        }
        Ok(())
    }
}

// Test function used freq = 64.0 and amp = 1.0
fn gen_perlin_noise(freq: f64, amp: f64) -> [[f64; 128]; 128] {
    let mut out = [[0.0; 128]; 128];
    let mut random = [[0.0; 64]; 64];

    let mut rng = rand::thread_rng();

    for i in 0..64 {
        for j in 0..64 {
            random[i][j] = rng.gen::<f64>();
        }
    }

    for i in 0..(out.len() - 1) {
        for j in 0..(out.len() - 1) {
            let cord = (i, j);

            let n = noise(&random, (cord.0 as f64 / 64.0, cord.1 as f64 / (freq))) * (amp)
                + noise(
                    &random,
                    (cord.0 as f64 / 32.0, cord.1 as f64 / (freq / 2.0)),
                ) * (amp / 2.0)
                + noise(
                    &random,
                    (cord.0 as f64 / 16.0, cord.1 as f64 / (freq / 4.0)),
                ) * (amp / 4.0)
                + noise(&random, (cord.0 as f64 / 8.0, cord.1 as f64 / (freq / 8.0))) * (amp / 8.0);
            let modifier = n * 0.5 + 0.5;

            out[i][j] = modifier;
        }
    }
    return out;
}

fn gen_point_mod(cord: (i32, i32), freq: f64, amp: f64) -> f64 {
    let mut random = [[0.0; 64]; 64];

    let mut rng = rand::thread_rng();

    for i in 0..64 {
        for j in 0..64 {
            random[i][j] = rng.gen::<f64>();
        }
    }

    let n = noise(&random, (cord.0 as f64 / 64.0, cord.1 as f64 / (freq))) * (amp)
        + noise(
            &random,
            (cord.0 as f64 / 32.0, cord.1 as f64 / (freq / 2.0)),
        ) * (amp / 2.0)
        + noise(
            &random,
            (cord.0 as f64 / 16.0, cord.1 as f64 / (freq / 4.0)),
        ) * (amp / 4.0)
        + noise(&random, (cord.0 as f64 / 8.0, cord.1 as f64 / (freq / 8.0))) * (amp / 8.0);
    let modifier = n * 0.5 + 0.5;
    return modifier;
}

//Perlin Noise helper function
fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

//Perlin Noise helper function
fn grad(random: &[[f64; 64]; 64], p: (f64, f64)) -> (f64, f64) {
    let v = (
        random[(p.0 / random.len() as f64) as usize][0],
        random[0][(p.1 / random.len() as f64) as usize],
    );
    let n = (v.0 * 2.0 - 1.0, v.1 * 2.0 - 1.0);
    let normalize = (v.0 * v.0 + v.1 * v.1).sqrt();
    return (n.0 / normalize, n.1 / normalize);
}

//Perlin Noise helper function
fn noise(random: &[[f64; 64]; 64], p: (f64, f64)) -> f64 {
    let p0 = (p.0.floor(), p.1.floor());
    let p1 = (p0.0 + 1.0, p0.1);
    let p2 = (p0.0, p0.1 + 1.0);
    let p3 = (p0.0 + 1.0, p0.1 + 1.0);

    let g0 = grad(&random, p0);
    let g1 = grad(&random, p1);
    let g2 = grad(&random, p2);
    let g3 = grad(&random, p3);

    let t0 = p.0 - p0.0;
    let fade_t0 = fade(t0);

    let t1 = p.1 - p0.1;
    let fade_t1 = fade(t1);

    let p_minus_p0 = (p.0 - p0.0, p.1 - p0.1);
    let p_minus_p1 = (p.0 - p1.0, p.1 - p1.1);
    let p_minus_p2 = (p.0 - p2.0, p.1 - p2.1);
    let p_minus_p3 = (p.0 - p3.0, p.1 - p3.1);

    let p0p1 = (1.0 - fade_t0) * (g0.0 * p_minus_p0.0 + g0.1 * p_minus_p0.1)
        + fade_t0 * (g1.0 * p_minus_p1.0 + g1.1 * p_minus_p1.1);
    let p2p3 = (1.0 - fade_t0) * (g2.0 * p_minus_p2.0 + g2.1 * p_minus_p2.1)
        + fade_t0 * (g3.0 * p_minus_p3.0 + g3.1 * p_minus_p3.1);

    return (1.0 - fade_t1) * p0p1 + fade_t1 * p2p3;
}

fn gen_bezier_curve(point_mod: f64) -> bool {
    //TODO
    //Bezier curve
    false
}
