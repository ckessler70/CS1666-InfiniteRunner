use crate::rect;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const BUFF_LENGTH: usize = 320;

pub struct ProceduralGen; // This is getting axed for the refractor I think

pub enum TerrainType {
    Grass,
    Asphalt,
    Sand,
    Water,
}

pub enum StaticObject {
    Coin,
    Statue,
    Power,
    Spring,
}

// Old code, starting point for the overall goal of this refractor
pub struct TerrainSegment {
    pos: Rect,
    // curve: Bezier Curve,
    terrainType: TerrainType,
    color: Color,
}

impl TerrainSegment {
    pub fn new(
        pos: Rect,
        texture: &Texture,
        color: Color,
        terrainType: TerrainType,
    ) -> TerrainSegment {
        TerrainSegment {
            pos,
            terrainType,
            color,
        }
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

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn update_pos(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.set_x(self.pos.x() + x_adj);
        self.pos.set_y(self.pos.y() + y_adj);
    }
}

/*  I don't understand a lot of what's going on in this impl,
 *  but it needs cleaning
 *
 */
#[allow(dead_code)]
impl ProceduralGen {
    pub fn init() -> Result<Self, String> {
        Ok(ProceduralGen {})
    }

    /*
     *
     */
    pub fn init_terrain<'a>(cam_w: i32, cam_h: i32, texture: &'a Texture<'a>) /*-> TerrainSegment*/
    {
        // TerrainSegment::new(rect!(0, cam_h * 2 / 3, cam_w, cam_h / 3), &texture)
    }

    /*
     *
     */
    pub fn gen_land<'a>(
        random: &[[(i32, i32); 256]; 256],
        prev_segment: &TerrainSegment,
        cam_w: i32,
        cam_h: i32,
        _is_pit: bool,
        _is_flat: bool,
        _is_cliff: bool,
        texture: &'a Texture<'a>,
    ) /*-> TerrainSegment*/
    {
        //TODO

        let mut rng = rand::thread_rng();

        let flat_mod: f64 = 0.25;
        let cliff_min_mod: f64 = 2.0;
        let cliff_max_mod: f64 = 5.0;

        let freq = rng.gen_range(32.0..256.0);
        let amp: f64 = if _is_flat {
            rng.gen::<f64>() * flat_mod
        } else if _is_cliff {
            rng.gen::<f64>() * cliff_max_mod.clamp(cliff_min_mod, cliff_max_mod)
        } else {
            rng.gen::<f64>()
        };

        // //Generates perlin noise map each terrain
        // let perlin_noise: [[f64; 128]; 128] = gen_perlin_noise(freq, amp);

        // // As mod is closer to 1, it should be higher. As it is closer to 0, it
        // should be lower let point_mod: f64 = perlin_noise
        //     [((rng.gen::<f64>() * (perlin_noise.len() - 1) as f64).floor()) as usize]
        //     [((rng.gen::<f64>() * (perlin_noise.len() - 1) as f64).floor()) as
        // usize];

        // Generates perlin noise for random point instead of whole map
        let map_size = 128;
        let point_mod_1a: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );
        let point_mod_1b: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );
        let point_mod_2: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );

        // Extract x and y point from last terrain segment
        // let curve = gen_bezier_curve((x, y), cam_w, cam_h, (point_mod_1a,
        // point_mod_1b), point_mod_2, 100);

        //prev_point - Last point of the previouly generated bit of land
        //length - length of next batch of generated land
        //is_pit - binary tick, next batch of land will have a pit in it
        //is_flat - binary tick, next batch of land will be flat or mostly flat
        // (shallow curve) is_cliff - binary tick, next batch of land
        // will have a point where it drops down into a cliff face

        // TerrainSegment::new(
        //     rect!(
        //         prev_segment.x() + prev_segment.w(),
        //         prev_segment.y(),
        //         cam_w,
        //         cam_h / 3
        //     ),
        //     &texture,
        // )
    }

    /*
     *
     */
    pub fn gen_bezier_land(
        random: &[[(i32, i32); 256]; 256],
        mut prev_point: (f64, f64),
        cam_w: i32,
        cam_h: i32,
        _is_pit: bool,
        _is_flat: bool,
        _is_cliff: bool,
    ) -> [(f64, f64); BUFF_LENGTH + 1] {
        //last point will act as bouncy flag.
        let mut rng = rand::thread_rng();

        let flat_mod: f64 = 0.25;
        let cliff_min_mod: f64 = 2.0;
        let cliff_max_mod: f64 = 5.0;

        let freq = rng.gen_range(32.0..256.0);
        let amp: f64 = if _is_flat {
            rng.gen::<f64>() * flat_mod
        } else if _is_cliff {
            rng.gen::<f64>() * cliff_max_mod.clamp(cliff_min_mod, cliff_max_mod)
        } else {
            rng.gen::<f64>()
        };

        // //Generates perlin noise map each terrain
        // let perlin_noise: [[f64; 128]; 128] = gen_perlin_noise(freq, amp);

        // // As mod is closer to 1, it should be higher. As it is closer to 0, it
        // should be lower let point_mod: f64 = perlin_noise
        //     [((rng.gen::<f64>() * (perlin_noise.len() - 1) as f64).floor()) as usize]
        //     [((rng.gen::<f64>() * (perlin_noise.len() - 1) as f64).floor()) as
        // usize];

        // Generates perlin noise for random point instead of whole map
        let map_size = 128;
        let point_mod_1a: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );
        let point_mod_1b: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );
        let point_mod_2a: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );
        let point_mod_2b: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );
        let point_mod_3a: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );
        let point_mod_3b: f64 = gen_point_mod(
            &random,
            (
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
                (rng.gen_range(0.0..(map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );

        prev_point = if _is_pit {
            (prev_point.0, prev_point.1 + 100.0)
        } else {
            prev_point
        };

        // Extract x and y point from last terrain segment
        let mut curve = gen_bezier_curve(
            prev_point,
            cam_w,
            cam_h,
            (point_mod_1a, point_mod_1b),
            (point_mod_2a, point_mod_2b),
            (point_mod_3a, point_mod_3b),
            100,
        );

        let is_bouncy = rng.gen_range(0.0..1.0);

        if (is_bouncy < 0.5) {
            curve[curve.len() - 1] = (1.0, 1.0); //True value
        } else {
            curve[curve.len() - 1] = (0.0, 0.0); //False value
        }

        return (curve);
    }

    /* Handles the object spawning for the game.
     * Includes determining object type and how long until it comes up
     *
     *  - Takes in `random` which is the array of random tuples of (i32, i32)
     *    Needs to be the same values on each run for porper noise output
     *    Represents the gradient value for points.
     *    Passed into gen_point_mod
     *  - Takes in `min_length` and `max_length` which
     *    control min/max distance to this obstacle arriving
     *
     *  - Returns the random StaticObject type and length to that object
     */
    pub fn spawn_object(
        random: &[[(i32, i32); 256]; 256],
        min_length: i32,
        max_length: i32,
    ) -> (Option<StaticObject>, usize) {
        let mut rng = rand::thread_rng();

        let freq = rng.gen::<f64>() * 256.0 + 32.0;
        let amp = rng.gen::<f64>();

        let map_size = 128;
        let point_mod: f64 = gen_point_mod(
            &random,
            (
                ((rng.gen::<f64>() * (map_size - 1) as f64).floor()) as i32,
                ((rng.gen::<f64>() * (map_size - 1) as f64).floor()) as i32,
            ),
            freq,
            amp,
        );

        let object = rand::random();

        let length = (point_mod * max_length as f64 + min_length as f64)
            .clamp(min_length as f64, max_length as f64)
            .floor() as usize;

        (Some(object), length)
    }
}

/*  Function for extending a cubic bezier curve while keeping the chained curve
 *  smooth. Works similarly to gen_cubic_bezier_curve_points()
 *      http://www.inf.ed.ac.uk/teaching/courses/cg/d3/bezierJoin.html
 */
pub fn extend_cubic_bezier_curve(
    prev_pn: (f64, f64),
    prev_pn_minus_1: (f64, f64),
    //no p0 or p1, above data structures work instead
    p2: (f64, f64),
    p3: (f64, f64),
) -> [(f64, f64); BUFF_LENGTH + 1] {
    let mut points: [(f64, f64); BUFF_LENGTH + 1] = [(-1.0, -1.0); BUFF_LENGTH + 1];

    //Calculate p1
    let mut p1: (f64, f64) = (0.0, 0.0);

    p1.0 = prev_pn.0 + (prev_pn.0 - prev_pn_minus_1.0);
    p1.1 = prev_pn.1 + (prev_pn.1 - prev_pn_minus_1.1);

    for t in 0..BUFF_LENGTH {
        let point = t as f64;
        //points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / 32.0);
        points[t] = cubic_bezier_curve_point(prev_pn, p1, p2, p3, point / BUFF_LENGTH as f64);
    }
    return points;
}

/* Randomly choose a TerrainType.
 * Heavily weighted to pick Grass as that should be most common
 *
 *  - Takes in `upper` which is the top of of the gen_range.
 *    Should be >= 3. Higher it is, more weighted to choose Grass
 *
 *  - Returns a random TerrainType
 */
fn get_random_terrain(upper: i32) -> TerrainType {
    let mut rng = rand::thread_rng();

    let upper = upper.clamp(3, i32::MAX);

    match rng.gen_range(0..=10) {
        0 => TerrainType::Asphalt,
        1 => TerrainType::Sand,
        2 => TerrainType::Water,
        _ => TerrainType::Grass,
    }
}

/* Overwriting of `rand::random()` for our use to determine a random StaticObject
 *
 *  - Returns a random StaticObject
 */
impl Distribution<StaticObject> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StaticObject {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=3) {
            // rand 0.8
            0 => StaticObject::Coin,
            1 => StaticObject::Statue,
            2 => StaticObject::Spring,
            _ => StaticObject::Power,
        }
    }
}

/******      Bezier primary functions      ***** */

// Description
fn gen_bezier_curve(
    p0: (f64, f64),
    length: i32, // Needs to be static which is stupid so 1280
    height: i32,
    point_mod_1: (f64, f64),
    point_mod_2: (f64, f64),
    point_mod_3: (f64, f64),
    buffer: i32,
) -> [(f64, f64); BUFF_LENGTH + 1] {
    //TODO
    //Bezier curve

    let mut rng = rand::thread_rng();

    if rng.gen::<f64>() < 0.5 {
        //Quadratic
        let p1: (f64, f64) = (
            (point_mod_1.0 * (length - buffer) as f64 + p0.0 + buffer as f64)
                .clamp(p0.0 + buffer as f64, (length - buffer) as f64),
            (point_mod_1.1 * p0.1 - p0.1).clamp(p0.1 - buffer as f64, height as f64),
        );

        let p2: (f64, f64) = (length as f64 + p0.0, point_mod_2.1 * (height / 3) as f64);

        println!("Quadratic");

        let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
            gen_quadratic_bezier_curve_points(p0, p1, p2);

        return group_of_points;
    } else {
        //Cubic
        let p1: (f64, f64) = (
            (point_mod_1.0 * (length / 2 + buffer) as f64
                + p0.0
                + buffer as f64
                + (length / 2) as f64)
                .clamp(
                    p0.0 + buffer as f64 + (length / 2) as f64,
                    (length - buffer) as f64,
                ),
            (point_mod_1.1 * p0.1 * 2.0 - p0.1).clamp(p0.1 - buffer as f64, height as f64),
        );

        let p2: (f64, f64) = (
            (point_mod_2.0 * (length / 2 - buffer) as f64 + p0.0 + buffer as f64)
                .clamp(p0.0 + buffer as f64, (length / 2 - buffer) as f64),
            (point_mod_2.1 * p0.1 * 2.0 - p0.1).clamp(p0.1 - buffer as f64, height as f64),
        );

        let p3: (f64, f64) = (length as f64 + p0.0, point_mod_3.1 * (height / 3) as f64);

        println!("Cubic");

        let group_of_points: [(f64, f64); BUFF_LENGTH + 1] =
            gen_cubic_bezier_curve_points(p0, p1, p2, p3);

        return group_of_points;
    }
}

// Description
// Returns an array of the points' (x,y) values
pub fn gen_cubic_bezier_curve_points(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> [(f64, f64); BUFF_LENGTH + 1] {
    let mut points: [(f64, f64); BUFF_LENGTH + 1] = [(-1.0, -1.0); BUFF_LENGTH + 1];

    for t in 0..BUFF_LENGTH {
        let point = t as f64;
        //points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / 32.0);
        points[t] = cubic_bezier_curve_point(p0, p1, p2, p3, point / BUFF_LENGTH as f64);
    }
    return points;
}

// Description
// Returns an array of the points' (x,y) values
pub fn gen_quadratic_bezier_curve_points(
    p0: (f64, f64), // Start point
    p1: (f64, f64), // Mid point
    p2: (f64, f64), // End point
) -> [(f64, f64); BUFF_LENGTH + 1] {
    let mut points: [(f64, f64); BUFF_LENGTH + 1] = [(-1.0, -1.0); BUFF_LENGTH + 1];
    for t in 0..BUFF_LENGTH {
        let point = t as f64;
        //points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / 32.0);
        points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / BUFF_LENGTH as f64);
    }
    return points;
}

/******      Bezier helper functions      ***** */

// Description
fn cubic_bezier_curve_point(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    t: f64,
) -> (f64, f64) {
    let x_value = (1.0 - t) * (1.0 - t) * (1.0 - t) * p0.0
        + 3.0 * (1.0 - t) * (1.0 - t) * t * p1.0
        + 3.0 * (1.0 - t) * t * t * p2.0
        + t * t * t * p3.0;
    let y_value = (1.0 - t) * (1.0 - t) * (1.0 - t) * p0.1
        + 3.0 * (1.0 - t) * (1.0 - t) * t * p1.1
        + 3.0 * (1.0 - t) * t * t * p2.1
        + t * t * t * p3.1;

    return (x_value, y_value);
}

// Description
fn quadratic_bezier_curve_point(
    p0: (f64, f64), // Point args obtained from perlin
    p1: (f64, f64),
    p2: (f64, f64),
    t: f64, // t = Point range 0-1 of the curve
) -> (f64, f64) {
    let x_value = (1.0 - t) * ((1.0 - t) * p0.0 + t * p1.0) + t * ((1.0 - t) * p1.0 + t * p2.0);
    let y_value = (1.0 - t) * ((1.0 - t) * p0.1 + t * p1.1) + t * ((1.0 - t) * p1.1 + t * p2.1);
    return (x_value, y_value);
}

/******      Perlin primary functions      ***** */

/* Generates a single value from the 1d perlin noise
 *
 *  - Takes in point `i` which is the x value we want to get the y of
 *  - Takes in `freq` which is a control value on the cord
 *  - Takes in `amp` which is a control value on the noise_1d outputs
 *  - Takes in `mul` which is a control value on the entire augmented noise_1d outputs
 *
 *  - Returns the y position associated witht the output of the augmented outputs
 */
pub fn gen_perlin_hill_point(i: usize, freq: f32, amp: f32, modifier: f32, mul: f32) -> i16 {
    for j in 0..720 {
        let cord = (i, j);

        let n = modifier
            * (noise_1d(cord.0 as f32 * (1.0 / freq)) * amp
                + noise_1d(cord.0 as f32 * (1.0 / freq / 2.0)) * amp / 2.0
                + noise_1d(cord.0 as f32 * (1.0 / freq / 4.0)) * amp / 4.0
                + noise_1d(cord.0 as f32 * (1.0 / freq / 8.0)) * amp / 8.0);

        let y = 2.0 * (cord.1 as f32 / mul) - 1.0;

        if n > y {
        } else {
            return j as i16;
        }
    }
    return 720 as i16;
}

/* Not currently utilized...Can probably be removed
 *  Generates entire perlin map of 128x128
 *
 *  - Takes in `random` which is the array of random tuples of (i32, i32)
 *    Needs to be the same values on each run for porper noise output
 *    Represents the gradient value for points. Passed into noise_2d each time it is called
 *  - Takes in `freq` which is a control value on the cord
 *  - Takes in `amp` which is a control value on the noise_2d outputs
 *
 *  - Returns the entire 128x128 perlin noise map values
 */
fn gen_perlin_noise(random: &[[(i32, i32); 256]; 256], freq: f64, amp: f64) -> [[f64; 128]; 128] {
    let mut out = [[0.0; 128]; 128];

    for i in 0..(out.len() - 1) {
        for j in 0..(out.len() - 1) {
            let cord = (i, j);

            let n = noise_2d(&random, (cord.0 as f64 / 64.0, cord.1 as f64 / (freq))) * (amp)
                + noise_2d(
                    &random,
                    (cord.0 as f64 / 32.0, cord.1 as f64 / (freq / 2.0)),
                ) * (amp / 2.0)
                + noise_2d(
                    &random,
                    (cord.0 as f64 / 16.0, cord.1 as f64 / (freq / 4.0)),
                ) * (amp / 4.0)
                + noise_2d(&random, (cord.0 as f64 / 8.0, cord.1 as f64 / (freq / 8.0)))
                    * (amp / 8.0);
            let modifier = n * 0.5 + 0.5;

            out[i][j] = modifier;
        }
    }
    return out;
}

/* Generates the advanced perlin noise value for a single point.
 *  Calls noise_2d 4 times to make output more "interesting"
 *
 *  - Takes in `random` which is the array of random tuples of (i32, i32)
 *    Needs to be the same values on each run for porper noise output
 *    Represents the gradient value for points.
 *    Passed into noise_2d each time it is called
 *  - Takes in point values `cord` to get the noise values of
 *  - Takes in `freq` which is a control value on the cord
 *  - Takes in `amp` which is a control value on the noise_2d outputs
 *
 *  - Returns the advanced perlin noise value for given point augmented by control values
 */
fn gen_point_mod(random: &[[(i32, i32); 256]; 256], cord: (i32, i32), freq: f64, amp: f64) -> f64 {
    let n = noise_2d(&random, (cord.0 as f64 / (freq), cord.1 as f64 / (freq))) * (amp)
        + noise_2d(
            &random,
            (cord.0 as f64 / (freq / 2.0), cord.1 as f64 / (freq / 2.0)),
        ) * (amp / 2.0)
        + noise_2d(
            &random,
            (cord.0 as f64 / (freq / 4.0), cord.1 as f64 / (freq / 4.0)),
        ) * (amp / 4.0)
        + noise_2d(
            &random,
            (cord.0 as f64 / (freq / 8.0), cord.1 as f64 / (freq / 8.0)),
        ) * (amp / 8.0);
    let modifier = n * 0.5 + 0.5;
    return modifier;
}

/******      Perlin helper functions      ***** */
// Implementation adapted from https://gpfault.net/posts/perlin-noise.txt.html

/* Smoothing the input value so the result isn't as "sharp"
 *  Used for interpolation step of Perlin Noise Algorithm.
 *  Interchangeable between 1d and 2d implementation
 *
 *  - Takes in value `t` to apply fade upon
 *
 *  - Returns smoothed value
 */
fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

/* Determine gradient value for given point p
 *
 *  - Takes in `random` which is the array of random tuples of (i32, i32)
 *    Needs to be the same values on each run for porper noise output
 *    Represents the gradient value for points
 *  - Takes in point `p` to determine gradient of
 *
 *  - Returns unit vector of gradient vector
 */
fn grad_2d(random: &[[(i32, i32); 256]; 256], p: (f64, f64)) -> (f64, f64) {
    let pre_v = random[p.0 as usize % 256][p.1 as usize % 256];

    let v = (pre_v.0 as f64 / 256.0, pre_v.1 as f64 / 256.0);

    let n = (v.0 * 2.0 - 1.0, v.1 * 2.0 - 1.0);

    let length = (n.0 * n.0 + n.1 * n.1).sqrt();

    let unit = (n.0 / length, n.1 / length);

    return unit;
}

/* Putting everything together for making the 2d noise
 *
 *  - Takes in `random` which is the array of random tuples of (i32, i32)
 *    Needs to be the same values on each run for porper noise output
 *    Represents the gradient value for points. Passed into grad_2d
 *  - Takes in point values `p` to give noise output on
 *
 *  - Returns noise output for given values
 */
pub fn noise_2d(random: &[[(i32, i32); 256]; 256], p: (f64, f64)) -> f64 {
    let p0 = (p.0.floor(), p.1.floor());
    let p1 = (p0.0 + 1.0, p0.1);
    let p2 = (p0.0, p0.1 + 1.0);
    let p3 = (p0.0 + 1.0, p0.1 + 1.0);

    let g0 = grad_2d(&random, p0);
    let g1 = grad_2d(&random, p1);
    let g2 = grad_2d(&random, p2);
    let g3 = grad_2d(&random, p3);

    let t0 = p.0 - p0.0;
    let fade_t0 = fade(t0);

    let t1 = p.1 - p0.1;
    let fade_t1 = fade(t1);

    let p_minus_p0 = (p.0 - p0.0, p.1 - p0.1);
    let p_minus_p1 = (p.0 - p1.0, p.1 - p1.1);
    let p_minus_p2 = (p.0 - p2.0, p.1 - p2.1);
    let p_minus_p3 = (p.0 - p3.0, p.1 - p3.1);

    let g0_dot_p0 = g0.0 * p_minus_p0.0 + g0.1 * p_minus_p0.1;
    let g1_dot_p1 = g1.0 * p_minus_p1.0 + g1.1 * p_minus_p1.1;
    let g2_dot_p2 = g2.0 * p_minus_p2.0 + g2.1 * p_minus_p2.1;
    let g3_dot_p3 = g3.0 * p_minus_p3.0 + g3.1 * p_minus_p3.1;

    let p0p1 = (1.0 - fade_t0) * g0_dot_p0 + fade_t0 * g1_dot_p1;
    let p2p3 = (1.0 - fade_t0) * g2_dot_p2 + fade_t0 * g3_dot_p3;

    let result = (1.0 - fade_t1) * p0p1 + fade_t1 * p2p3;

    return result;
}

/* Determine gradient value for given value p
 *  *NOTE*: Some wierdness taking in the random values array but
 *    setting value to consistently output either -1 always or 1 always
 *    gives expected output
 *
 *  - Takes in point `p` to determine gradient of
 *
 *  - Returns binary output (-1 or 1)
 */
fn grad_1d(p: f32) -> f32 {
    let v: f32 = 0.0;

    return if v > 0.5 { 1.0 } else { -1.0 };
}

/* Putting everything together for making the 1d noise
 *
 *  - Takes in point value `p` to give noise output on
 *
 *  - Returns noise output for given value
 */
fn noise_1d(p: f32) -> f32 {
    let p0 = p.floor();
    let p1 = p0 + 1.0;

    let t = p - p0;
    let ft = fade(t as f64) as f32;

    let g0 = grad_1d(p0);
    let g1 = grad_1d(p1);

    return ((1.0 - ft) * g0 * (p - p0) + ft * g1 * (p - p1));
}
