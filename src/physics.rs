use inf_runner::ObstacleType;
use inf_runner::PowerType;
use inf_runner::TerrainType;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use std::time::{Duration, SystemTime};

use crate::runner::TILE_SIZE as InitTILE_SIZE;
use std::f64::consts::PI;

const LOWER_SPEED: f64 = -5.0;
const UPPER_SPEED: f64 = 12.5;
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
    pub fn check_player_upright<'a>(player: &mut Player, angle: f64, ground: Point) -> bool {
        let on_ground = player.hitbox().contains_point(ground);
        if on_ground {
            player.was_flipping = false;
        }
        !on_ground
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
        terrain_type: &TerrainType,
        power_up: Option<PowerType>,
    ) {
        // Set Gravity & Friction Strength From TerrainType
        let mut fric_coeff: f64;
        let mut g: f64 = 1.25;
        //As of now, all conds lead to +accel on flat ground (we could change this)
        match terrain_type {
            TerrainType::Asphalt => {
                //quick accel to max on flat
                fric_coeff = 0.035;
            }
            TerrainType::Grass => {
                //moderate accel to max on flat
                fric_coeff = 0.065;
            }
            TerrainType::Sand => {
                //v slow accel to max on flat & short jumps
                fric_coeff = 0.1; //less friction is more bc higher gravity
                g = 1.5;
            }
            TerrainType::Water => {
                //NOT YET CONFIGURED
                fric_coeff = 0.0;
            }
        }

        // Lower gravity if power is low gravity
        if let Some(PowerType::LowerGravity) = power_up {
            g = g * 2.0 / 3.0;
        }

        // Gravity: mg
        body.apply_force((0.0, -body.mass() * g));

        if let TerrainType::Water = terrain_type {
            return;
        } else {
            /*
                Note on angles:
                - Negative angle == uphill
                - Positive angle == downhill
                - sin(-x) is negative
                - cos(-x) is positive
            */

            // If body is on ground, apply normal
            let pre_forces_direction = (body.vel_x() + body.accel_x()).signum();
            let height = body.hitbox().height() as i32;
            if body.hitbox().y() + height > ground.y() {
                // Land on ground
                if body.vel_y() < 0.0
                    || (body.y() as f64 + 0.95 * (height as f64)) > ground.y() as f64
                {
                    body.hard_set_pos((
                        body.x() as f64,
                        ground.y() as f64 - 0.95 * (height as f64),
                    ));
                    body.hard_set_vel((body.vel_x(), -0.01));
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
                    // make negative if object is moving backwards
                    let direction_adjust = body.vel_x().signum();
                    body.apply_force((
                        -fric_coeff * body.mass() * g * angle.cos() * direction_adjust,
                        fric_coeff * body.mass() * g * angle.sin() * direction_adjust,
                    ));
                }
                let post_forces_direction = (body.vel_x() + body.accel_x()).signum();

                if pre_forces_direction != post_forces_direction {
                    body.hard_set_vel((0.0, 0.0));
                    body.reset_accel();
                }
                // Else if body is on ground and STILL, apply STATIC FRICTION
                // NOTE: This might be unnecessary
                // else {
                //     // (+x, +y) on an uphill
                //     // (-x, +y) on a downhill
                //     body.apply_force((
                //         -angle.signum() * body.mass() * g * angle.cos(),
                //         angle.signum() * body.mass() * g * angle.sin(),
                //     ));
                // }
            }
        }
    }

    // Applies forward motion to player, as if they're propelling themselves
    // Serves to oppose and overcome backwards forces (friction and normal)
    // Params: player, angle of ground, ground position is as SDL Point
    // Returns: None
    pub fn apply_skate_force(player: &mut Player, angle: f64, ground: Point) {
        // Skate force
        let mut skate_force = 1.0 / 7.0 * player.mass();

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
        let k = 0.2;

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
        let p = player.mass() / 5000.0;

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

            player.theta = player.theta()
                - 0.05
                    * player.theta()
                    * (submerged_area
                        / (player.hitbox().width() * player.hitbox().height()) as f64);
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
    fn align_hitbox_to_pos(&mut self); // After the pos is set with f64s, this method moves hitbox
                                       // to proper SDL coordinates using i32s

    // Adjusts terrain postion in runner.rs based on camera_adj_x & camera_adj_y
    fn camera_adj(&mut self, x_adj: i32, y_adj: i32);
}

pub trait Body<'a>: Entity<'a> {
    fn mass(&self) -> f64;
    fn rotational_inertia(&self) -> f64 {
        let radius = (self.hitbox().width() as f64) / 2.0;
        self.mass() * radius * radius
    }
    fn update_pos(&mut self, ground: Point, angle: f64, game_over: bool);
    fn hard_set_pos(&mut self, pos: (f64, f64)); // Official method to hardcode position

    fn vel_x(&self) -> f64;
    fn vel_y(&self) -> f64;
    fn update_vel(&mut self, game_over: bool);
    fn hard_set_vel(&mut self, vel: (f64, f64)); // Official method to hardcode velocity

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

// Holds the player's current power and texture associated
pub struct PlayerPower<'a> {
    power_up: Option<PowerType>,
    texture: &'a Texture<'a>,
}

impl<'a> PlayerPower<'a> {
    pub fn new(power_up: Option<PowerType>, tex: &'a Texture<'a>) -> PlayerPower<'a> {
        PlayerPower {
            power_up: power_up,
            texture: tex,
        }
    }

    // Return set power-up or None if set to None
    fn power_up(&self) -> Option<PowerType> {
        self.power_up
    }

    // Return texture associated.
    fn texture(&self) -> &Texture<'a> {
        self.texture
    }
}

pub struct Player<'a> {
    pub pos: (f64, f64),
    velocity: (f64, f64),
    accel: (f64, f64),
    hitbox: Rect,

    theta: f64, // angle of rotation, in radians
    omega: f64, // angular speed

    mass: f64,
    texture: &'a Texture<'a>,
    power_up_stuct: PlayerPower<'a>,

    jump_time: SystemTime,
    lock_jump_time: bool,
    jumping: bool,
    flipping: bool,
    was_flipping: bool,
}

impl<'a> Player<'a> {
    pub fn new(hitbox: Rect, mass: f64, texture: &'a Texture<'a>) -> Player<'a> {
        let empty_player = PlayerPower::new(None, texture); // Inital texture doesn't matter
        Player {
            pos: (hitbox.x() as f64, hitbox.y() as f64),
            velocity: (0.0, 0.0),
            accel: (0.0, 0.0),
            hitbox,

            theta: 0.0,
            omega: 0.0,

            texture,
            mass,
            power_up_stuct: empty_player,

            jump_time: SystemTime::now(),
            lock_jump_time: false,
            jumping: true,
            flipping: false,
            was_flipping: false,
        }
    }

    pub fn is_jumping(&self) -> bool {
        self.jumping
    }

    pub fn jumpmoment_lock(&self) -> bool {
        self.lock_jump_time
    }

    pub fn is_flipping(&self) -> bool {
        self.flipping
    }

    pub fn was_flipping(&self) -> bool {
        self.was_flipping
    }

    // Returns specific power-up player has, or None if player hasn't collected a power-up
    pub fn power_up(&self) -> Option<PowerType> {
        self.power_up_stuct.power_up
    }

    // Returns specific power-up texture. Should only be called after a check if player has a power-up
    pub fn power_up_tex(&self) -> &Texture<'a> {
        self.power_up_stuct.texture
    }

    // Setter for power-up
    pub fn set_power_up(&mut self, power_up: Option<PowerType>, texture: &'a Texture<'a>) {
        let power_struct = PlayerPower::new(power_up, texture);
        self.power_up_stuct = power_struct;
    }

    // Brings player's rotational velocity to a stop
    pub fn stop_flipping(&mut self) {
        self.flipping = false;
    }

    // Gives player rotational velocity
    pub fn resume_flipping(&mut self) {
        self.flipping = true;
        self.was_flipping = true;
        self.omega = OMEGA;
    }

    pub fn set_jumpmoment(&mut self, time: SystemTime) {
        self.jump_time = time;
        self.lock_jump_time = true;
    }

    pub fn jump_moment(&mut self) -> SystemTime {
        self.jump_time
    }

    // Returns true if a jump was initiated
    pub fn jump(&mut self, ground: Point) -> bool {
        let height = self.hitbox.height() as f64;
        // Jump if on ground (with some give as to what "on ground" means)
        if self.hitbox().y() + (1.05 * height) as i32 > ground.y() {
            // Starting from the position of the ground
            self.hard_set_pos((self.pos.0, ground.y() as f64 - height));
            self.align_hitbox_to_pos();
            // Apply upward force
            self.apply_force((0.0, 80.0));
            self.jumping = true;
            true
        } else {
            false
        }
    }

    pub fn flip(&mut self, angle: f64) -> bool {
        if self.is_flipping() {
            self.rotate();
            //Player rotated enough to die, so let's call it a flip?
            if (self.theta() > OMEGA * 6.0 + angle
            || self.theta() < 2.0 * PI - OMEGA * 6.0 + angle) {
                true
            }
            else{
                false
            }
        } else if self.was_flipping() {
            //allows for momentum when player stops flipping
            //to adjust rate of angular velocity decrease,
            //change the value being subtracted from omega
            if (self.omega - (0.03 * self.omega)) != 0.0 {
                self.omega = self.omega - (0.03 * self.omega);
            } else {
                self.omega = 0.0;
            }
            self.rotate();
            false
        }else{
            false
        }
    }

    // Handles collisions with player and any type of obstacle
    // Params: obstacle to collide with
    // Returns: true if real game-ending collision occurs, false otherwise
    pub fn collide_obstacle(&mut self, obstacle: &mut Obstacle) -> bool {
        let mut shielded = false;
        if let Some(PowerType::Shield) = self.power_up() {
            // Put on shield if applicable
            shielded = true;
        }

        // if the collision box is taller than it is wide, the player hit the side of the object
        if (self
            .hitbox()
            .intersection(obstacle.hitbox())
            .unwrap()
            .height()
            > self
                .hitbox()
                .intersection(obstacle.hitbox())
                .unwrap()
                .width())
            && self.hitbox.x() < obstacle.hitbox.x()
        {
            // Response to collision dependent on type of obstacle
            match obstacle.obstacle_type {
                // For statue and chest, elastic collision
                ObstacleType::Statue | ObstacleType::Chest | ObstacleType::Bench => {
                    if obstacle.collided() {
                        // If collision already happened, pretend nothing happened
                        false
                    } else {
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
                        let p_vy = if self.jumping { self.velocity.1 } else { 0.0 };
                        let p_vx_f = 2.0 * (p_mass - o_mass) * (p_vx) / (p_mass + o_mass);
                        let p_vy_f = 2.0 * (p_mass - o_mass) * (p_vy) / (p_mass + o_mass);
                        let o_vx_f = 2.0 * (2.0 * p_mass) * (p_vx) / (p_mass + o_mass);
                        let o_vy_f = 2.0 * (2.0 * p_mass) * (p_vy) / (p_mass + o_mass);

                        // CALCULATE PLAYER AND OBJECT NEW OMEGAS HERE
                        // Torque = r*F * sin(angle)
                        // alpha = Torque/body.rotational_inertia()
                        // For ease of calculation, just set omega = alpha
                        /*
                        //Not certain if this math is correct
                        let force = self.mass() * ((self.velocity.0*self.velocity.0) + (self.velocity.1*self.velocity.1)).sqrt();
                        let torque = ((self.hitbox().width() as f64) / 2.0)  *  angle.sin();
                        let alpha = torque / self.rotational_inertia();             //rot inertia is 7500
                        self.omega = alpha;
                        //println!("t:{} a:{} f:{} sin:{} rot:{}", torque, alpha,force,angle.sin(),self.rotational_inertia());
                        */

                        /***************************************************/
                        
                        // Move obstacle
                        obstacle.collided = true;
                        obstacle.hard_set_vel((o_vx_f, o_vy_f));

                        if shielded{    // Don't move player
                            false       // Game not over
                        }
                        else{
                            // Move player
                            self.hard_set_vel((p_vx_f, p_vy_f));
                            self.hard_set_pos((
                            obstacle.x() as f64 - 1.05 * TILE_SIZE,
                            self.y() as f64,
                            ));
                            self.align_hitbox_to_pos();
                            true        // game over
                        }
                    }
                }
                // For Balloon, do nothing upon SIDE collision
                ObstacleType::Balloon => false
            }
        }
        // if the collision box is wider than it is tall, the player hit the top of the object
        // don't apply the collision to the top of an object if the player is moving upward, otherwise they will "stick" to the top on the way up
        else if self.is_jumping() && self.vel_y() < 0.0 {
            match obstacle.obstacle_type {
                // On top collision with chest, treat the chest as if it's normal ground
                ObstacleType::Chest | ObstacleType::Bench => {

                    if !obstacle.collided() {
                      self.pos.1 = (obstacle.y() as f64 - 0.95 * (TILE_SIZE as f64));
                      self.align_hitbox_to_pos();
                      self.velocity.1 = 0.0;
                      self.jumping = false;
                      self.lock_jump_time = false;
                      self.apply_force((0.0, self.mass()));
                      self.omega = 0.0;
                      obstacle.collided = true;
                      obstacle.collected = true;

                        if self.theta() < OMEGA * 6.0 || self.theta() > 360.0 - OMEGA * 6.0 {
                            self.theta = 0.0;
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
                // For irregularly shaped statue, player gets hurt and game over
                ObstacleType::Statue => {
                    // bounce for fun
                    obstacle.collided = true;
                    Physics::apply_bounce(self, obstacle);
                    !shielded
                }
                // For spring, bounce off with Hooke's law force
                ObstacleType::Balloon => {
                    Physics::apply_bounce(self, obstacle);
                    obstacle.collected = true;
                    false
                }
            }
        } else {
            false
        }
    }

    // Collects a coin
    // Params: coin to collect
    // Returns: true if coin has been collected, false otherwise (e.g. if it's been collected already)
    pub fn collide_coin(&mut self, coin: &mut Coin) -> bool {
        if !coin.collected() {
            coin.collect();
            true
        } else {
            false
        }
    }

    // Receives new power-up
    // Params: power to use
    // Returns:
    pub fn collide_power(&mut self, power: &mut Power, texture: &'a Texture<'a>) -> bool {
        if !power.collected() {
            self.set_power_up(Some(power.power_type()), texture);
            power.collect();
            true
        } else {
            false
        }
    }
}

impl<'a> Entity<'a> for Player<'a> {
    fn texture(&self) -> &Texture<'a> {
        self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }

    // Adjusts terrain postion in runner.rs based on camera_adj_x & camera_adj_y
    fn camera_adj(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.0 += (x_adj as f64);
        self.pos.1 += (y_adj as f64);

        self.align_hitbox_to_pos();
    }
}

impl<'a> Body<'a> for Player<'a> {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn update_pos(&mut self, ground: Point, angle: f64, on_water: bool) {
        self.pos.1 -= self.vel_y();

        // Match the angle of the ground if on ground
        if self.hitbox().contains_point(ground) && !on_water {
            self.theta = angle;
            if self.jumping {
                self.jumping = false;
                self.lock_jump_time = false;
                self.was_flipping = false;
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

    fn update_vel(&mut self, game_over: bool) {
        let mut upper_x_speed = UPPER_SPEED;
        if let Some(PowerType::SpeedBoost) = self.power_up() {
            upper_x_speed *= 2.0;
        }
        if game_over {
            self.velocity.0 = (self.velocity.0 + self.accel.0).clamp(-upper_x_speed, upper_x_speed);
        } else {
            self.velocity.0 = (self.velocity.0 + self.accel.0).clamp(6.5, upper_x_speed);
        }

        self.velocity.1 =
            (self.velocity.1 + self.accel.1).clamp(3.0 * LOWER_SPEED, 4.0 * UPPER_SPEED);
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
    hitbox: Rect,

    mass: f64,
    pub value: i32,
    texture: &'a Texture<'a>,
    obstacle_type: ObstacleType,

    theta: f64,
    omega: f64,

    pub collided: bool,
    pub collected: bool,
    pub spawned: bool,
    pub delete_me: bool,
}

impl<'a> Obstacle<'a> {
    pub fn new(
        hitbox: Rect,
        mass: f64,
        value: i32,
        texture: &'a Texture<'a>,
        obstacle_type: ObstacleType,
    ) -> Obstacle<'a> {
        Obstacle {
            pos: (hitbox.x() as f64, hitbox.y() as f64),
            velocity: (0.0, 0.0),
            accel: (0.0, 0.0),
            hitbox,

            mass,
            value,
            texture,
            obstacle_type,

            theta: 0.0,
            omega: 0.0,

            collided: false,
            collected: false,
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

    pub fn collected(&self) -> bool {
        self.collected
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    // Shifts objects left with the terrain in runner.rs
    pub fn travel_update(&mut self, travel_adj: i32) {
        self.pos.0 -= (travel_adj as f64);
    }
}

impl<'a> Entity<'a> for Obstacle<'a> {
    fn texture(&self) -> &Texture<'a> {
        self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }

    // Adjusts terrain postion in runner.rs based on camera_adj_x & camera_adj_y
    fn camera_adj(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.0 += (x_adj as f64);
        self.pos.1 += (y_adj as f64);

        self.align_hitbox_to_pos();
    }
}

impl<'a> Body<'a> for Obstacle<'a> {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn update_pos(&mut self, ground: Point, angle: f64, game_over: bool) {
        if self.hitbox.contains_point(ground) && !game_over {
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

    fn update_vel(&mut self, game_over: bool) {
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
    hitbox: Rect,
    texture: &'a Texture<'a>,
    value: i32,
    collected: bool,
}

impl<'a> Coin<'a> {
    pub fn new(hitbox: Rect, texture: &'a Texture<'a>, value: i32) -> Coin<'a> {
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

    // Shifts objects left with the terrain in runner.rs
    pub fn travel_update(&mut self, travel_adj: i32) {
        self.pos.0 -= travel_adj;
    }
}

impl<'a> Entity<'a> for Coin<'a> {
    fn texture(&self) -> &Texture<'a> {
        self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0);
        self.hitbox.set_y(self.pos.1);
    }

    // Adjusts terrain postion in runner.rs based on camera_adj_x & camera_adj_y
    fn camera_adj(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.0 += x_adj;
        self.pos.1 += y_adj;

        self.align_hitbox_to_pos();
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
    hitbox: Rect,
    texture: &'a Texture<'a>,
    power_type: PowerType,
    collected: bool,
}

impl<'a> Power<'a> {
    pub fn new(hitbox: Rect, texture: &'a Texture<'a>, power_type: PowerType) -> Power<'a> {
        Power {
            pos: (hitbox.x(), hitbox.y()),
            hitbox,
            texture,
            collected: false,
            power_type,
        }
    }

    pub fn power_type(&self) -> PowerType {
        self.power_type
    }

    // Shifts objects left with the terrain in runner.rs
    pub fn travel_update(&mut self, travel_adj: i32) {
        self.pos.0 -= travel_adj;
    }
}

impl<'a> Entity<'a> for Power<'a> {
    fn texture(&self) -> &Texture<'a> {
        self.texture
    }

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn align_hitbox_to_pos(&mut self) {
        self.hitbox.set_x(self.pos.0 as i32);
        self.hitbox.set_y(self.pos.1 as i32);
    }

    // Adjusts terrain postion in runner.rs based on camera_adj_x & camera_adj_y
    fn camera_adj(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.0 += x_adj;
        self.pos.1 += y_adj;

        self.align_hitbox_to_pos();
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
