use crate::rect;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const LOWER_SPEED: i32 = 1;
const UPPER_SPEED: i32 = 5;

pub struct Player<'a> {
    pos: Rect,
    velocity: (i32, i32),
    accel: (i32, i32),

    theta: i32, // angle of rotation, in radians
    omega: i32, // angular speed
    alpha: i32, // angular acceleration

    mass: i32,
    texture: Texture<'a>,
    spinning: bool,
}

impl<'a> Player<'a> {
    fn new(pos: Rect, mass: i32, texture: Texture<'a>) -> Player {
        Player {
            pos: pos,
            velocity: (1, 0),
            accel: (0, 0),
            theta: 0,
            omega: 0,
            alpha: 0,
            texture: texture,
            mass: mass,
            spinning: false,
        }
    }

    /****************** Constants ****************/

    fn mass(&self) -> i32 {
        self.mass
    }

    fn rotational_inertia(&self) -> i32 {
        // TODO
        self.mass // to appease compiler
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }

    /****************** Linear motion ****************/

    fn x(&self) -> i32 {
        self.pos.x()
    }

    fn y(&self) -> i32 {
        self.pos.y()
    }

    fn vel_x(&self) -> i32 {
        self.velocity.0
    }

    fn vel_y(&self) -> i32 {
        self.velocity.1
    }

    fn update_pos(&mut self, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
        self.pos
            .set_x((self.x() + self.vel_x()).clamp(x_bounds.0, x_bounds.1));
        self.pos
            .set_y((self.y() + self.vel_y()).clamp(y_bounds.0, y_bounds.1));
    }

    fn update_vel(&mut self) {
        // Update to make the TOTAL MAX VELOCITY constant
        // Right now it's UPPER_SPEED in one direction and UPPER_SPEED*sqrt(2) diagonally
        self.velocity.0 = (self.velocity.0 + self.accel.0).clamp(LOWER_SPEED, UPPER_SPEED);
        self.velocity.1 = (self.velocity.1 + self.accel.1).clamp(0, UPPER_SPEED);
    }

    fn apply_force(&mut self, force: (i32, i32)) {
        self.accel.0 += force.0 / self.mass;
        self.accel.1 += force.1 / self.mass;
    }

    /****************** Angular motion ****************/

    fn theta(&self) -> i32 {
        self.theta
    }

    fn omega(&self) -> i32 {
        self.omega
    }

    fn alpha(&self) -> i32 {
        self.alpha
    }

    // Update_theta
    fn rotate(&mut self, degree: i32) {
        // TODO
    }

    fn update_omega(&mut self) {
        // TODO
    }

    fn apply_torque(&mut self, force: i32, radius: i32) {
        // TODO
        // Update_alpha (angular acceleration)
    }
}

pub struct Obstacle<'a> {
    pos: Rect,
    mass: i32,
    texture: Texture<'a>,
    bouncy: bool,
}

impl<'a> Obstacle<'a> {
    fn new(pos: Rect, mass: i32, texture: Texture<'a>) -> Obstacle {
        Obstacle {
            pos: pos,
            texture: texture,
            mass: mass, // maybe randomize? idk @procedural gen team
            bouncy: false,
        }
    }

    fn mass(&self) -> i32 {
        self.mass
    }

    fn x(&self) -> i32 {
        self.pos.x()
    }

    fn y(&self) -> i32 {
        self.pos.y()
    }

    fn update_pos(&mut self, x: i32, y: i32) {
        self.pos.set_x(x);
        self.pos.set_y(y);
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }
}
