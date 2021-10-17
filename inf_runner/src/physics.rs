use crate::rect;
use sdl2::rect::Rect;
use sdl2::render::Texture;

// use crate::ProceduralGen;

const LOWER_SPEED: i32 = 1;
const UPPER_SPEED: i32 = 5;
const GRAVITY: f64 = 9.80665;

pub struct Physics;

impl Physics {
    fn check_collision(player: &Player, obstacle: &Obstacle) -> bool {
        // TODO
        // Using Rect::has_intersection -> bool OR Rect::intersection -> Rect
        // Apply collision to Player AND Obstacle if necessary (i.e. spin out of control and break object or whatever)
        // This includes force and torque
        false
    }

    fn apply_friction() {
        // TODO
        // fn apply_friction(&player: Player, &surface: Option<Box<ProceduralGen::Surface>>) {
        //      Completely made up ProceduralGen::Surface, but it's there to represent
        //      checking the coefficient of friction of the ground
        //      and using player.apply_force() appropriately

        //      match surface {
        //          Some(s) => {
        //              F_friction = µmg*cos(θ)
        //              let friction: f64 = (s.friction * player.mass() * GRAVITY * f64::cos(player.theta()));
        //              let friction: (i32, i32) = [friction but split into components]
        //              player.apply_force(normal);
        //          }
        //          None => {}
        //      }
        //
        // }
    }

    fn bounce(player: &Player, obstacle: &Obstacle) {
        // TODO
        // Update player velocity
        // Smash block into pieces if we want
        // Broken pieces from collisions was a physics thing Farnan was looking for
    }

    fn buoyancy(player: &Player) {
        // TODO
        // apply_force()
    }
}

pub struct Player<'a> {
    pos: Rect,
    velocity: (i32, i32),
    accel: (i32, i32),

    theta: f64, // angle of rotation, in radians
    omega: f64, // angular speed
    alpha: f64, // angular acceleration

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
            theta: 0.0,
            omega: 0.0,
            alpha: 0.0,
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

    // Should we take in force as a magnitude and an angle? Makes the friction calculation above simpler
    fn apply_force(&mut self, force: (i32, i32)) {
        self.accel.0 += force.0 / self.mass;
        self.accel.1 += force.1 / self.mass;
    }

    /****************** Angular motion ****************/

    // This doesn't NEED to be a function, but still here to keep things uniform
    fn theta(&self) -> f64 {
        self.theta
    }

    fn omega(&self) -> f64 {
        self.omega
    }

    fn alpha(&self) -> f64 {
        self.alpha
    }

    // Update_theta
    fn rotate(&mut self, degree: f64) {
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
