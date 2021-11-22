use inf_runner::PowerType;
use inf_runner::StaticObject;
use inf_runner::TerrainType;

use crate::physics::Power;
use crate::rect;

use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const CAM_W: u32 = 1280;

// SIZE relates to the length of the background hills array.
// Used to convert width of drawn rectangles to fill up the screen.
// Reason for it being 1/10th width is that it was the highest resolution we
// could get with previous iterations of that array and still have good
// performance
const SIZE: usize = CAM_W as usize / 10; // 1/10 of screen for good performance

// Similar to SIZE, the length of the ground_buffer array.
// Reason for it being 1/4th width is arbitrary. As long as it is consistent,
// can be any length const CAM_W: usize = CAM_W as usize / 4; // Why 1/4 of
// screen width specifically?

// Where all the math is done?
pub struct ProceduralGen;

// Representation of a single bezier curve
pub struct TerrainSegment {
    pos: Rect,                               // Bounding box
    curve: [(i32, i32); CAM_W as usize + 1], // Array of points defining the bezier curve
    angle_from_last: f64,                    /* Angle between previous segment and this segment,
                                              * should trend
                                              * downward on average */
    terrain_type: TerrainType,
    color: Color,
}

// Terrain Segment Definitions
impl TerrainSegment {
    pub fn new(
        pos: Rect,
        curve: [(i32, i32); CAM_W as usize + 1],
        angle_from_last: f64,
        terrain_type: TerrainType,
        color: Color,
    ) -> TerrainSegment {
        // Set defaults, should probably be different than this
        TerrainSegment {
            pos: pos,
            curve: curve,
            angle_from_last: angle_from_last,
            terrain_type: terrain_type,
            color: color,
        }
    }

    // Mutators
    // Adjusts terrain postion in runner.rs based on camera_adj_x & camera_adj_y
    pub fn camera_adj(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.set_x(self.pos.x() + x_adj);
        self.pos.set_y(self.pos.y() + y_adj);
        for (x, y) in self.curve.iter_mut() {
            *x += x_adj;
            *y += y_adj;
        }
    }

    // Shifts terrain left so player can "move forward"
    pub fn travel_update(&mut self, travel_adj: i32) {
        self.pos.set_x(self.pos.x() + travel_adj);
        for (x, y) in self.curve.iter_mut() {
            *x += travel_adj;
        }
    }

    /*
        pub fn get_view(&self) -> Vec<(i32, i32)> {
            let mut view: Vec<(i32, i32)> = Vec::new();

            // println!("{:?}", self.pos.x());
            if self.pos.x() > self.curve.len() as i32 {
                return view;
            }

            for i in self.pos.x()..self.pos.x() + SIZE as i32 {
                if i < 0 {
                    continue;
                }

                if i < self.curve.len() as i32 - 1 && i >= 0 {
                    view.push(self.curve[i as usize])
                } else {
                    break;
                }
            }

            return view;
        }
    */

    // Accessors
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

    pub fn pos(&self) -> Rect {
        self.pos
    }

    pub fn angle_from_last(&self) -> f64 {
        self.angle_from_last
    }

    pub fn get_type(&self) -> &TerrainType {
        &self.terrain_type
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn curve(&self) -> [(i32, i32); CAM_W as usize + 1] {
        self.curve
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
    pub fn init_terrain<'a>(cam_w: i32, cam_h: i32, texture: &'a Texture<'a>) /* -> TerrainSegment */
    {
        // TerrainSegment::new(rect!(0, cam_h * 2 / 3, cam_w, cam_h / 3),
        // &texture)
    }

    /*  Initilization of terrain segments
     *
     *  - Takes in `random` which is the array of random tuples of (i32, i32)
     *    Needs to be the same values on each run for porper noise output
     *    Represents the gradient value for points. Passed into gen_point_mod
     *  - Takes in `prev_point` which is the x (assumes 0) and y of the last part
     *    of generated terrain
     *  - Takes in `cam_w` which is the width of the camera (1280)
     *  - Takes in `cam_h` which is the height of the camera (720)
     *  - Takes in `_is_pit` boolean which will generate a pit within this land
     *    segment *NOT IMPLEMENTED YET*
     *  - Takes in `_is_flat` boolean which will make the generated control point
     *    modifiers around the same y and thus, curves should be relatively flat
     *    for the next land segment
     *  - Takes in `_is_cliff` boolean which will make a cliff within the next
     *    land segment *NOT IMPLEMENTED YET*
     *
     *  - Returns array of tuples associated with the output curve.
     */
    pub fn gen_terrain(
        random: &[[(i32, i32); 256]; 256],
        mut prev_point: (f64, f64),
        cam_w: i32,
        cam_h: i32,
        _is_pit: bool,
        _is_flat: bool,
        _is_cliff: bool,
    ) -> TerrainSegment {
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

        prev_point = if _is_pit || _is_cliff {
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

        // Bouncy or not bouncy
        // May be obsolete with TerrainType
        match rng.gen_range(0..=1) {
            1 => curve[curve.len() - 1] = (1, 1),
            _ => curve[curve.len() - 1] = (0, 0),
        }

        let rect = rect!(0, 0, 10, 10); // ?
        let angle_from_last = 0.0; // ?
        let terrain_type = choose_terrain_type(10);
        let color = match (terrain_type) {
            TerrainType::Asphalt => Color::RGB(19, 10, 6),
            TerrainType::Sand => Color::RGB(194, 178, 128),
            TerrainType::Water => Color::RGB(212, 241, 249),
            TerrainType::Grass => Color::RGB(86, 125, 70),
        };

        let terrain = TerrainSegment::new(rect, curve, angle_from_last, terrain_type, color);

        return terrain;
    }

    /*  Currently Deprecated
     *  Similar intent to gen_bezier_land
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
    ) /* -> TerrainSegment */
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

    /*  Handler method to setup necessary parts for generating Bezier Curves
     *
     *  - Takes in `random` which is the array of random tuples of (i32, i32)
     *    Needs to be the same values on each run for porper noise output
     *    Represents the gradient value for points. Passed into gen_point_mod
     *  - Takes in `prev_point` which is the x (assumes 0) and y of the last part
     *    of generated terrain
     *  - Takes in `cam_w` which is the width of the camera (1280)
     *  - Takes in `cam_h` which is the height of the camera (720)
     *  - Takes in `_is_pit` boolean which will generate a pit within this land
     *    segment *NOT IMPLEMENTED YET*
     *  - Takes in `_is_flat` boolean which will make the generated control point
     *    modifiers around the same y and thus, curves should be relatively flat
     *    for the next land segment
     *  - Takes in `_is_cliff` boolean which will make a cliff within the next
     *    land segment *NOT IMPLEMENTED YET*
     *
     *  - Returns array of tuples associated with the output curve.
     */
    pub fn gen_bezier_land(
        random: &[[(i32, i32); 256]; 256],
        mut prev_point: (f64, f64),
        cam_w: i32,
        cam_h: i32,
        _is_pit: bool,
        _is_flat: bool,
        _is_cliff: bool,
    ) -> [(i32, i32); CAM_W as usize + 1] {
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
            curve[curve.len() - 1] = (1, 1); //True value
        } else {
            curve[curve.len() - 1] = (0, 0); //False value
        }

        return (curve);
    }
}

/*  Function for extending a cubic bezier curve while keeping the chained
 * curve  smooth. Works similarly to gen_cubic_bezier_curve_points()
 *      http://www.inf.ed.ac.uk/teaching/courses/cg/d3/bezierJoin.html
 */
pub fn extend_cubic_bezier_curve(
    prev_pn: (f64, f64),
    prev_pn_minus_1: (f64, f64),
    //no p0 or p1, above data structures work instead
    p2: (f64, f64),
    p3: (f64, f64),
) -> [(i32, i32); CAM_W as usize + 1] {
    let mut points: [(i32, i32); CAM_W as usize + 1] = [(-1, -1); CAM_W as usize + 1];

    //Calculate p1
    let mut p1: (f64, f64) = (0.0, 0.0);

    p1.0 = prev_pn.0 + (prev_pn.0 - prev_pn_minus_1.0);
    p1.1 = prev_pn.1 + (prev_pn.1 - prev_pn_minus_1.1);

    for t in 0..CAM_W as usize {
        let point = t as f64;
        //points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / 32.0);
        points[t] = cubic_bezier_curve_point(prev_pn, p1, p2, p3, point / CAM_W as f64);
    }
    return points;
}

/* ~~~~~~     Bezier primary functions      ~~~~~~ */

/*  Handler for getting either quadratic or cubic bezier curve representation
 *
 *  - Takes in `p0` which is the last place the previously generated land
 *    ended
 *  - Takes in `length` which is a control parameter
 *  - Takes in `height` which is a control parameter
 *  - Takes in `point_mod_x` which are the Perlin Noise Modifiers to help
 *    generate control points
 *  - Takes in `buffer` which is a control parameter saying how close control
 *    points can be in the x direction
 *
 *  - Returns Bezier Curve representation
 */
fn gen_bezier_curve(
    p0: (f64, f64),
    length: i32, // Needs to be static which is stupid so 1280
    height: i32,
    point_mod_1: (f64, f64),
    point_mod_2: (f64, f64),
    point_mod_3: (f64, f64),
    buffer: i32,
) -> [(i32, i32); CAM_W as usize + 1] {
    //TODO - CONTROL POINT LOGIC NEEDS TO BE REFINED
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

        let group_of_points: [(i32, i32); CAM_W as usize + 1] =
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

        let group_of_points: [(i32, i32); CAM_W as usize + 1] =
            gen_cubic_bezier_curve_points(p0, p1, p2, p3);

        return group_of_points;
    }
}

/*
 *
 *
 *
 *
 */
// Returns an array of the points' (x,y) values
pub fn gen_cubic_bezier_curve_points(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> [(i32, i32); CAM_W as usize + 1] {
    let mut points: [(i32, i32); CAM_W as usize + 1] = [(-1, -1); CAM_W as usize + 1];

    for t in 0..CAM_W as usize {
        let point = t as f64;
        //points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / 32.0);
        points[t] = cubic_bezier_curve_point(p0, p1, p2, p3, point / CAM_W as f64);
    }
    return points;
}

/*
 *
 *
 *
 *
 */
// Returns an array of the points' (x,y) values
pub fn gen_quadratic_bezier_curve_points(
    p0: (f64, f64), // Start point
    p1: (f64, f64), // Mid point
    p2: (f64, f64), // End point
) -> [(i32, i32); CAM_W as usize + 1] {
    let mut points: [(i32, i32); CAM_W as usize + 1] = [(-1, -1); CAM_W as usize + 1];
    for t in 0..CAM_W as usize {
        let point = t as f64;
        //points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / 32.0);
        points[t] = quadratic_bezier_curve_point(p0, p1, p2, point / CAM_W as f64);
    }
    return points;
}

/******      Bezier helper functions      ***** */

/*
 *
 *
 *
 *
 */
fn cubic_bezier_curve_point(
    p0: (f64, f64), // Start point
    p1: (f64, f64), // Mid_0 point
    p2: (f64, f64), // Mid_1 point
    p3: (f64, f64), // End point
    t: f64,
) -> (i32, i32) {
    let x_value = (1.0 - t) * (1.0 - t) * (1.0 - t) * p0.0
        + 3.0 * (1.0 - t) * (1.0 - t) * t * p1.0
        + 3.0 * (1.0 - t) * t * t * p2.0
        + t * t * t * p3.0;
    let y_value = (1.0 - t) * (1.0 - t) * (1.0 - t) * p0.1
        + 3.0 * (1.0 - t) * (1.0 - t) * t * p1.1
        + 3.0 * (1.0 - t) * t * t * p2.1
        + t * t * t * p3.1;

    return (x_value as i32, y_value as i32);
}

/*
 *
 *
 *
 *
 */
fn quadratic_bezier_curve_point(
    // Point args obtained from perlin
    p0: (f64, f64), // Start point
    p1: (f64, f64), // Mid point
    p2: (f64, f64), // End point
    t: f64,         // t = Point range 0-1 of the curve
) -> (i32, i32) {
    let x_value = (1.0 - t) * ((1.0 - t) * p0.0 + t * p1.0) + t * ((1.0 - t) * p1.0 + t * p2.0);
    let y_value = (1.0 - t) * ((1.0 - t) * p0.1 + t * p1.1) + t * ((1.0 - t) * p1.1 + t * p2.1);
    return (x_value as i32, y_value as i32);
}

/******      Perlin primary functions      ***** */

/*  Generates a single value from the 1d perlin noise
 *
 *  - Takes in point `i` which is the x value we want to get the y of
 *  - Takes in `freq` which is a control value on the cord
 *  - Takes in `amp` which is a control value on the noise_1d outputs
 *  - Takes in `mul` which is a control value on the entire augmented
 *    noise_1d outputs
 *
 *  - Returns the y position associated witht the output of the augmented
 *    outputs
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

/*  Not currently utilized...Can probably be removed
 *  Generates entire perlin map of 128x128
 *
 *  - Takes in `random` which is the array of random tuples of (i32, i32)
 *    Needs to be the same values on each run for porper noise output
 *    Represents the gradient value for points. Passed into noise_2d each
 *    time it is called
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

/*  Generates the advanced perlin noise value for a single point.
 *  Calls noise_2d 4 times to make output more "interesting"
 *
 *  - Takes in `random` which is the array of random tuples of (i32, i32)
 *    Needs to be the same values on each run for porper noise output
 *    Represents the gradient value for points. Passed into noise_2d each
 *    time it is called
 *  - Takes in point values `cord` to get the noise values of
 *  - Takes in `freq` which is a control value on the cord
 *  - Takes in `amp` which is a control value on the noise_2d outputs
 *
 *  - Returns the advanced perlin noise value for given point augmented by
 *    control values
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

/*  Smoothing the input value so the result isn't as "sharp"
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

/*  Determine gradient value for given point p
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

/*  Putting everything together for making the 2d noise
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

/*  Determine gradient value for given value p
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

/*  Putting everything together for making the 1d noise
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

/* ~~~~~~ Random Distributions ~~~~~~ */

/* Randomly choose a TerrainType. Heavily weighted to pick Grass.
 *  - Takes in `upper` which is the top of of the gen_range. Should be >= 3.
 *    Higher it is, more weighted to choose Grass
 *
 *  - Returns a random TerrainType
 */
// Renamed from get_random_terrain
fn choose_terrain_type(upper: i32) -> TerrainType {
    let mut rng = rand::thread_rng();

    let upper = upper.clamp(3, i32::MAX);

    match rng.gen_range(0..=upper) {
        0 => TerrainType::Asphalt,
        1 => TerrainType::Sand,
        2 => TerrainType::Water,
        _ => TerrainType::Grass,
    }
}

/*  Randomly choose a StaticObject
 *
 *  - Returns a random StaticObject
 */
pub fn choose_static_object() -> StaticObject {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=3) {
        0 => StaticObject::Coin,
        1 => StaticObject::Statue,
        2 => StaticObject::Spring,
        _ => StaticObject::Power,
    }
}

/*  Randomly choose a PowerUp
 *
 *  - Returns a random PowerUp
 */
// Probably shouldn't be pub when call is moved to procgen.rs
pub fn choose_power_up() -> PowerType {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=4) {
        // rand 0.8
        0 => PowerType::SpeedBoost,
        1 => PowerType::ScoreMultiplier,
        2 => PowerType::BouncyShoes,
        3 => PowerType::LowerGravity,
        _ => PowerType::Shield,
    }
}
