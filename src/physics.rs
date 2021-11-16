// use crate::rect;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::runner::TILE_SIZE as InitTILE_SIZE;
use std::f64::consts::PI;
use std::num;

// use crate::ProceduralGen;

const LOWER_SPEED: f64 = 1.0;
const UPPER_SPEED: f64 = 5.0;
// const GRAVITY: f64 = 9.80665;
const OMEGA: f64 = PI / 18.0;
const TILE_SIZE: f64 = InitTILE_SIZE as f64;

pub struct Physics;

impl Physics {
    pub fn check_collection(player: &Player, coin: &Coin) -> bool {
        // For collection, collsion including force and torque does not need to be accounted for
        // If any of the player hitboxes intersect with a `Collectible`, the object will be aquired by the player
        for h in player.hitbox().iter() {
            if h.has_intersection(coin.hitbox()) {
                return true;
            }
        }
        return false;
    }

    pub fn check_power(player: &Player, power: &Power) -> bool {
        // For collection, collsion including force and torque does not need to be accounted for
        // If any of the player hitboxes intersect with a `Collectible`, the object will be aquired by the player
        for h in player.hitbox().iter() {
            if h.has_intersection(power.hitbox()) {
                return true;
            }
        }
        return false;
    }
    //applies gravity, normal & friction forces
    //depends on whether or not player is on ground
    pub fn apply_gravity<'a>(body: &mut impl Body<'a>, angle: f64, coeff: f64) {
        //onground --> apply gravity in x & y direction based on angle of the ground
        //Note: "angle" is positive going downhill & negative going uphill
        //---- but we always need a negative force in y direction...

        if body.is_onground() {
            // -angle
            //apply gravity in -x & -y
            // body.apply_force((body.mass() * angle.sin(), body.mass() * angle.cos()));
            //apply grav in -y
            body.apply_force((0.0, -body.mass()));

            //apply normal (force positive)
            // body.apply_force((0.0, -body.mass() * angle.cos()));
            //apply normal in -x & +y
            body.apply_force((body.mass() * angle.sin(), -body.mass() * angle.cos()));

            //apply friction (same as gravity in -x)
            // body.apply_force(((body.mass() * angle.sin() * coeff), 0.0));
            //apply fricition in -x & -y
            body.apply_force((
                -coeff * body.mass() * angle.cos(),
                coeff * body.mass() * angle.sin(),
            ));
        } else {
            //player in the air
            //apply entirity of gravity force in -y direction (bc player not on ground)
            body.apply_force((0.0, -body.mass()));
            //no normal, no friction, bc in air
        }
    }

    pub fn apply_friction<'a>(body: &mut impl Body<'a>, coeff: f64) {
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
        // friction: (f64, f64) = [friction but split into components]
        //              player.apply_force(normal);
        //          }
        //          None => {}
        //      }
        //
        // }
        body.apply_force(((-coeff * body.mass()), 0.0));
    }

    fn bounce(player: &Player, obstacle: &Obstacle) {
        // TODO
        // Update player velocity
        // Smash block into pieces if we want
        // Broken pieces from collisions was a physics thing Farnan was looking
        // for
        todo!();
    }

    fn apply_buoyancy(player: &Player) {
        // TODO
        // apply_force()

        // Buoyant force = gravity * density of liquid * (volume of fluid displaced)
        // Also  = mass * grav * (density object/density liquid)
        // we probably just give the player a set volume
        // then see "how far into the liquid it has collided"

        todo!();
    }
}

/// Object can be represented on the display using a texture, as well as being
/// able to change position and rotation.
pub trait Entity<'a> {
    /****************** Constants ******************** */

    /// Returns the `Texture` currently loaded into the `Entity`
    fn texture(&self) -> &Texture<'a>;

    /****************** Linear motion *************** */

    /// Returns the x position of the `Entity`'s top left corner
    fn x(&self) -> i32;
    /// Returns the y position of the `Entity`'s top left corner
    fn y(&self) -> i32;
    /// Returns the center position of the `Entity`
    fn center(&self) -> Point;
    /// Modifies the position of the `Entity`
    fn update_pos(&mut self, ground: Point, angle: f64);

    /****************** Angular motion *************** */

    /// Returns the `Entity`'s angle of rotation in radians, relative to the
    /// horizontal
    fn theta(&self) -> f64;
    /// Modifies the rotation of the `Entity`
    ///
    /// # Arguments
    ///
    /// * `angle`: the angle to rotate the entity by in radians
    fn rotate(&mut self);
}

/// Object can collide with other objects using a hitbox
pub trait Collider<'a>: Entity<'a> {
    /****************** Collision ******************** */

    /// Returns the collision boundary of the object as a list of `Rect` stored
    /// in a `Vec`
    fn hitbox(&self) -> Vec<Rect>;
    /// Checks for collision between two objects that can collide by iterating through all of their hitboxes
    ///
    /// # Arguments
    /// * `other`: the `Collider` object that may collide with the current object
    ///
    /// # Return
    /// * If objects are colliding, return `tuple` as follows:
    ///     1. `Rect`: the hitbox belonging to this object that collided with the other object
    ///     2. `Rect`: the hitbox belonging to the other object that collided with this object
    /// * If the objects are not colliding, return `None`
    fn check_collision(&mut self, other: &impl Collider<'a>) -> Option<(Rect, Rect)> {
        // TODO
        // Using Rect::has_intersection -> bool OR Rect::intersection -> Rect
        // Apply collision to Player AND Obstacle if necessary (i.e. spin out of control
        // and break object or whatever) This includes force and torque
        for h in self.hitbox().iter() {
            for i in other.hitbox().iter() {
                if h.has_intersection(*i) {
                    return Some((*h, *i));
                }
            }
        }
        (None)
    }
    /// Applies a collision to the `Collider` using the physical attributes of
    /// it and a second `Collider`
    ///
    /// # Arguments
    ///
    /// * `other`: the other `Collider` object that is involved in the collision
    fn collide(&mut self, Obstacle: &mut Obstacle, hitboxes: (Rect, Rect), bool: bool) -> bool {
        // if the intersection area is vertical, then the collision was from the side
        if (hitboxes.0.intersection(hitboxes.1).unwrap().height()
            > hitboxes.0.intersection(hitboxes.1).unwrap().width())
        {
            println!("Side collision");
            //otherwise it the collision was on the top or bottom
            true
        } else {
            println!("Top/bottom Collision");
            true
        }
    }
}

/// Object can change its linear velocity and acceleration as well as rotation
/// velocity and acceleration
pub trait Dynamic<'a>: Entity<'a> {
    /****************** Linear motion *************** */

    /// Returns the `Entity`'s x velocity
    fn vel_x(&self) -> f64;
    /// Returns the `Entity`'s y velocity
    fn vel_y(&self) -> f64;
    /// Returns the `Entity`'s x acceleration
    fn accel_x(&self) -> f64;
    /// Returns the `Entitiy`'s y acceleration
    fn accel_y(&self) -> f64;
    // Resets acceleration vector to be recalculated
    fn reset_accel(&mut self);

    /****************** Angular motion *************** */

    /// Returns the `Entity`'s angle of rotation in radians
    // fn alpha(&self) -> f64;
    /// Returns the `Body`'s rate of rotation
    fn omega(&self) -> f64;
    /// Modifies the velocity of the `Dynamic` `Entity`
    fn update_vel(&mut self, fall_rate: f64, speed_adjust: f64);
    // /// Modifies the rotation speed of the `Dynamic` `Entity`
    fn update_omega(&mut self);
}

/// Object has mass and rotational inertia. Object responds to forces and
/// torque, which can be arbitrarily applied to it.
pub trait Body<'a>: Collider<'a> + Dynamic<'a> {
    /****************** Constants ******************** */

    /// Returns the `Body`'s mass
    fn mass(&self) -> f64;
    /// Returns the `Body`'s rotational inertia (i.e. moment of inertia)
    fn rotational_inertia(&self) -> f64;
    //Returns true when play is on the terrain & not in the air
    fn is_onground(&self) -> bool;

    /****************** Forces *********************** */

    /// Applies a force to the `Body` that has an x any y component
    ///
    /// # Arguments
    /// * `force`: an array containing the force's x and y components
    ///     * `force.0` is the x-component
    ///     * `force.1` is the y-component
    fn apply_force(&mut self, force: (f64, f64));
    // // / Applies torque to the `Body`
    // // /
    // // / # Arguments
    // // / * `force`: the magnitude of the force being applied tangent to the
    // // /   object
    // // / * `radius`: the distance from the object's center of mass
    fn apply_torque(&mut self, force: f64, radius: f64);

    //set the normal force acting on the player
    //fn set_normal(&mut self,normal: f64);

    /****************** Collision ******************** */

    // /// Applies a collision to the `Body` with the terrain
    // ///
    // /// # Arguments
    // /// * `terrain_type`: the name of the terrain type the `Body` collided with
    // fn collide_terrain(&mut self, ground: Point, angle: f64, terrain_type: String);
}

///Represents the player character
///
/// # Traits
/// * `Body`
/// * `Collider`
/// * `Dynamic`
/// * `Entity`
pub struct Player<'a> {
    pos: (f64, f64),
    velocity: (f64, f64),
    accel: (f64, f64),
    hitbox: Rect,

    theta: f64, // angle of rotation, in radians
    omega: f64, // angular speed
    alpha: f64, // angular acceleration
    mass: f64,
    texture: Texture<'a>,
    jumping: bool,
    flipping: bool,
    onground: bool,
}

impl<'a> Player<'a> {
    pub fn new(hitbox: Rect, mass: f64, texture: Texture<'a>) -> Player {
        Player {
            pos: (hitbox.x() as f64, hitbox.y() as f64),
            velocity: (3.0, 0.0),
            accel: (0.0, 0.0),
            hitbox,

            theta: 0.0,
            omega: 0.0,
            alpha: 0.0,
            texture,
            mass,
            jumping: false,
            flipping: false,
            onground: false,
            //normal: 0,
        }
    }

    pub fn is_onground(&self) -> bool {
        self.onground
    }

    pub fn is_jumping(&self) -> bool {
        self.jumping
    }

    pub fn is_flipping(&self) -> bool {
        self.flipping
    }

    pub fn stop_flipping(&mut self) {
        self.flipping = false;

        //if the player was flipping previously and still in the air,
        //want to continue rotation but with a negative angular acceleration
        self.omega = OMEGA;
        self.alpha = -0.005;

        // if self.theta() >= OMEGA * 3.0 {
        //     self.theta = 0.0;
        // }
    }

    pub fn set_y_vel_temp(&mut self, speed: f64){
        self.velocity.1 = speed
    }

    pub fn resume_flipping(&mut self) {
        self.flipping = true;
        self.omega = OMEGA;
        self.rotate();
    }

    // Returns true if a jump was initiated
    pub fn jump(&mut self, ground: Point, bouncy: bool, change: f64) -> bool {
        // Bouncy will not set flipping to true by default if true
        // Change is a way to change height of jump depending on value
        if bouncy {
            if self.hitbox.contains_point(ground) {
                self.velocity.1 = 25.0 + change;
                self.jumping = true;
                self.onground = false;

                self.omega = OMEGA;
                self.flipping = false;

                true
            } else {
                false
            }
        } else {
            if self.hitbox.contains_point(ground) {
                self.velocity.1 = 25.0 + change;
                self.jumping = true;
                self.onground = false;

                self.omega = OMEGA;
                self.flipping = true;

                true
            } else {
                false
            }
        }
    }

    pub fn flip(&mut self) {
        if self.is_flipping() {
            self.omega = OMEGA;
            self.rotate();
        }
        //******************************************************
        //UNCOMMENT BELOW TO ADD ANGULAR MOMENTUM AFTER FLIPPING
        // NEEDS SOME MORE WORK THOUGH
        //******************************************************

        //else if(self.omega!=0.0){
        //    self.alpha = -0.005;
        //    if(self.omega > self.alpha.abs()){
        //        self.omega = self.omega + self.alpha;
        //    }else{
        //        self.omega = 0.0;
        //    }
        //    self.rotate();
        //}
        
    }

    // Returns false if the player crashed
    // Uses slope of ground to approximate landing angle
    // angle param is relative to the horizontal
    //   - flat ground has angle 0
    //   - ground sloped UP has negative angle
    //   - ground sloped DOWN has positive angle
    pub fn collide_terrain(&mut self, ground: Point, angle: f64) -> bool {
        if self.vel_y() <= 0.0 && self.hitbox.contains_point(ground) {
            if self.jumping {
                self.velocity.1 = 0.0;
            }
            self.pos.1 = (ground.y() as f64) - 0.95 * TILE_SIZE;
            self.align_hitbox_to_pos();
            self.jumping = false;
            self.onground = true;

            // This is normal force ... is this being applied twice in our code?
            self.apply_force((0.0, self.mass()));

            self.omega = 0.0;

            if self.theta() < OMEGA * 6.0 + angle || self.theta() > 2.0 * PI - OMEGA * 6.0 + angle {
                self.theta = angle;
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }
}

impl<'a> Entity<'a> for Player<'a> {
    fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    fn x(&self) -> i32 {
        self.hitbox.x()
    }

    fn y(&self) -> i32 {
        self.hitbox.y()
    }

    fn center(&self) -> Point {
        self.hitbox.center()
    }

    fn theta(&self) -> f64 {
        self.theta
    }

    fn update_pos(&mut self, ground: Point, angle: f64) {
        if self.hitbox.contains_point(ground) {
            self.theta = angle;
        }

        // Player's x position is fixed
        self.pos.1 -= self.vel_y();
        self.align_hitbox_to_pos();
    }

    fn rotate(&mut self) {
        self.theta = (self.theta - self.omega + 2.0 * PI) % (2.0 * PI);
    }
}

impl<'a> Dynamic<'a> for Player<'a> {
    fn vel_x(&self) -> f64 {
        self.velocity.0
    }

    fn vel_y(&self) -> f64 {
        self.velocity.1
    }

    fn accel_x(&self) -> f64 {
        self.accel.0
    }

    fn accel_y(&self) -> f64 {
        self.accel.1
    }

    fn reset_accel(&mut self) {
        self.accel = (0.0, 0.0);
    }

    fn omega(&self) -> f64 {
        self.omega
    }

    // fn alpha(&self) -> f64 {
    //     self.alpha
    // }

    // Fall rate is the lower clamp value for the y velocity
    // Speed adjust is the augmenting value for the x velocity
    fn update_vel(&mut self, fall_rate: f64, speed_adjust: f64) {
        // Update to make the TOTAL MAX VELOCITY constant
        // Right now it's UPPER_SPEED in one direction and UPPER_SPEED*sqrt(2)
        // diagonally
        self.velocity.0 =
            (self.velocity.0 + self.accel.0 + speed_adjust).clamp(LOWER_SPEED, UPPER_SPEED);
        self.velocity.1 = (self.velocity.1 + self.accel.1).clamp(fall_rate, 1000.0);
    }

    fn update_omega(&mut self) {
        //Update omega to change player rotational velocity
        if (self.omega < self.alpha.abs()) {
            self.omega = 0.0;
        } else {
            self.omega = self.omega - self.alpha;
        }
    }
}

impl<'a> Collider<'a> for Player<'a> {
    fn hitbox(&self) -> Vec<Rect> {
        vec![self.hitbox]
    }

    fn collide(&mut self, obstacle: &mut Obstacle, hitboxes: (Rect, Rect), shielded: bool) -> bool {
        let mut result = false;

        // if the collision box is taller than it is wide, the player hit the side of the object
        if (hitboxes.0.intersection(hitboxes.1).unwrap().height()
            > hitboxes.0.intersection(hitboxes.1).unwrap().width())
        {
            println!("collided with side of obstacle");

            /********** ELASTIC COLLISION CALCULATION **********/
            // https://en.wikipedia.org/wiki/Elastic_collision#One-dimensional_Newtonian
            // Assumed object has velocity (0,0)
            // Assumed player has velocity (vx,vy)
            let angle = ((self.center().y() - obstacle.center().y()) as f64
                / (self.center().x() - obstacle.center().x()) as f64)
                .atan();
            let p_mass = self.mass();
            let o_mass = obstacle.mass();
            let p_vx = self.velocity.0;
            let p_vy = self.velocity.1;
            let p_vx_f = (p_mass - o_mass) * (p_vx) / (p_mass + o_mass);
            let p_vy_f = (p_mass - o_mass) * (p_vy) / (p_mass + o_mass);
            let o_vx_f = (2.0 * p_mass) * (p_vx) / (p_mass + o_mass);
            let o_vy_f = (2.0 * p_mass) * (p_vy) / (p_mass + o_mass);

            // println!("INTENDED TRAJECTORIES: ELASTIC COLLISION: ");
            // println!("\tplayer mass: {}", p_mass);
            // println!("\tobject mass: {}", o_mass);
            // println!("\tplayer initial velocity: ({},{})", p_vx, p_vy);
            // println!("\tobject initial velocity: ({},{})", 0, 0);
            // println!("\tangle from player to object in rads: {}", angle);
            // println!("\tplayer final velocity({},{})", p_vx_f, p_vy_f);
            // println!("\tobject final velocity({},{})", o_vx_f, o_vy_f);
            /***************************************************/
            match obstacle.o_type {
                ObstacleType::Statue => {
                    if shielded {
                        //player has shield
        
                        //self.align_hitbox_to_pos();
                        //other.pos.0 = 6.5;
                        
                        obstacle.apply_force((-0.4, 1.0));
                        // obstacle.velocity.0 = o_vx_f;
                        // obstacle.velocity.1 = o_vy_f;
                        obstacle.collided = true;
                        //println!("ayOb{}", Obstacle.accel_y());
        
                        //.0 = o_vx_f;
                        return true
                        
                    } else {
                        //player does not have shield
                        self.pos.0 = (obstacle.x() as f64 - 1.05 * TILE_SIZE);
                        self.velocity.0 = p_vx_f;
                        self.velocity.1 = p_vy_f;
                        self.align_hitbox_to_pos();
                        return false
                    }
                }
                ObstacleType::Spring => {
                    self.set_y_vel_temp(40.0);
                    print!("Spring");  
                    return true
                }
                _ => return true
            }
           

            // self.velocity.0 = 0.0;
            // self.apply_force((self.mass(), 0.0));

            // for now (week 5), end the game when the player hits the side of an object
            // alternately, set this value to true to cause the player to stop when they run into the object
            // the screen does not follow the player when they stop
            //bool
        }
        // if the collision box is wider than it is tall, the player hit the top of the object
        // don't apply the collision to the top of an object if the player is moving upward, otherwise they will "stick" to the top on the way up
        else if (self.vel_y() < 0.0) {
            self.pos.1 = (obstacle.y() as f64 - 0.95 * (TILE_SIZE as f64));
            self.align_hitbox_to_pos();
            self.velocity.1 = 0.0;
            self.jumping = false;
            self.apply_force((0.0, self.mass()));
            println!("collided with top of obstacle");
            self.omega = 0.0;
            if self.theta() < OMEGA * 6.0 || self.theta() > 360.0 - OMEGA * 6.0 {
                self.theta = 0.0;
                // Add Hooke's law bounce here
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

impl<'a> Body<'a> for Player<'a> {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn is_onground(&self) -> bool {
        self.onground
    }

    /*fn set_normal(&mut self, normal: f64){
        self.normal = normal
    }*/

    fn rotational_inertia(&self) -> f64 {
        // TODO:
        // Rotaional inertia -- I = L/omega
        // I think we'll wanna use L = mass*R^2     (ie. angular momentum for a sphere/thing with effective radius R)
        // Torque (if we need it) tau = I * alpha
        if !self.jumping {
            return 0.0;
        }
        let mut effective_radius: f64;
        if self.flipping {
            effective_radius = TILE_SIZE / 4.0;
        } else {
            effective_radius = TILE_SIZE / 2.0;
        }
        let mut L: f64 = self.mass * effective_radius * effective_radius;
        let mut rot_inertia: f64 = L / self.omega;

        rot_inertia
    }

    // Should we take in force as a magnitude and an angle? Makes the friction
    // calculation above simpler
    fn apply_force(&mut self, force: (f64, f64)) {
        self.accel.0 += force.0 / self.mass;
        self.accel.1 += force.1 / self.mass;
    }

    fn apply_torque(&mut self, force: f64, radius: f64) {
        // TODO
        // Update_alpha (angular acceleration)
        //`force`: magnitude of the force being applied tangent to the object
        //For the above described force, we can use the equation F=mr(omega)
        //instead of the formula for torque, T=I(omega)
        self.alpha = (self.mass * radius) / force;
    }
}
//Dynamic
//Entity
//Collider
//Body
pub struct Obstacle<'a> {
    pub pos: (f64, f64),
    pub velocity: (f64, f64),
    pub accel: (f64, f64),
    pub hitbox: Rect,
    pub o_type: ObstacleType,

    pub mass: f64,
    texture: Texture<'a>,

    bouncy: bool,
    theta: f64,
    omega: f64,
    alpha: f64,
    

    onground: bool,
    jumping: bool,
    flipping: bool,
    collided: bool,
}

pub enum ObstacleType{
    Statue,
    Spring,
}

/// #TODO
/// * Refactor Obstacle to use traits
/// * Make multiple types of obstacles, some that move and some that do not
/// * Add default impls of certain obstacle traits so that it is easier to make
/// different types of obstacles
impl<'a> Obstacle<'a> {
    pub fn new(hitbox: Rect, mass: f64, texture: Texture<'a>, o_type: ObstacleType) -> Obstacle {
        Obstacle {
            pos: (hitbox.x() as f64, hitbox.y() as f64),
            velocity: (0.0, 0.0),
            accel: (0.0, 0.0),
            o_type,
            hitbox,
            texture,
            mass, // maybe randomize? idk @procedural gen team
            bouncy: false,
            theta: 0.0,
            omega: 0.0,
            alpha: 0.0,

            onground: true,
            jumping: false,
            flipping: false,
            collided: false,
        }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn collided(&self) -> bool {
        self.collided
    }

    //This is gonna need a better implementation
    //right now: just detects collision w/ image Rect
    //future: need tighter hitboxes, per obstacle
    pub fn hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn x(&self) -> i32 {
        self.hitbox.x()
    }

    pub fn y(&self) -> i32 {
        self.hitbox.y()
    }

    pub fn omega(&self) -> f64 {
        self.omega
    }

    pub fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

//a lot of these need overwriten this is copied from player impl of entity
impl<'a> Entity<'a> for Obstacle<'a> {
    fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    fn x(&self) -> i32 {
        self.hitbox.x()
    }

    fn y(&self) -> i32 {
        self.hitbox.y()
    }

    fn center(&self) -> Point {
        self.hitbox.center()
    }

    fn theta(&self) -> f64 {
        self.theta
    }

    fn update_pos(&mut self, ground: Point, angle: f64) {
        if self.collided {
            self.theta += angle;
        }

        self.pos.0 -= self.velocity.0;
        self.pos.1 -= self.velocity.1;
        //println!("AAAAA {}", self.pos.1);

        self.align_hitbox_to_pos();
    }

    fn rotate(&mut self) {
        todo!();
        //self.theta = (self.theta - self.omega) % 360.0;
    }
}

impl<'a> Dynamic<'a> for Obstacle<'a> {
    fn vel_x(&self) -> f64 {
        self.velocity.0
    }

    fn vel_y(&self) -> f64 {
        self.velocity.1
    }

    fn accel_x(&self) -> f64 {
        self.accel.0
    }

    fn accel_y(&self) -> f64 {
        self.accel.1
    }

    fn reset_accel(&mut self) {
        self.accel = (0.0, 0.0);
    }

    fn omega(&self) -> f64 {
        self.omega
    }

    // fn alpha(&self) -> f64 {
    //     self.alpha
    // }

    // Fall rate is the lower clamp value for the y velocity
    // Speed adjust is the augmenting value for the x velocity
    fn update_vel(&mut self, fall_rate: f64, speed_adjust: f64) {
        // Update to make the TOTAL MAX VELOCITY constant
        // Right now it's UPPER_SPEED in one direction and UPPER_SPEED*sqrt(2)
        // diagonally
        self.velocity.0 = (self.velocity.0 + self.accel.0).clamp(-20.0, 20.0);
        self.velocity.1 = (self.velocity.1 + self.accel.1).clamp(-20.0, 20.0);
        //(self.velocity.0 + self.accel.0 + speed_adjust).clamp(LOWER_SPEED, UPPER_SPEED);
        //self.velocity.1 = (self.velocity.1 + self.accel.1).clamp(fall_rate, 1000.0);
    }

    fn update_omega(&mut self) {
        //Update omega to change player rotational velocity
        if (self.omega < self.alpha.abs()) {
            self.omega = 0.0;
        } else {
            self.omega = self.omega() - self.alpha;
        }
    }
}

impl<'a> Body<'a> for Obstacle<'a> {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn is_onground(&self) -> bool {
        self.onground
    }

    /*fn set_normal(&mut self, normal: f64){
        self.normal = normal
    }*/

    fn rotational_inertia(&self) -> f64 {
        // TODO:
        // Rotaional inertia -- I = L/omega
        // I think we'll wanna use L = mass*R^2     (ie. angular momentum for a sphere/thing with effective radius R)
        // Torque (if we need it) tau = I * alpha
        if !self.jumping {
            return 0.0;
        }
        let mut effective_radius: f64;
        if self.flipping {
            effective_radius = TILE_SIZE / 4.0;
        } else {
            effective_radius = TILE_SIZE / 2.0;
        }
        let mut L: f64 = self.mass * effective_radius * effective_radius;
        let mut rot_inertia: f64 = L / self.omega;

        rot_inertia
    }

    // Should we take in force as a magnitude and an angle? Makes the friction
    // calculation above simpler
    fn apply_force(&mut self, force: (f64, f64)) {
        self.accel.0 += force.0 / self.mass;
        self.accel.1 += force.1 / self.mass;
    }

    fn apply_torque(&mut self, force: f64, radius: f64) {
        // TODO
        // Update_alpha (angular acceleration)
        //`force`: magnitude of the force being applied tangent to the object
        //For the above described force, we can use the equation F=mr(omega)
        //instead of the formula for torque, T=I(omega)
        self.alpha = (self.mass * radius) / force;
    }
}

//same with this
impl<'a> Collider<'a> for Obstacle<'a> {
    fn hitbox(&self) -> Vec<Rect> {
        vec![self.hitbox]
    }
    fn collide(&mut self, Obstacle: &mut Obstacle, hitboxes: (Rect, Rect), bool: bool) -> bool {
        // TODO
        todo!();
    }
}

pub trait Collectible<'a> {
    /****************** Collection ******************** */

    /// Returns the collection boundary of the object as a `Rect`
    fn hitbox(&self) -> Rect;
    /// Applies a collection to the `Collectible` using the physical attributes of
    /// it and another object that must be of type `Collider`
    ///
    // collect the collectible (set its collected field to true & delete it)
    fn collect(&mut self);
}

pub struct Coin<'a> {
    pub pos: (f64, f64),
    pub hitbox: Rect,
    texture: Texture<'a>,
    value: i32,
    pub collected: bool,
}

impl<'a> Coin<'a> {
    pub fn new(hitbox: Rect, texture: Texture<'a>, value: i32) -> Coin {
        Coin {
            pos: (hitbox.x() as f64, hitbox.y() as f64),
            texture,
            hitbox,
            value,
            collected: false,
        }
    }

    pub fn x(&self) -> i32 {
        self.hitbox.x()
    }

    pub fn y(&self) -> i32 {
        self.hitbox.y()
    }

    pub fn update_pos(&mut self, x: f64, y: f64) {
        self.pos.0 = x;
        self.pos.1 = y;
        self.align_hitbox_to_pos();
    }

    pub fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn collected(&self) -> bool {
        self.collected
    }
    //if we delete coin by dropping them from mem (once collected)
    // pub fn drop(&mut self) {}
}

impl<'a> Collectible<'a> for Coin<'a> {
    //for now (honestly not a horrible long term soln)
    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn collect(&mut self) {
        self.collected = true;
        drop(self);
    }
}

//I think this is how we'll delete the coin
impl Drop for Coin<'_> {
    fn drop(&mut self) {
        println!("dropping coin");
    }
}

pub struct Power<'a> {
    pub pos: Rect,
    texture: Texture<'a>,
    pub collected: bool,
}

impl<'a> Power<'a> {
    pub fn new(pos: Rect, texture: Texture<'a>) -> Power {
        Power {
            pos,
            texture,
            collected: false,
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.x()
    }

    pub fn y(&self) -> i32 {
        self.pos.y()
    }

    fn update_pos(&mut self, x: i32, y: i32) {
        self.pos.set_x(x);
        self.pos.set_y(y);
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn collected(&self) -> bool {
        self.collected
    }
    //if we delete Power by dropping them from mem (once collected)
    pub fn drop(&mut self) {}
}

impl<'a> Collectible<'a> for Power<'a> {
    //for now (honestly not a horrible long term soln)
    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.pos.width(), self.pos.height())
    }

    fn collect(&mut self) {
        self.collected = true;
        drop(self);
    }
}

//I think this is how we'll delete the Power
impl Drop for Power<'_> {
    fn drop(&mut self) {
        println!("dropping Power");
    }
}
