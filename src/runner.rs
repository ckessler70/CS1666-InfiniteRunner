// use crate::physics::Body;
use crate::physics::Physics;
// use crate::physics::Collider;
use crate::physics::Coin;
use crate::physics::Collectible;
use crate::physics::Collider;
use crate::physics::Dynamic;
use crate::physics::Entity;
use crate::physics::Obstacle;
use crate::physics::Player;
use crate::physics::Power;
use crate::physics::ObstacleType;

use crate::proceduralgen;
// use crate::proceduralgen::ProceduralGen;
// use crate::proceduralgen::TerrainSegment;

use crate::powers;

use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::SDLCore;
use proceduralgen::StaticObject;

use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
// use sdl2::render::Texture;
use sdl2::render::TextureQuery;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

const FPS: f64 = 60.0;
const FRAME_TIME: f64 = 1.0 / FPS as f64;

const CAM_H: u32 = 720;
const CAM_W: u32 = 1280;

pub const TILE_SIZE: u32 = 100;

// Ensure that SIZE is not a decimal
// 1, 2, 4, 5, 8, 10, 16, 20, 32, 40, 64, 80, 128, 160, 256, 320, 640
const SIZE: usize = CAM_W as usize / 10;
const BUFF_LENGTH: usize = CAM_W as usize / 4;

const FRONT_HILL_INDEX: usize = 0;
const BACK_HILL_INDEX: usize = 1;
const GROUND_INDEX: usize = 2;

// Bounds we want to keep the player within
const player_upper_bound: i32 = 2 * TILE_SIZE as i32;
const player_lower_bound: i32 = CAM_H as i32 - player_upper_bound;

pub struct Runner;

impl Game for Runner {
    fn init() -> Result<Self, String> {
        Ok(Runner {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = core.wincan.texture_creator();
        let tex_bg = texture_creator.load_texture("assets/bg.png")?;
        let tex_sky = texture_creator.load_texture("assets/sky.png")?;
        let tex_grad = texture_creator.load_texture("assets/sunset_gradient.png")?;
        let tex_statue = texture_creator.load_texture("assets/statue.png")?;
        let tex_coin = texture_creator.load_texture("assets/coin.gif")?;

        let tex_speed = texture_creator.load_texture("assets/speed.png")?;
        let tex_multiplier = texture_creator.load_texture("assets/multiplier.png")?;
        let tex_bouncy = texture_creator.load_texture("assets/bouncy.png")?;
        let tex_floaty = texture_creator.load_texture("assets/floaty.png")?;
        let tex_shield = texture_creator.load_texture("assets/shield.png")?;

        let mut bg_buff = 0;

        // Create player at default position
        let mut player = Player::new(
            rect!(
                CAM_W / 2 - TILE_SIZE / 2, // Center of screen
                CAM_H / 2 - TILE_SIZE / 2,
                TILE_SIZE,
                TILE_SIZE
            ),
            3.0,
            texture_creator.load_texture("assets/player.png")?,
        );

        //empty obstacle & coin vectors
        let mut obstacles: Vec<_> = Vec::new();
        let mut coins: Vec<_> = Vec::new();
        let mut powers: Vec<_> = Vec::new();

        // Used to keep track of animation status
        let src_x: i32 = 0;

        let mut score: i32 = 0;
        let mut tick_score: i32;
        let mut coin_count: i32 = 0;

        let mut game_paused: bool = false;
        let mut initial_pause: bool = false;
        let mut game_over: bool = false;
        let mut power_override: bool = false;

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        let mut next_status = GameStatus::Main;

        let mut ct: usize;
        let mut tick = 0;
        let mut power_tick: i32 = 0;
        let mut buff_1: usize = 0;
        let mut buff_2: usize = 0;
        let mut object_spawn: usize = 0;
        let mut object_count: i32 = 0;

        let mut object = None;

        let mut power: Option<powers::PowerUps> = None;
        let mut next_power: Option<powers::PowerUps> = None;

        let mut player_accel_rate: f64 = -10.0;
        let mut player_jump_change: f64 = 0.0;
        let mut player_speed_adjust: f64 = 0.0;

        // bg[0] = Front hills
        // bg[1] = Back hills
        // bg[2] = Ground
        let mut bg: [[i16; SIZE]; 3] = [[0; SIZE]; 3];

        let mut ground_buffer: [(f64, f64); BUFF_LENGTH + 1] = [(0.0, 0.0); BUFF_LENGTH + 1];
        let mut buff_idx = 0;

        let mut rng = rand::thread_rng();

        let freq: f32 = rng.gen::<f32>() * 1000.0 + 100.0;

        let amp_1: f32 = rng.gen::<f32>() * 4.0 + 1.0;
        let amp_2: f32 = rng.gen::<f32>() * 2.0 + amp_1;
        let amp_3: f32 = rng.gen::<f32>() * 2.0 + 1.0;

        // Perlin Noise init
        let mut random: [[(i32, i32); 256]; 256] = [[(0, 0); 256]; 256];

        for i in 0..random.len() - 1 {
            for j in 0..random.len() - 1 {
                random[i][j] = (rng.gen_range(0..256), rng.gen_range(0..256));
            }
        }

        ct = 0;
        let p0 = (0.0, (CAM_H / 3) as f64);
        ground_buffer = proceduralgen::ProceduralGen::gen_bezier_land(
            &random,
            p0,
            CAM_W as i32,
            CAM_H as i32,
            false,
            false,
            false,
        );

        while ct < SIZE as usize {
            bg[FRONT_HILL_INDEX][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_1), freq, amp_1, 0.5, 600.0);
            bg[BACK_HILL_INDEX][ct] =
                proceduralgen::gen_perlin_hill_point((ct + buff_2), freq, amp_2, 1.0, 820.0);
            bg[GROUND_INDEX][ct] = ground_buffer[buff_idx].1 as i16;
            ct += 1;
            buff_idx += 1;
        }

        'gameloop: loop {
            // FPS tracking
            last_raw_time = Instant::now();

            if game_paused {
                // Game paused handler
                for event in core.event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Q),
                            ..
                        } => {
                            next_status = GameStatus::Credits;
                            break 'gameloop;
                        }
                        Event::KeyDown {
                            keycode: Some(k), ..
                        } => match k {
                            Keycode::Escape | Keycode::Space => {
                                game_paused = false;
                            }
                            Keycode::R => {
                                next_status = GameStatus::Game;
                                break 'gameloop;
                            }
                            Keycode::M => {
                                next_status = GameStatus::Main;
                                break 'gameloop;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                // Draw it to screen once and then wait due to BlendMode
                if initial_pause {
                    let resume_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("Escape/Space - Resume Play")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let restart_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("R - Restart game")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let main_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("M - Main menu")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let quit_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("Q - Quit game")
                                .blended(Color::RGBA(119, 3, 252, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    // Grey out screen
                    core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 128));
                    core.wincan.fill_rect(rect!(0, 0, CAM_W, CAM_H))?;

                    // Draw text
                    core.wincan
                        .copy(&resume_texture, None, Some(rect!(100, 100, 1000, 125)))?;
                    core.wincan
                        .copy(&restart_texture, None, Some(rect!(100, 250, 700, 125)))?;
                    core.wincan
                        .copy(&main_texture, None, Some(rect!(100, 400, 600, 125)))?;
                    core.wincan
                        .copy(&quit_texture, None, Some(rect!(100, 550, 600, 125)))?;

                    core.wincan.present();

                    initial_pause = false;
                }
            } else if game_over {
                if initial_pause {
                    ct = 0;
                    let game_over_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("GAME OVER")
                                .blended(Color::RGBA(255, 0, 0, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    let TextureQuery { width, height, .. } = game_over_texture.query();

                    let padding = 64;

                    let wr = width as f32 / (CAM_W - padding) as f32;
                    let hr = height as f32 / (CAM_H - padding) as f32;

                    let (w, h) = if wr > 1f32 || hr > 1f32 {
                        if wr > hr {
                            let h = (height as f32 / wr) as i32;
                            ((CAM_W - padding) as i32, h)
                        } else {
                            let w = (width as f32 / hr) as i32;
                            (w, (CAM_H - padding) as i32)
                        }
                    } else {
                        (width as i32, height as i32)
                    };

                    let cx = (CAM_W as i32 - w) / 2;
                    let cy = (CAM_H as i32 - h) / 2;

                    core.wincan
                        .copy(&game_over_texture, None, Some(rect!(cx, cy, w, h)))?;

                    core.wincan.present();

                    initial_pause = false;
                }

                ct += 1;
                if ct == 120 {
                    break 'gameloop;
                }
            } else {
                // Left ground position
                let current_ground = Point::new(
                    player.x(),
                    CAM_H as i32
                        - bg[2][(player.x() as usize) / (CAM_W / SIZE as u32) as usize] as i32,
                );
                // Right ground position
                let next_ground = Point::new(
                    player.x() + TILE_SIZE as i32,
                    CAM_H as i32
                        - bg[2][(((player.x() + TILE_SIZE as i32) as usize)
                            / (CAM_W / SIZE as u32) as usize)] as i32,
                );
                // Angle between
                let angle = ((next_ground.y() as f64 - current_ground.y() as f64)
                    / (TILE_SIZE as f64))
                    .atan();

                for event in core.event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'gameloop,
                        Event::KeyDown {
                            keycode: Some(k), ..
                        } => match k {
                            Keycode::W | Keycode::Up | Keycode::Space => {
                                if player.is_jumping() {
                                    player.resume_flipping();
                                } else {
                                    player.jump(current_ground, true, player_jump_change);
                                }
                            }
                            Keycode::Escape => {
                                game_paused = true;
                                initial_pause = true;
                            }
                            _ => {}
                        },
                        Event::KeyUp {
                            keycode: Some(k), ..
                        } => match k {
                            Keycode::W | Keycode::Up | Keycode::Space => {
                                player.stop_flipping();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                tick_score = 1;

                //in the future when obstacles & coins are proc genned we will probs wanna
                //only check for obstacles/coins based on their location relative to players x
                // cord
                //(also: idt this can be a for loop bc it moves the obstacles values?)
                for o in obstacles.iter_mut() {
                    //.filter(|near by obstacles|).collect()
                    if let Some(collision_boxes) = player.check_collision(o) {
                        //Bad way to ignore collision with a shield
                        /*if power_override {
                            // if !player.collide(o, collision_boxes) {
                            //     Apply some simulation/annimation on the obstacle knocking it over
                            // }
                            continue;
                        }*/
                        //Temp option: can add these 2 lines to end game upon obstacle collsions
                        //INVICIBILTY: chane true to power_override (when you dont wanna be invincible)
                        let mut shielded = false;
                        if let Some(powers::PowerUps::Shield) = power {
                            shielded = true;
                        }
                        if !player.collide(o, collision_boxes, shielded) {
                            game_over = true;
                            initial_pause = true;
                            continue 'gameloop;
                        }
                        //println!("ypos{} vyo{} ayo{}  ", o.pos.1, o.velocity.1, o.accel.1 );
                        // o.update_vel(0.0,0.0);   //these args do nothing
                        // o.update_pos(Point::new(0,0), 3.0);  //the 3 makes the obstacle spin
                        // println!("ypos{} vyo{} ayo{}  ", o.pos.1, o.velocity.1, o.accel.1 );
                        //Real Solution: need to actually resolve the collision, should go something like this
                        //player.collide(o);
                        //Physics::apply_gravity(&mut obstacle);    //maybe...
                        //obstacle.update_pos();
                        continue;
                    };
                }

                for c in coins.iter_mut() {
                    //check collection
                    if Physics::check_collection(&mut player, c) {
                        if !c.collected() {
                            //so you only collect each coin once
                            c.collect(); //deletes the coin once collected (but takes too long)
                            coin_count += 1;
                            tick_score += c.value(); //increments the score based on the coins value
                                                     //maybe print next to score: "+ c.value()""
                        }

                        continue;
                    }
                }

                // Roughly the code needed for collecting power objects as it should follow the coin idea closely.
                for p in powers.iter_mut() {
                    if Physics::check_power(&mut player, p) {
                        if !p.collected() {
                            match next_power {
                                Some(powers::PowerUps::SpeedBoost) => {
                                    power = Some(powers::PowerUps::SpeedBoost);
                                }
                                Some(powers::PowerUps::ScoreMultiplier) => {
                                    power = Some(powers::PowerUps::ScoreMultiplier);
                                }
                                Some(powers::PowerUps::BouncyShoes) => {
                                    power = Some(powers::PowerUps::BouncyShoes);
                                }
                                Some(powers::PowerUps::LowerGravity) => {
                                    power = Some(powers::PowerUps::LowerGravity);
                                }
                                Some(powers::PowerUps::Shield) => {
                                    power = Some(powers::PowerUps::Shield);
                                }
                                _ => {}
                            }

                            // Reset any previously active power values to default
                            power_override = false;
                            player_accel_rate = -10.0;
                            player_jump_change = 0.0;
                            player_speed_adjust = 0.0;

                            p.collect();
                            power_tick = 360;
                        }
                        continue;
                    }
                }

                //Power handling
                if power_tick > 0 {
                    power_tick -= 1;
                    match power {
                        Some(powers::PowerUps::SpeedBoost) => {
                            // May not be the proper way to handle this.
                            // Adds player speed adjust to player's velocity
                            player_speed_adjust = 5.0;
                        }
                        Some(powers::PowerUps::ScoreMultiplier) => {
                            // Doubles tick score while active
                            tick_score *= 2;
                        }
                        Some(powers::PowerUps::BouncyShoes) => {
                            // Forces jumping while active and jumps 0.3 velocity units higher
                            player_jump_change = 0.3;
                            player.jump(current_ground, true, player_jump_change);
                        }
                        Some(powers::PowerUps::LowerGravity) => {
                            // Accel rate is how the y velocity is clamped
                            // Has player jump 0.2 velocity units higher.
                            player_accel_rate = -5.0;
                            player_jump_change = 0.2;
                        }
                        Some(powers::PowerUps::Shield) => {
                            // Player override will say to ignore obstacle collisions
                            power_override = true;
                        }
                        _ => {}
                    }
                } else if power_tick == 0 {
                    power_tick -= 1;

                    // Reset values to default if power times out
                    match power {
                        // Stop any power from going
                        Some(powers::PowerUps::SpeedBoost) => {
                            player_speed_adjust = 0.0;
                        }
                        Some(powers::PowerUps::ScoreMultiplier) => {}
                        Some(powers::PowerUps::BouncyShoes) => {
                            player_jump_change = 0.0;
                        }
                        Some(powers::PowerUps::LowerGravity) => {
                            player_accel_rate = -10.0;
                            player_jump_change = 0.0;
                        }
                        Some(powers::PowerUps::Shield) => {
                            power_override = false;
                        }
                        _ => {}
                    }

                    power = None;
                }

                //applies gravity, normal & friction now
                //friciton is currently way OP (stronger than grav) bc cast to i32 in apply_force
                //so to ever have an effect, it needs to be set > 1 for now...
                Physics::apply_gravity(&mut player, angle, 0.3);

                //apply friction
                //Physics::apply_friction(&mut player, 1.0);

                for o in obstacles.iter_mut() {
                    o.update_vel(0.0, 0.0); //these args do nothing
                    o.update_pos(Point::new(0, 0), 15.0); //the 3 makes the obstacle spin
                }
                player.update_pos(current_ground, angle);
                player.update_vel(player_accel_rate, player_speed_adjust);
                player.flip();

                //kinematics change, scroll speed does not :(
                //can see best when super curvy map generated
                /*  println!(
                    "px:{}  vx:{} ax:{} ay:{}",
                    player.x(),
                    player.vel_x(),
                    player.accel_x(),
                    player.accel_y(),
                ); */

                if !player.collide_terrain(current_ground, angle) {
                    game_over = true;
                    initial_pause = true;
                    continue;
                }

                core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
                core.wincan.clear();

                core.wincan
                    .copy(&tex_grad, None, rect!(0, -128, CAM_W, CAM_H))?;

                // Every tick, build a new ground segment
                if tick % 1 == 0 {
                    if buff_idx == BUFF_LENGTH {
                        ground_buffer = proceduralgen::ProceduralGen::gen_bezier_land(
                            &random,
                            (0.0, bg[GROUND_INDEX][(SIZE - 1) as usize] as f64),
                            CAM_W as i32,
                            CAM_H as i32,
                            false,
                            false,
                            false,
                        );
                        buff_idx = 0;
                    }

                    for i in 0..(SIZE as usize - 1) {
                        bg[GROUND_INDEX][i] = bg[GROUND_INDEX][i + 1];
                    }

                    bg[GROUND_INDEX][(SIZE - 1) as usize] = ground_buffer[buff_idx].1 as i16;

                    buff_idx += 1;

                    if (ground_buffer[ground_buffer.len() - 1] == (1.0, 1.0)) {
                        println!("Bouncy!");
                    } else {
                        println!("Not Bouncy!");
                    }
                }

                // Every 3 ticks, build a new front mountain segment
                if tick % 3 == 0 {
                    for i in 0..(SIZE as usize - 1) {
                        bg[FRONT_HILL_INDEX][i] = bg[FRONT_HILL_INDEX][i + 1];
                    }
                    buff_1 += 1;
                    let chunk_1 = proceduralgen::gen_perlin_hill_point(
                        ((SIZE - 1) as usize + buff_1),
                        freq,
                        amp_1,
                        0.5,
                        600.0,
                    );
                    bg[FRONT_HILL_INDEX][(SIZE - 1) as usize] = chunk_1;
                }

                // Every 5 ticks, build a new back mountain segment
                if tick % 5 == 0 {
                    for i in 0..(SIZE as usize - 1) {
                        bg[BACK_HILL_INDEX][i] = bg[BACK_HILL_INDEX][i + 1];
                    }
                    buff_2 += 1;
                    let chunk_2 = proceduralgen::gen_perlin_hill_point(
                        ((SIZE - 1) as usize + buff_2),
                        freq,
                        amp_2,
                        1.0,
                        820.0,
                    );
                    bg[BACK_HILL_INDEX][(SIZE - 1) as usize] = chunk_2;
                }

                if object_spawn == 0 {
                    let breakdown = proceduralgen::ProceduralGen::spawn_object(
                        &random,
                        SIZE as i32,
                        (SIZE * 2) as i32,
                    );
                    object = breakdown.0;
                    object_spawn = breakdown.1;

                    object_count += 1; //for now...
                } else {
                    object_spawn -= 1;
                }

                if tick % 10 == 0 {
                    bg_buff -= 1;
                }

                //creates a single obstacle/coin or overwrites the old one
                //everytime one a new one is spawned & adds it to corresponding vector
                //not a good impl bc will not work when > 1 obstacle/coin spawned at a time
                if (object_count > 0) {
                    match object {
                        Some(StaticObject::Statue) => {
                            let obstacle = Obstacle::new(
                                rect!(0, 0, 0, 0),
                                2.0,
                                texture_creator.load_texture("assets/statue.png")?,
                                ObstacleType::Statue,
                            );
                            obstacles.push(obstacle);
                            object_count -= 1;
                        }
                        Some(StaticObject::Coin) => {
                            let coin = Coin::new(
                                rect!(0, 0, 0, 0),
                                texture_creator.load_texture("assets/coin.gif")?,
                                1000,
                            );
                            coins.push(coin);
                            object_count -= 1;
                        }
                        Some(StaticObject::Spring) => {
                            let obstacle = Obstacle::new(
                                rect!(0, 0, 0, 0),
                                1.0,
                                texture_creator.load_texture("assets/temp_spring.jpg")?,
                                ObstacleType::Spring,
                            );
                            obstacles.push(obstacle);
                            object_count -= 1;
                        }
                        Some(StaticObject::Power) => {
                            next_power = Some(rand::random());
                            let pow = Power::new(
                                rect!(0, 0, 0, 0),
                                texture_creator.load_texture("assets/powerup.png")?,
                            );
                            powers.push(pow);
                            object_count -= 1;
                        }
                        _ => {}
                    }
                }
              
                // Vertical draw offset to keep game in vertical bounds
                // Should be calculated before postion updates and added to the y object's y variable
                // Currently is not being applied to the player in an ideal way :(
                // Seems to be fine for obstables?
                let mut vert_draw_offset = 0;
                if (current_ground.y() - TILE_SIZE as i32) < player_upper_bound {
                    vert_draw_offset = player_upper_bound - current_ground.y();
                }
                if current_ground.y() > player_lower_bound {
                    vert_draw_offset = player_lower_bound - current_ground.y();
                }
              
                //Object spawning
                if object_spawn > 0 && object_spawn < SIZE {
                    /* println!(
                        "{:?} | {:?}",
                        object_spawn * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                        CAM_H as i16 - bg[GROUND_INDEX][object_spawn]
                    );*/

                    match object {
                        Some(proceduralgen::StaticObject::Statue) => {
                            //update physics obstacle position
                            for s in obstacles.iter_mut() {
                                //this is hacky & dumb (will only work if one obstacle spawned at a time)
                                if !s.collided() && s.mass > 1.0 {
                                    //once it collides we can't draw it like this
                                    s.hitbox = rect!(
                                        object_spawn * CAM_W as usize / SIZE
                                            + CAM_W as usize / SIZE / 2,
                                        CAM_H as i16
                                            - bg[GROUND_INDEX][object_spawn]
                                            - TILE_SIZE as i16,
                                            + vert_draw_offset as i16,
                                        TILE_SIZE,
                                        TILE_SIZE
                                    );
                                    s.pos = (s.hitbox.x() as f64, s.hitbox.y() as f64);
                                }
                            }
                        }
                        Some(proceduralgen::StaticObject::Coin) => {
                            //update physics coins position
                            for s in coins.iter_mut() {
                                //hacky "soln" part 2
                                s.hitbox = rect!(
                                    object_spawn * CAM_W as usize / SIZE
                                        + CAM_W as usize / SIZE / 2,
                                    CAM_H as i16
                                        - bg[GROUND_INDEX][object_spawn]
                                        - TILE_SIZE as i16
                                        + vert_draw_offset as i16,
                                    TILE_SIZE,
                                    TILE_SIZE
                                );
                                s.pos = (s.hitbox.x() as f64, s.hitbox.y() as f64);
                            }
                        }
                        Some(proceduralgen::StaticObject::Spring) => {
                            //update physics obstacle position
                            for s in obstacles.iter_mut() {
                                //this is hacky & dumb (will only work if one obstacle spawned at a time)
                                if !s.collided() && s.mass<2.0  {   //gaurantees spring for now
                                    //once it collides we can't draw it like this
                                    s.hitbox = rect!(
                                        object_spawn * CAM_W as usize / SIZE
                                            + CAM_W as usize / SIZE / 2,
                                        (CAM_H as i16
                                            - bg[GROUND_INDEX][object_spawn]
                                            - (TILE_SIZE/4) as i16),
                                            + vert_draw_offset as i16,
                                        TILE_SIZE,
                                        TILE_SIZE/4
                                    );
                                    s.pos = (s.hitbox.x() as f64, s.hitbox.y() as f64);
                                }
                            }
                        }
                        Some(proceduralgen::StaticObject::Power) => {
                            //update physics power position
                            for p in powers.iter_mut() {
                                p.pos = rect!(
                                    object_spawn * CAM_W as usize / SIZE
                                        + CAM_W as usize / SIZE / 2,
                                    CAM_H as i16
                                        - bg[GROUND_INDEX][object_spawn]
                                        - TILE_SIZE as i16
                                        + vert_draw_offset as i16,
                                    TILE_SIZE,
                                    TILE_SIZE
                                );
                            }
                        }
                        _ => {}
                    }
                }

                core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                core.wincan.fill_rect(rect!(0, 470, CAM_W, CAM_H))?;

                // Draw background
                core.wincan
                    .copy(&tex_bg, None, rect!(bg_buff, -150, CAM_W, CAM_H))?;
                core.wincan.copy(
                    &tex_bg,
                    None,
                    rect!(bg_buff + (CAM_W as i32), -150, CAM_W, CAM_H),
                )?;

                //Draw sky in background
                core.wincan
                    .copy(&tex_sky, None, rect!(bg_buff, 0, CAM_W, CAM_H / 3))?;
                core.wincan.copy(
                    &tex_sky,
                    None,
                    rect!(CAM_W as i32 + bg_buff, 0, CAM_W, CAM_H / 3),
                )?;

                for i in 0..bg[FRONT_HILL_INDEX].len() - 1 {
                    // Furthest back mountains
                    core.wincan.set_draw_color(Color::RGBA(128, 51, 6, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                        CAM_H as i16 - bg[BACK_HILL_INDEX][i] + vert_draw_offset as i16,
                        CAM_W as usize / SIZE,
                        CAM_H as i16
                    ))?;

                    // Closest mountains
                    core.wincan.set_draw_color(Color::RGBA(96, 161, 152, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                        CAM_H as i16 - bg[FRONT_HILL_INDEX][i] + vert_draw_offset as i16,
                        CAM_W as usize / SIZE,
                        CAM_H as i16
                    ))?;

                    // Ground
                    core.wincan.set_draw_color(Color::RGBA(13, 66, 31, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / SIZE + CAM_W as usize / SIZE / 2,
                        CAM_H as i16 - bg[GROUND_INDEX][i] + vert_draw_offset as i16,
                        CAM_W as usize / SIZE,
                        CAM_H as i16
                    ))?;
                }

                //Power asset drawing
                if power_tick > 0 {
                    match power {
                        Some(powers::PowerUps::SpeedBoost) => {
                            core.wincan.copy(
                                &tex_speed,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(powers::PowerUps::ScoreMultiplier) => {
                            core.wincan.copy(
                                &tex_multiplier,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(powers::PowerUps::BouncyShoes) => {
                            core.wincan.copy(
                                &tex_bouncy,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(powers::PowerUps::LowerGravity) => {
                            core.wincan.copy(
                                &tex_floaty,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(powers::PowerUps::Shield) => {
                            core.wincan.copy(
                                &tex_shield,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        _ => {}
                    }

                    let m = power_tick as f64 / 360.0;

                    let r = 256.0 * (1.0 - m);
                    let g = 256.0 * (m);
                    let w = TILE_SIZE as f64 * m;

                    core.wincan.set_draw_color(Color::RGB(r as u8, g as u8, 0));
                    core.wincan.fill_rect(rect!(10, 210, w as u8, 10))?;
                }

                tick += 1;
                score += tick_score;

                if tick % 3 == 0 && tick % 5 == 0 {
                    tick = 0;
                }

                if -bg_buff == CAM_W as i32 {
                    bg_buff = 0;
                }

                // Draw player
                // Ideally draw offset could be part of position calculations, and that var could be removed from the second rect
                core.wincan.copy_ex(
                    player.texture(),
                    rect!(src_x, 0, TILE_SIZE, TILE_SIZE),
                    rect!(player.x(), player.y() + vert_draw_offset, TILE_SIZE, TILE_SIZE),
                    player.theta() * 180.0 / std::f64::consts::PI,
                    None,
                    false,
                    false,
                )?;
                core.wincan.set_draw_color(Color::BLACK);
              
                // Hacky way of adjusting player's hitbox with the draw offset
                // Ideally draw offset could be part of position calculations, and this could be a regular iter 
                for h in player.hitbox().iter_mut() {
                    (*h).set_y((*h).y() + vert_draw_offset);
                    core.wincan.draw_rect(*h)?;
                }

                // Draw obstacles
                for o in obstacles.iter() {
                    if (o.x() > 50 && o.y() > 20) {
                        //hacky - will not work if more than one obstacle spawned
                        //println!("XXXXX ypos{} vyo{} ayo{}  ", o.pos.1, o.velocity.1, o.accel.1 );
                        match o.o_type {
                            ObstacleType::Statue => {
                                core.wincan.copy_ex(
                                    o.texture(),
                                    None,
                                    rect!(o.pos.0, o.pos.1, TILE_SIZE, TILE_SIZE),
                                    o.theta(),
                                    None,
                                    false,
                                    false,
                                )?;
                                core.wincan.set_draw_color(Color::RED);
                                core.wincan.draw_rect(o.hitbox())?;
                                break
                            }
                            ObstacleType::Spring => {
                                core.wincan.copy_ex(
                                    o.texture(),
                                    None,
                                    rect!(o.pos.0, o.pos.1, TILE_SIZE, TILE_SIZE/4),
                                    o.theta(),
                                    None,
                                    false,
                                    false,
                                )?;
                                core.wincan.set_draw_color(Color::BLUE);
                                core.wincan.draw_rect(o.hitbox())?;
                            }
                            _ => {}
                        }
                    }
                    /*else{
                        drop(o);
                        object_count-= 1;
                    }*/
                }

                //Draw coins
                for c in coins.iter() {
                    //need a method to delete it from vector, possibly somwthing like this
                    /*if c.collected(){
                        coins.retain(|x| x != c.collected);
                    }*/

                    if !c.collected() && c.x() > 50 {
                        //hacky - will not work if more than one coin spawned
                        core.wincan.copy_ex(
                            c.texture(),
                            rect!(src_x, 0, TILE_SIZE, TILE_SIZE),
                            rect!(c.x(), c.y(), TILE_SIZE, TILE_SIZE),
                            0.0,
                            None,
                            false,
                            false,
                        )?;
                        core.wincan.set_draw_color(Color::GREEN);
                        core.wincan.draw_rect(c.hitbox())?;
                    }
                }

                //Draw power
                for p in powers.iter() {
                    //need a method to delete it from vector, possibly somwthing like this
                    /*if p.collected(){
                        powers.retain(|x| x != p.collected);
                    }*/

                    if !p.collected() && p.x() > 50 {
                        //hacky - will not work if more than one coin spawned
                        core.wincan.copy_ex(
                            p.texture(),
                            rect!(src_x, 0, TILE_SIZE, TILE_SIZE),
                            rect!(p.x(), p.y(), TILE_SIZE, TILE_SIZE),
                            0.0,
                            None,
                            false,
                            false,
                        )?;
                        core.wincan.set_draw_color(Color::YELLOW);
                        core.wincan.draw_rect(p.hitbox())?;
                    }
                }

                let surface = font
                    .render(&format!("{:08}", score))
                    .blended(Color::RGBA(255, 0, 0, 100))
                    .map_err(|e| e.to_string())?;
                let score_texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                core.wincan
                    .copy(&score_texture, None, Some(rect!(10, 10, 100, 50)))?;

                core.wincan.present();

                /*let other_surface = font
                    .render(&format!("{:03}", coin_count))
                    .blended(Color::RGBA(100, 0, 200, 100))
                    .map_err(|e| e.to_string())?;
                let coin_count_texture = texture_creator
                    .create_texture_from_surface(&other_surface)
                    .map_err(|e| e.to_string())?;

                core.wincan
                    .copy(&coin_count_texture, None, Some(rect!(160, 10, 80, 50)))?;

                core.wincan.present();*/
            }

            // FPS Calculation
            // the time taken to display the last frame
            let raw_frame_time = last_raw_time.elapsed().as_secs_f64();
            let delay = FRAME_TIME - raw_frame_time;
            // if the amount of time to display the last frame was less than expected, sleep
            // until the expected amount of time has passed
            if delay > 0.0 {
                // using sleep to delay will always cause slightly more delay than intended due
                // to CPU scheduling; possibly find a better way to delay
                sleep(Duration::from_secs_f64(delay));
            }
            // let adjusted_frame_time = last_adjusted_time.elapsed().as_secs_f64();
            all_frames += 1;
            let time_since_last_measurement = last_measurement_time.elapsed();
            // measure the FPS once every second
            if time_since_last_measurement > Duration::from_secs(1) {
                // println!(
                //     "Average FPS: {:.2}",
                //     (all_frames as f64) / time_since_last_measurement.as_secs_f64()
                // );
                all_frames = 0;
                last_measurement_time = Instant::now();
            }

            player.reset_accel();
        }

        Ok(GameState {
            status: Some(next_status),
            score: score,
        })
    }
}

//Remaking rand::random() to fit with powers.
impl Distribution<powers::PowerUps> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> powers::PowerUps {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=4) {
            // rand 0.8
            0 => powers::PowerUps::SpeedBoost,
            1 => powers::PowerUps::ScoreMultiplier,
            2 => powers::PowerUps::BouncyShoes,
            3 => powers::PowerUps::LowerGravity,
            _ => powers::PowerUps::Shield,
        }
    }
}
