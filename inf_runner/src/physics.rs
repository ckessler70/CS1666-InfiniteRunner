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
        // Apply collision to Player AND Obstacle if necessary (i.e. spin out of control
        // and break object or whatever) This includes force and torque
        false
    }

    fn apply_friction() {
        // TODO
        // fn apply_friction(&player: Player, &surface:
        // Option<Box<ProceduralGen::Surface>>) {      Completely made
        // up ProceduralGen::Surface, but it's there to represent
        //      checking the coefficient of friction of the ground
        //      and using player.apply_force() appropriately

        //      match surface {
        //          Some(s) => {
        //              F_friction = µmg*cos(θ)
        //              let friction: f64 = (s.friction * player.mass() *
        // GRAVITY * f64::cos(player.theta()));              let
        // friction: (i32, i32) = [friction but split into components]
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
        // Broken pieces from collisions was a physics thing Farnan was looking
        // for
    }

    fn buoyancy(player: &Player) {
        // TODO
        // apply_force()
    }
}

/// Object can be represented on the display using a texture, as well as being able to change position and rotation.
pub trait Entity<'a> {
    
    /****************** Constants ******************** */

    /// Returns the `Texture` currently loaded into the `Entity`
    fn texture(&self) -> &Texture<'a>;

    /****************** Linear motion *************** */
    
    /// Returns the x position of the `Entity`'s top left corner
    fn x(&self) -> i32;
    /// Returns the y position of the `Entity`'s top left corner
    fn y(&self) -> i32;
    /// Modifies the position of the `Entity`
    fn update_pos(&mut self);

    /****************** Angular motion *************** */

    /// Returns the `Entity`'s angle of rotation in radians, relative to the
    /// horizontal
    fn theta(&self) -> f64;
    /// Modifies the rotation of the `Entity`
    ///
    /// # Arguments
    ///
    /// * `angle`: the angle to rotate the entity by in radians
    fn rotate(&self, angle: f64);
}

/// Object can collide with other objects using a hitbox
pub trait Collider<'a>: Entity<'a> {

    /****************** Collision ******************** */

    /// Returns the collision boundary of the object as a list of `Rect` stored
    /// in a `Vec`
    fn hitbox(&self) -> Vec<Rect>;
    /// Applies a collision to the `Collider` using the physical attributes of
    /// it and a second `Collider`
    ///
    /// # Arguments
    ///
    /// * `other`: the other `Collider` object that is involved in the collision
    fn collide(&mut self, other: &impl Collider<'a>);
}

/// Object can change its linear velocity and acceleration as well as rotation velocity and acceleration
pub trait Dynamic<'a>: Entity<'a> {

    /****************** Linear motion *************** */

    /// Returns the `Entity`'s x velocity
    fn vel_x(&self) -> i32;
    /// Returns the `Entity`'s y velocity
    fn vel_y(&self) -> i32;
    /// Returns the `Entity`'s x acceleration
    fn accel_x(&self) -> i32;
    /// Returns the `Entitiy`'s y acceleration
    fn accel_y(&self) -> i32;

    /****************** Angular motion *************** */

    /// Returns the `Entity`'s angle of rotation in radians
    fn alpha(&self) -> f64;
    /// Returns the `Body`'s rate of rotation
    fn omega(&self) -> f64;
    /// Modifies the velocity of the `Dynamic` `Entity`
    fn update_vel(&mut self);
    /// Modifies the rotation speed of the `Dynamic` `Entity`
    fn update_omega(&mut self);
}

/// Object has mass and rotational inertia. Object responds to forces and torque, which can be arbitrarily applied to it.
pub trait Body<'a>: Collider<'a> + Dynamic<'a> {

    /****************** Constants ******************** */

    /// Returns the `Body`'s mass
    fn mass(&self) -> i32;
    /// Returns the `Body`'s rotational inertia (i.e. moment of inertia)
    fn rotational_inertia(&self) -> i32;

    /****************** Forces *********************** */

    /// Applies a force to the `Body` that has an x any y component
    ///
    /// # Arguments
    /// * `force`: an array containing the force's x and y components
    ///     * `force[0]` is the x-component
    ///     * `force[1]` is the y-component
    fn apply_force(&mut self, force: [i32; 2]);
    /// Applies torque to the `Body`
    ///
    /// # Arguments
    /// * `force`: the magnitude of the force being applied tangent to the
    ///   object
    /// * `radius`: the distance from the object's center of mass
    fn apply_torque(&mut self, force: i32, radius: i32);

    /****************** Collision ******************** */

    /// Applies a collision to the `Body` with the terrain
    ///
    /// # Arguments
    /// * `terrain_type`: the name of the terrain type the `Body` collided with
    fn collide_terrain(terrain_type: String);
}

///Represents the player character
/// 
/// # Traits
/// * `Body`
/// * `Collider`
/// * `Dynamic`
/// * `Entity`
pub struct Player<'a> {
    pos: Rect,
    velocity: [i32; 2],
    accel: [i32; 2],

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
            pos,
            velocity: [1, 0],
            accel: [0, 0],
            theta: 0.0,
            omega: 0.0,
            alpha: 0.0,
            texture,
            mass,
            spinning: false,
        }
    }
}

impl<'a> Entity<'a> for Player<'a> {
    fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    fn x(&self) -> i32 {
        self.pos.x()
    }

    fn y(&self) -> i32 {
        self.pos.y()
    }

    fn theta(&self) -> f64 {
        self.theta
    }

    fn update_pos(&mut self) {
        self.pos.set_x(self.x() + self.vel_x());
        self.pos.set_y(self.y() + self.vel_y());
    }

    fn rotate(&self, angle: f64) {}
}

impl<'a> Dynamic<'a> for Player<'a> {
    fn vel_x(&self) -> i32 {
        self.velocity[0]
    }

    fn vel_y(&self) -> i32 {
        self.velocity[1]
    }

    fn accel_x(&self) -> i32 {
        self.accel_x()
    }

    fn accel_y(&self) -> i32 {
        self.accel_y()
    }

    fn alpha(&self) -> f64 {
        self.alpha()
    }

    fn omega(&self) -> f64 {
        self.omega()
    }

    fn update_vel(&mut self) {
        // Update to make the TOTAL MAX VELOCITY constant
        // Right now it's UPPER_SPEED in one direction and UPPER_SPEED*sqrt(2)
        // diagonally
        self.velocity[0] = (self.velocity[0] + self.accel[0]).clamp(LOWER_SPEED, UPPER_SPEED);
        self.velocity[1] = (self.velocity[1] + self.accel[1]).clamp(0, UPPER_SPEED);
    }

    fn update_omega(&mut self) {
        // TODO
    }
}

impl<'a> Collider<'a> for Player<'a> {
    fn hitbox(&self) -> Vec<Rect> {
        Vec::new()
        // TODO
    }
    fn collide(&mut self, other: &impl Collider<'a>) {
        // TODO
    }
}

impl<'a> Body<'a> for Player<'a> {
    fn mass(&self) -> i32 {
        self.mass
    }

    fn rotational_inertia(&self) -> i32 {
        // TODO
        todo!();
    }

    // Should we take in force as a magnitude and an angle? Makes the friction
    // calculation above simpler
    fn apply_force(&mut self, force: [i32; 2]) {
        self.accel[0] += force[0] / self.mass;
        self.accel[1] += force[1] / self.mass;
    }

    // This doesn't NEED to be a function, but still here to keep things uniform

    fn apply_torque(&mut self, force: i32, radius: i32) {
        // TODO
        // Update_alpha (angular acceleration)
    }

    fn collide_terrain(terrain_type: String) {
        // TODO
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
            pos,
            texture,
            mass, // maybe randomize? idk @procedural gen team
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
