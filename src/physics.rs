// use crate::rect;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::rect;
use crate::runner::TILE_SIZE as InitTILE_SIZE;
use std::any::Any;
use std::f64::consts::PI;
use std::num;

const LOWER_SPEED: f64 = -5.0;
const UPPER_SPEED: f64 = 5.0;
const OMEGA: f64 = PI / 18.0;
const TILE_SIZE: f64 = InitTILE_SIZE as f64;

pub struct Physics;

impl Physics {
    // Checks if entities are colliding
    // Params: entityA, entityB
    // Returns: true if entities are colliding, false otherwise
    pub fn check_collision<'a>(
        entity_a: &mut impl Entity<'a>,
        entity_b: &mut impl Entity<'a>,
    ) -> bool {
        entity_a.hitbox().has_intersection(entity_b.hitbox())
    }

    // Checks if player hasn't landed on their head
    // Params: player, ground position as SDL point, angle of ground
    // Returns: true if player is upright, false otherwise
    pub fn check_player_upright<'a>(player: &Player, angle: f64, ground: Point) -> bool {
        !player.hitbox().contains_point(ground)
            || (player.theta() < OMEGA * 6.0 + angle
                || player.theta() > 2.0 * PI - OMEGA * 6.0 + angle)
    }

    // Applies terrain forces to a body, i.e. gravity, normal, and friction forces
    // Params: body, angle of ground, ground position as SDL Point, coeff of kinetic friction
    // Returns: none
    pub fn apply_terrain_forces<'a>(
        body: &mut impl Body<'a>,
        angle: f64,
        ground: Point,
        coeff: f64,
        power_up: Option<PowerType>,
    ) {
        // Acceleration of gravity
        let mut g: f64 = 1.0;
        if let Some(PowerType::LowerGravity) = power_up {
            // Lower gravity if power is low gravity
            g = 2.0 / 3.0;
        }

        // Gravity: mg
        body.apply_force((0.0, -body.mass() * g));

        /*
            Note on angles:
            - Negative angle == uphill
            - Positive angle == downhill
            - sin(-x) is negative
            - cos(-x) is positive
        */

        // If body is on ground, apply normal
        if body.hitbox().contains_point(ground) {
            // Land on ground
            if body.vel_y() < 0.0 {
                body.hard_set_pos((body.x() as f64, ground.y() as f64 - 0.95 * TILE_SIZE));
                body.hard_set_vel((body.vel_x(), 0.0));
                body.align_hitbox_to_pos();
            }

            // Normal: mg, but on an incline
            // (-x, +y) on an uphill
            // (+x, +y) on a downhill
            body.apply_force((body.mass() * g * angle.sin(), body.mass() * g * angle.cos()));

            // If body is on ground AND moving, apply KINETIC FRICTION
            if body.vel_x().abs() + body.vel_y().abs() > 0.0 {
                // Friction: µmg, on an incline, perpendicular to normal
                // (-x, -y) on an uphill
                // (-x, +y) on an downhill
                body.apply_force((
                    -coeff * body.mass() * g * angle.cos(),
                    coeff * body.mass() * g * angle.sin(),
                ));
            }
            // Else if body is on ground and STILL, apply STATIC FRICTION
            else {
                // (+x, +y) on an uphill
                // (-x, +y) on a downhill
                body.apply_force((
                    -angle.signum() * body.mass() * g * angle.cos(),
                    angle.signum() * body.mass() * g * angle.sin(),
                ));
            }
        }
    }

    // Applies forward motion to player, as if they're propelling themselves
    // Serves to oppose and overcome backwards forces (friction and normal)
    // Params: player, angle of ground, ground position is as SDL Point
    // Returns: None
    pub fn apply_skate_force(player: &mut Player, angle: f64, ground: Point) {
        // Skate force
        let mut skate_force = 1.0 / 5.0 * player.mass();
        if let Some(PowerType::SpeedBoost) = player.power_up() {
            // Speed up with powerup
            skate_force = 2.0 / 5.0 * player.mass();
        }

        if player.hitbox().contains_point(ground) {
            // (+x, +y) on an uphill
            // (+x, -y) on a downhill
            player.apply_force((skate_force * angle.cos(), -skate_force * angle.sin()));
        }
    }

    // Applies upward spring force using Hooke's law
    // Dependent on player's position: F = kx
    // Params: player, spring object
    // Returns: none
    pub fn apply_bounce<'a>(player: &mut Player, body: &impl Body<'a>) {
        // Spring force constant
        let k = 1.0;

        // Find how far player has depressed the spring
        let intersection = player.hitbox().intersection(body.hitbox());

        // If the player is really touching the spring, apply the force
        if let Some(overlap) = intersection {
            let displacement = overlap.y() as f64;
            // Force is always upwards
            player.apply_force((0.0, k * displacement));
        }
    }

    // Applies upward buoyant force according to Archimedes Principle
    // Dependent on player's area: F = pgV
    // Params: player, surface position as SDL Point
    pub fn apply_buoyancy(player: &mut Player, surface: Point) {
        // Density
        let p = player.mass() / 4.0;

        // Acceleration of gravity
        let mut g: f64 = 1.0;
        if let Some(PowerType::LowerGravity) = player.power_up() {
            // Lower gravity if power is low gravity
            g = 2.0 / 3.0;
        }

        // Calculate player's 2D-volume beneath water
        let submerged_area = player.hitbox().width() as f64
            * (player.hitbox().y() + player.hitbox().height() as i32 - surface.y()) as f64;

        // If the player really is underwater, apply the force
        if submerged_area > 0.0 {
            // Force is always upwards
            player.apply_force((0.0, p * g * submerged_area));
        }
    }
}

/******************************* TRAITS *******************************/

pub trait Entity<'a> {
    fn texture(&self) -> &Texture<'a>;

    fn x(&self) -> i32 {
        self.hitbox().x()
    }
    fn y(&self) -> i32 {
        self.hitbox().y()
    }
    fn center(&self) -> Point {
        self.hitbox().center()
    }

    fn hitbox(&self) -> Rect;
    fn align_hitbox_to_pos(&mut self);
}

pub trait Body<'a>: Entity<'a> {
    fn mass(&self) -> f64;
    fn rotational_inertia(&self) -> f64 {
        let radius = (self.hitbox().width() as f64) / 2.0;
        self.mass() * radius * radius
    }
    fn update_pos(&mut self, ground: Point, angle: f64, game_over: bool);
    fn hard_set_pos(&mut self, pos: (f64, f64));

    fn vel_x(&self) -> f64;
    fn vel_y(&self) -> f64;
    fn update_vel(&mut self);
    fn hard_set_vel(&mut self, vel: (f64, f64));

    fn accel_x(&self) -> f64;
    fn accel_y(&self) -> f64;
    fn apply_force(&mut self, force: (f64, f64));
    fn reset_accel(&mut self);

    fn theta(&self) -> f64;
    fn rotate(&mut self);

    fn omega(&self) -> f64;
}

pub trait Collectible<'a>: Entity<'a> {
    fn update_pos(&mut self, x: i32, y: i32);
    fn collect(&mut self);
    fn collected(&self) -> bool;
}

/**********************************************************************/

/****************************** PLAYER ********************************/

pub struct Player<'a> {
    pub pos: (f64, f64),
    velocity: (f64, f64),
    accel: (f64, f64),
    pub hitbox: Rect,

    theta: f64, // angle of rotation, in radians
    omega: f64, // angular speed

    mass: f64,
    texture: Texture<'a>,
    power_up: Option<PowerType>,

    jumping: bool,
    flipping: bool,
    second_jump: bool,
}

impl<'a> Player<'a> {
    pub fn new(hitbox: Rect, mass: f64, texture: Texture<'a>) -> Player {
        Player {
            pos: (hitbox.x() as f64, hitbox.y() as f64),
            velocity: (0.0, 0.0),
            accel: (0.0, 0.0),
            hitbox,

            theta: 0.0,
            omega: 0.0,

            texture,
            mass,
            power_up: None,

            jumping: true,
            flipping: false,
            second_jump: false,
        }
    }

    pub fn is_jumping(&self) -> bool {
        self.jumping
    }

    pub fn is_flipping(&self) -> bool {
        self.flipping
    }

    pub fn power_up(&self) -> Option<PowerType> {
        self.power_up
    }

    pub fn set_power_up(&mut self, power_up: Option<PowerType>) {
        self.power_up = power_up;
    }

    // Brings player's rotational velocity to a stop
    pub fn stop_flipping(&mut self) {
        self.flipping = false;
        self.omega = 0.0;
    }

    // Gives player rotational velocity
    pub fn resume_flipping(&mut self) {
        self.flipping = true;
        self.omega = OMEGA;
    }

    // Returns true if a jump was initiated
    pub fn jump(&mut self, ground: Point) -> bool {
        if self.hitbox().contains_point(ground) {
            self.hard_set_pos((self.pos.0, ground.y() as f64 - TILE_SIZE));
            self.align_hitbox_to_pos();
            self.apply_force((0.0, 100.0));
            self.jumping = true;
            true
        } else {
            false
        }
    }

    pub fn flip(&mut self) {
        if self.is_flipping() {
            self.rotate();
        }
    }

    pub fn collide_obstacle(&mut self, obstacle: &mut Obstacle) {}

    pub fn collide_coin(&mut self, obstacle: &mut Obstacle) {}

    pub fn collide_power(&mut self, obstacle: &mut Obstacle) {}
}

impl<'a> Entity<'a> for Player<'a> {
    fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }
}

impl<'a> Body<'a> for Player<'a> {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn update_pos(&mut self, ground: Point, angle: f64, game_over: bool) {
        // TEMPORARY: Player's x position is fixed until camera freezes on game ending
        if game_over {
            self.pos.0 += self.vel_x();
        }
        self.pos.1 -= self.vel_y();

        if self.hitbox.contains_point(ground) {
            self.theta = angle;
            if self.jumping {
                self.jumping = false;
            }
        }

        self.align_hitbox_to_pos();
    }

    fn hard_set_pos(&mut self, pos: (f64, f64)) {
        self.pos.0 = pos.0;
        self.pos.1 = pos.1;
    }

    fn vel_x(&self) -> f64 {
        self.velocity.0
    }

    fn vel_y(&self) -> f64 {
        self.velocity.1
    }

    fn update_vel(&mut self) {
        self.velocity.0 = (self.velocity.0 + self.accel.0).clamp(LOWER_SPEED, UPPER_SPEED);
        self.velocity.1 =
            (self.velocity.1 + self.accel.1).clamp(2.0 * LOWER_SPEED, 5.0 * UPPER_SPEED);
    }

    fn hard_set_vel(&mut self, vel: (f64, f64)) {
        self.velocity.0 = vel.0;
        self.velocity.1 = vel.1;
    }

    fn accel_x(&self) -> f64 {
        self.accel.0
    }

    fn accel_y(&self) -> f64 {
        self.accel.1
    }

    fn apply_force(&mut self, force: (f64, f64)) {
        self.accel.0 += force.0 / self.mass();
        self.accel.1 += force.1 / self.mass();
    }

    fn reset_accel(&mut self) {
        self.accel = (0.0, 0.0);
    }

    fn theta(&self) -> f64 {
        self.theta
    }

    fn rotate(&mut self) {
        self.theta = (self.theta - self.omega() + 2.0 * PI) % (2.0 * PI);
    }

    fn omega(&self) -> f64 {
        self.omega
    }
}

/**********************************************************************/

/*************************** OBSTACLE *********************************/

pub struct Obstacle<'a> {
    pub pos: (f64, f64),
    velocity: (f64, f64),
    accel: (f64, f64),
    pub hitbox: Rect,

    mass: f64,
    texture: Texture<'a>,
    obstacle_type: ObstacleType,

    theta: f64,
    omega: f64,

    pub collided: bool,
    pub spawned: bool,
    pub delete_me: bool,
}

#[derive(Copy, Clone)]
pub enum ObstacleType {
    Statue,
    Spring,
    Box,
}

impl<'a> Obstacle<'a> {
    pub fn new(
        hitbox: Rect,
        mass: f64,
        texture: Texture<'a>,
        obstacle_type: ObstacleType,
    ) -> Obstacle {
        Obstacle {
            pos: (hitbox.x() as f64, hitbox.y() as f64),
            velocity: (0.0, 0.0),
            accel: (0.0, 0.0),
            hitbox,

            mass,
            texture,
            obstacle_type,

            theta: 0.0,
            omega: 0.0,

            collided: false,
            spawned: false,
            delete_me: false,
        }
    }

    pub fn obstacle_type(&self) -> ObstacleType {
        self.obstacle_type
    }

    pub fn collided(&self) -> bool {
        self.collided
    }
}

impl<'a> Entity<'a> for Obstacle<'a> {
    fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }
}

impl<'a> Body<'a> for Obstacle<'a> {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn update_pos(&mut self, ground: Point, angle: f64, game_over: bool) {
        if self.hitbox.contains_point(ground) {
            self.theta = angle;
        }

        self.pos.0 += self.vel_x();
        self.pos.1 -= self.vel_y();
        self.align_hitbox_to_pos();
    }

    fn hard_set_pos(&mut self, pos: (f64, f64)) {
        self.pos.0 = pos.0;
        self.pos.1 = pos.1;
    }

    fn vel_x(&self) -> f64 {
        self.velocity.0
    }

    fn vel_y(&self) -> f64 {
        self.velocity.1
    }

    fn update_vel(&mut self) {
        self.velocity.0 = (self.velocity.0 + self.accel.0).clamp(-20.0, 20.0);
        self.velocity.1 = (self.velocity.1 + self.accel.1).clamp(-20.0, 20.0);
    }

    fn hard_set_vel(&mut self, vel: (f64, f64)) {
        self.velocity.0 = vel.0;
        self.velocity.1 = vel.1;
    }

    fn accel_x(&self) -> f64 {
        self.accel.0
    }

    fn accel_y(&self) -> f64 {
        self.accel.1
    }

    fn apply_force(&mut self, force: (f64, f64)) {
        self.accel.0 += force.0 / self.mass();
        self.accel.1 += force.1 / self.mass();
    }

    fn reset_accel(&mut self) {
        self.accel = (0.0, 0.0);
    }

    fn theta(&self) -> f64 {
        self.theta
    }

    fn rotate(&mut self) {
        self.theta = (self.theta - self.omega() + 2.0 * PI) % (2.0 * PI);
    }

    fn omega(&self) -> f64 {
        self.omega
    }
}

/**********************************************************************/

/**************************** COIN ************************************/

pub struct Coin<'a> {
    pub pos: (i32, i32),
    pub hitbox: Rect,
    texture: Texture<'a>,
    value: i32,
    collected: bool,
}

impl<'a> Coin<'a> {
    pub fn new(hitbox: Rect, texture: Texture<'a>, value: i32) -> Coin {
        Coin {
            pos: (hitbox.x(), hitbox.y()),
            texture,
            hitbox,
            value,
            collected: false,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl<'a> Entity<'a> for Coin<'a> {
    fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0);
        self.hitbox.set_y(self.pos.1);
    }
}

impl<'a> Collectible<'a> for Coin<'a> {
    fn update_pos(&mut self, x: i32, y: i32) {
        self.pos.0 = x;
        self.pos.1 = y;
    }

    fn collect(&mut self) {
        self.collected = true;
    }

    fn collected(&self) -> bool {
        self.collected
    }
}

/**********************************************************************/

/*************************** POWER ************************************/

pub struct Power<'a> {
    pub pos: (i32, i32),
    pub hitbox: Rect,
    texture: Texture<'a>,
    power_type: PowerType,
    collected: bool,
}

#[derive(Copy, Clone)]
pub enum PowerType {
    SpeedBoost,
    ScoreMultiplier,
    BouncyShoes,
    LowerGravity,
    Shield,
}

impl<'a> Power<'a> {
    pub fn new(hitbox: Rect, texture: Texture<'a>, power_type: PowerType) -> Power {
        Power {
            pos: (hitbox.x(), hitbox.y()),
            hitbox,
            texture,
            power_type,
            collected: false,
        }
    }

    pub fn power_type(&self) -> PowerType {
        self.power_type
    }
}

impl<'a> Entity<'a> for Power<'a> {
    fn texture(&self) -> &Texture<'a> {
        &self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }
}

impl<'a> Collectible<'a> for Power<'a> {
    fn update_pos(&mut self, x: i32, y: i32) {
        self.pos.0 = x;
        self.pos.1 = y;
    }

    fn collect(&mut self) {
        self.collected = true;
    }

    fn collected(&self) -> bool {
        self.collected
    }
}
