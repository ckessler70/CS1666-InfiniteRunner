use crate::physics::Body;
use crate::physics::Physics;
// use crate::physics::Collider;
use crate::physics::Coin;
use crate::physics::Collectible;
use crate::physics::Collider;
use crate::physics::Dynamic;
use crate::physics::Entity;
use crate::physics::Obstacle;
use crate::physics::ObstacleType;
use crate::physics::Player;
use crate::physics::Power;

use crate::proceduralgen;
use crate::proceduralgen::ProceduralGen;
use crate::proceduralgen::TerrainSegment;

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

use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

const FPS: f64 = 60.0;
const FRAME_TIME: f64 = 1.0 / FPS as f64;

const CAM_H: u32 = 720;
const CAM_W: u32 = 1280;
pub const TILE_SIZE: u32 = 100;

// Background sine wave stuff
const IND_BACKGROUND_MID: usize = 0;
const IND_BACKGROUND_BACK: usize = 1;
const BG_CURVES_SIZE: usize = CAM_W as usize / 10;
// const BUFF_LENGTH: usize = CAM_W as usize / 4;

// Bounds to keep the player within
// Used for camera postioning
const PLAYER_UPPER_BOUND: i32 = 2 * TILE_SIZE as i32;
const PLAYER_LOWER_BOUND: i32 = CAM_H as i32 - PLAYER_UPPER_BOUND;
const PLAYER_LEFT_BOUND: i32 = TILE_SIZE as i32;
const PLAYER_RIGHT_BOUND: i32 = (CAM_W / 2) as i32 - (TILE_SIZE / 2) as i32; // More restrictve:
                                                                             // player needs space to react

/* Minimum speed player can move.
 * In actuality, the minimum distance everything moves left relative to the
 * player per iteration of the game loop. Physics Team, please change or
 * remove this as needed. 1 is just an arbitrary small number.
 */
const MIN_SPEED: i32 = 1;

// Max total number of coins, obstacles, and powers that can exist at
// once. Could be split up later for more complicated procgen
const MAX_NUM_OBJECTS: i32 = 10;

pub struct Runner;

impl Game for Runner {
    fn init() -> Result<Self, String> {
        Ok(Runner {})
    }

    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String> {
        core.wincan.set_blend_mode(sdl2::render::BlendMode::Blend);
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        // Font
        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        // Load in all textures
        let texture_creator = core.wincan.texture_creator();
        let tex_bg = texture_creator.load_texture("assets/bg.png")?;
        let tex_sky = texture_creator.load_texture("assets/sky.png")?;
        let tex_grad = texture_creator.load_texture("assets/sunset_gradient.png")?;
        let tex_statue = texture_creator.load_texture("assets/statue.png")?;
        let tex_coin = texture_creator.load_texture("assets/coin.png")?;
        let tex_speed = texture_creator.load_texture("assets/speed.png")?;
        let tex_multiplier = texture_creator.load_texture("assets/multiplier.png")?;
        let tex_bouncy = texture_creator.load_texture("assets/bouncy.png")?;
        let tex_floaty = texture_creator.load_texture("assets/floaty.png")?;
        let tex_shield = texture_creator.load_texture("assets/shield.png")?;
        let tex_shielded = texture_creator.load_texture("assets/shielded_player.png")?;

        let tex_resume = texture_creator
            .create_texture_from_surface(
                &font
                    .render("Escape/Space - Resume Play")
                    .blended(Color::RGBA(119, 3, 252, 255))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;

        let tex_restart = texture_creator
            .create_texture_from_surface(
                &font
                    .render("R - Restart game")
                    .blended(Color::RGBA(119, 3, 252, 255))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;

        let tex_main = texture_creator
            .create_texture_from_surface(
                &font
                    .render("M - Main menu")
                    .blended(Color::RGBA(119, 3, 252, 255))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;

        let tex_quit = texture_creator
            .create_texture_from_surface(
                &font
                    .render("Q - Quit game")
                    .blended(Color::RGBA(119, 3, 252, 255))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;

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

        // Initialize ground / object vectors
        let mut all_terrain: Vec<TerrainSegment> = Vec::new();
        let mut all_obstacles: Vec<Obstacle> = Vec::new();
        let mut all_coins: Vec<Coin> = Vec::new();
        let mut all_powers: Vec<Power> = Vec::new(); // Refers to powers currently spawned on the
                                                     // ground, not active powers

        // Used to keep track of animation status
        let mut player_anim: i32 = 0; // 4 frames of animation
        let mut coin_anim: i32 = 0; // 60 frames of animation

        let mut score: i32 = 0;
        // let mut mult_power_tick: i32 = 0;
        // let mut coin_count: i32 = 0; // How is this being used?

        let mut game_paused: bool = false;
        let mut initial_pause: bool = false;
        let mut game_over: bool = false;
        // let mut power_override: bool = false; // Probably deprecated
        let mut shielded = false;

        // Number of frames to delay the end of the game by for demonstrating player
        // collision this should be removed once the camera tracks the player
        // properly
        let mut game_over_timer = 120;

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        // ???
        let mut next_status = GameStatus::Main;

        // Object spawning vars
        // let mut object_spawn: usize = 0;
        // let mut object_count: i32 = 0;
        let mut spawn_timer: i32 = 500; // Can spawn a new object when it reaches 0
        let mut min_spawn_gap: i32 = 500; // Value spawn_timer is reset to upon spawning
                                          // an object. Decreases over time.
        let mut spawn_dec: i32 = 1; // Timer countdown per game loop

        // let mut curr_power: Option<proceduralgen::Powers> = None;
        // let mut next_power: Option<proceduralgen::Powers> = None;

        // Physics vars
        let mut player_accel_rate: f64 = -10.0;
        let mut player_jump_change: f64 = 0.0;
        let mut player_speed_adjust: f64 = 0.0;

        // Background & sine wave vars
        let mut bg_buff = 0;
        let mut bg_tick = 0;
        // let mut power_tick: i32 = 0;
        let mut buff_1: usize = 0;
        let mut buff_2: usize = 0;
        // Perlin noise curves the player can't interact with, for visuals only
        // Use IND_BACKGROUND_BACK and IND_BACKGROUND_MID
        let mut background_curves: [[i16; BG_CURVES_SIZE]; 2] = [[0; BG_CURVES_SIZE]; 2];

        // Rand thread to be utilized within runner
        let mut rng = rand::thread_rng();

        // Frequency control modifier for background sine waves
        let freq: f32 = rng.gen::<f32>() * 1000.0 + 100.0;

        // Amplitude control modifiers for background sine waves
        let amp_1: f32 = rng.gen::<f32>() * 4.0 + 1.0;
        let amp_2: f32 = rng.gen::<f32>() * 2.0 + amp_1;

        // Perlin Noise init
        let mut random: [[(i32, i32); 256]; 256] = [[(0, 0); 256]; 256];
        for i in 0..random.len() - 1 {
            for j in 0..random.len() - 1 {
                random[i][j] = (rng.gen_range(0..256), rng.gen_range(0..256));
            }
        }

        // Initialize the starting terrain segments
        let p0 = (0.0, (CAM_H / 3) as f64);
        all_terrain.push(ProceduralGen::gen_terrain(
            &random,
            p0,
            CAM_W as i32,
            CAM_H as i32,
            false,
            false,
            false,
        ));
        all_terrain.push(ProceduralGen::gen_terrain(
            &random,
            (
                0.0,
                all_terrain[0].curve()[all_terrain[0].curve().len() - 2].1 as f64,
            ),
            CAM_W as i32,
            CAM_H as i32,
            false,
            false,
            false,
        ));

        // ground_buffer = proceduralgen::ProceduralGen::gen_bezier_land(
        //     &random,
        //     p0,
        //     CAM_W as i32,
        //     CAM_H as i32,
        //     false,
        //     false,
        //     false,
        // );

        // Pre-Generate perlin curves for background hills
        for i in 0..BG_CURVES_SIZE {
            background_curves[IND_BACKGROUND_MID][i] =
                proceduralgen::gen_perlin_hill_point((i + buff_1), freq, amp_1, 0.5, 600.0);
            background_curves[IND_BACKGROUND_BACK][i] =
                proceduralgen::gen_perlin_hill_point((i + buff_2), freq, amp_2, 1.0, 820.0);
        }

        /* ~~~~~~ Main Game Loop ~~~~~~ */
        'gameloop: loop {
            last_raw_time = Instant::now(); // FPS tracking

            let mut curr_step_score: i32 = 0; // Score collect in a single iteration of the game loop

            /* ~~~~~~ Pausing Handler ~~~~~~ */
            if game_paused {
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
                } // End Loop

                // Draw pause screen once due to BlendMode setting
                if initial_pause {
                    // Pause screen background, semitransparent grey
                    core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 128));
                    core.wincan.fill_rect(rect!(0, 0, CAM_W, CAM_H))?;

                    // Draw pause screen text
                    core.wincan
                        .copy(&tex_resume, None, Some(rect!(100, 100, 1000, 125)))?;
                    core.wincan
                        .copy(&tex_restart, None, Some(rect!(100, 250, 700, 125)))?;
                    core.wincan
                        .copy(&tex_main, None, Some(rect!(100, 400, 600, 125)))?;
                    core.wincan
                        .copy(&tex_quit, None, Some(rect!(100, 550, 600, 125)))?;

                    core.wincan.present();
                    initial_pause = false;
                }
            }
            // Normal unpaused game state
            else {
                // End game loop, 'player has lost' state
                if game_over {
                    game_over_timer -= 1; // Animation buffer?
                    if game_over_timer == 0 {
                        break 'gameloop;
                    }
                }

                /*
                If only there was some way for us to communicate the purpose of a section of code.
                if all_terrain[0].x() > all_terrain[0].curve().len() as i32 - 1 {
                    all_terrain.remove(0);
                    all_terrain.push(ProceduralGen::gen_terrain(
                        &random,
                        (
                            0.0,
                            all_terrain[0].curve()[all_terrain[0].curve().len() - 2].1 as f64,
                        ),
                        CAM_W as i32,
                        CAM_H as i32,
                        false,
                        false,
                        false,
                    ));
                }

                ground_buffer = Vec::new(); // Reinit it

                ground_buffer = all_terrain[0].get_view();

                if ground_buffer.len() < BG_CURVES_SIZE {
                    let mut ap_buff: Vec<(i32, i32)> = Vec::new();

                    ap_buff = all_terrain[1].get_view();

                    for i in ap_buff.iter_mut() {
                        if ground_buffer.len() < BG_CURVES_SIZE {
                            ground_buffer.push(*i);
                        } else {
                            break;
                        }
                    }
                }
                */

                /* Deprecated
                let current_ground = Point::new(
                    player.x(),
                    CAM_H as i32
                        - ground_buffer[((player.x() as usize)
                            / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                            % 128]
                            .1,
                );
                // Right ground position
                let next_ground = Point::new(
                    player.x() + TILE_SIZE as i32,
                    CAM_H as i32
                        - ground_buffer[(((player.x() + TILE_SIZE as i32) as usize)
                            / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                            % 128]
                            .1,
                );
                // Angle between
                let angle = ((next_ground.y() as f64 - current_ground.y() as f64)
                    / (TILE_SIZE as f64))
                    .atan();
                */

                let current_ground: TerrainSegment = get_current_ground(all_terrain, player.x());
                let angle = current_ground.angle_from_last();

                //Your time has come (refractor)
                // This conditional statement is here so that the game will go on for a few more
                // frames without player input once the player has died. The reason for this is
                // to demonstrate collisions even though the camera does not follow the player.
                // NOTE: Once the camera properly follows the player, this conditional should be
                // removed.
                //if !game_over {
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

                mult_power_tick = 1;
                //}

                /* ~~~~~~ Player Collecting an Object Section ~~~~~~ */
                /* Unnesseccary as player doesn't collect obstacles
                let mut to_remove: i32 = -1;
                let mut counter = 0;

                // Description
                //in the future when obstacles & coins are proc genned we will probs wanna
                //only check for obstacles/coins based on their location relative to players x
                // cord
                //(also: idt this can be a for loop bc it moves the obstacles values?)
                for obs in all_obstacles.iter_mut() {
                    //.filter(|near by obstacles|).collect()
                    if let Some(collision_boxes) = player.check_collision(obs) {
                        to_remove = counter;

                        if !player.collide(obs, collision_boxes, shielded) {
                            game_over = true;
                            initial_pause = true;
                            continue 'gameloop;
                        }
                        //println!("ypos{} vyo{} ayo{}  ", obs.pos.1, obs.velocity.1, obs.accel.1
                        // ); obs.update_vel(0.0,0.0);   //these args do
                        // nothing obs.update_pos(Point::new(0,0), 3.0);
                        // //the 3 makes the obstacle spin println!("ypos{}
                        // vyo{} ayo{}  ", obs.pos.1, obs.velocity.1, obs.accel.1 );
                        // Real Solution: need to actually resolve the collision, should go
                        // something like this player.collide(obs);
                        // Physics::apply_gravity(obs, 0.0, 0.3); //maybe...
                        continue;
                    };
                    counter += 1;
                }
                if to_remove != -1 {
                    obstacles.remove(to_remove as usize);
                }
                */

                // Remove coins if player collects them
                let mut to_remove_ind: i32 = -1;
                let mut counter = 0;
                for coin in all_coins.iter_mut() {
                    if Physics::check_collection(&mut player, coin) {
                        if !coin.collected() {
                            to_remove_ind = counter;

                            //so you only collect each coin once
                            coin.collect(); //deletes the coin once collected (but takes too long)
                            score +=      // coin_count += 1;
                            mult_power_tick += coin.value(); //increments the
                                                             // score
                                                             // based on the
                                                             // coins
                                                             // value
                                                             // maybe print next
                                                             // to
                                                             // score: "+ c.
                                                             // value()""
                        }
                        continue;
                    }
                    counter += 1;
                }
                if to_remove_ind != -1 {
                    all_coins.remove(to_remove_ind as usize);
                }

                // Remove power ups if player collects them
                // Rough, but should follow the coin idea closely.
                let mut to_remove_ind: i32 = -1;
                let mut counter = 0;
                for power in all_powers.iter_mut() {
                    if Physics::check_power(&mut player, power) {
                        if !power.collected() {
                            to_remove_ind = counter;

                            match next_power {
                                Some(proceduralgen::Powers::SpeedBoost) => {
                                    curr_power = Some(proceduralgen::Powers::SpeedBoost);
                                }
                                Some(proceduralgen::Powers::ScoreMultiplier) => {
                                    curr_power = Some(proceduralgen::Powers::ScoreMultiplier);
                                }
                                Some(proceduralgen::Powers::BouncyShoes) => {
                                    curr_power = Some(proceduralgen::Powers::BouncyShoes);
                                }
                                Some(proceduralgen::Powers::LowerGravity) => {
                                    curr_power = Some(proceduralgen::Powers::LowerGravity);
                                }
                                Some(proceduralgen::Powers::Shield) => {
                                    curr_power = Some(proceduralgen::Powers::Shield);
                                }
                                _ => {}
                            }

                            // Reset any previously active power values to default
                            power_override = false;
                            player_accel_rate = -10.0;
                            player_jump_change = 0.0;
                            player_speed_adjust = 0.0;
                            shielded = false;

                            power.collect();
                            power_tick = 360;
                        }
                        continue;
                    }
                    counter += 1;
                }
                if to_remove_ind != -1 {
                    all_powers.remove(to_remove_ind as usize);
                }

                /* ~~~~~~ Power Handling Section ~~~~~~ */
                if power_tick > 0 {
                    power_tick -= 1;
                    match curr_power {
                        Some(proceduralgen::Powers::SpeedBoost) => {
                            // May not be the proper way to handle this.
                            // Adds player speed adjust to player's velocity
                            player_speed_adjust = 5.0;
                        }
                        Some(proceduralgen::Powers::ScoreMultiplier) => {
                            // Doubles tick score while active
                            mult_power_tick *= 2;
                        }
                        Some(proceduralgen::Powers::BouncyShoes) => {
                            // Forces jumping while active and jumps 0.3 velocity units higher
                            player_jump_change = 0.3;
                            // This will need changed for refractor
                            player.jump(current_ground, true, player_jump_change);
                        }
                        Some(proceduralgen::Powers::LowerGravity) => {
                            // Accel rate is how the y velocity is clamped
                            // Has player jump 0.2 velocity units higher.
                            player_accel_rate = -5.0;
                            player_jump_change = 0.2;
                        }
                        Some(proceduralgen::Powers::Shield) => {
                            // Shielded will say to ignore obstacle collisions
                            shielded = true;
                        }
                        _ => {}
                    }
                } else if power_tick == 0 {
                    power_tick -= 1;

                    // Reset values to default if power times out
                    match curr_power {
                        // Stop any power from going
                        Some(proceduralgen::Powers::SpeedBoost) => {
                            player_speed_adjust = 0.0;
                        }
                        Some(proceduralgen::Powers::ScoreMultiplier) => {}
                        Some(proceduralgen::Powers::BouncyShoes) => {
                            player_jump_change = 0.0;
                        }
                        Some(proceduralgen::Powers::LowerGravity) => {
                            player_accel_rate = -10.0;
                            player_jump_change = 0.0;
                        }
                        Some(proceduralgen::Powers::Shield) => {
                            shielded = false;
                        }
                        _ => {}
                    }
                    curr_power = None;
                }

                // Applies gravity, normal & friction now
                // Friciton is currently way OP (stronger than grav) bc cast to i32 in
                // apply_force so to ever have an effect, it needs to be set > 1
                // for now...
                Physics::apply_gravity(&mut player, angle, 0.3);

                //apply friction
                //Physics::apply_friction(&mut player, 1.0);

                for obs in all_obstacles.iter_mut() {
                    obs.update_vel(0.0, 0.0); // These args do nothing
                    obs.update_pos(Point::new(0, 0), 15.0, false);
                }
                player.update_pos(current_ground, angle, game_over);
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

                // Generate new terrain / objects if player hasn't died
                if !game_over {
                    // Every 3 ticks, build a new front mountain segment
                    if bg_tick % 3 == 0 {
                        for i in 0..(BG_CURVES_SIZE as usize - 1) {
                            background_curves[IND_BACKGROUND_MID][i] =
                                background_curves[IND_BACKGROUND_MID][i + 1];
                        }
                        buff_1 += 1;
                        let chunk_1 = proceduralgen::gen_perlin_hill_point(
                            ((BG_CURVES_SIZE - 1) as usize + buff_1),
                            freq,
                            amp_1,
                            0.5,
                            600.0,
                        );
                        background_curves[IND_BACKGROUND_MID][(BG_CURVES_SIZE - 1) as usize] =
                            chunk_1;
                    }

                    // Every 5 ticks, build a new back mountain segment
                    if bg_tick % 5 == 0 {
                        for i in 0..(BG_CURVES_SIZE as usize - 1) {
                            background_curves[IND_BACKGROUND_BACK][i] =
                                background_curves[IND_BACKGROUND_BACK][i + 1];
                        }
                        buff_2 += 1;
                        let chunk_2 = proceduralgen::gen_perlin_hill_point(
                            ((BG_CURVES_SIZE - 1) as usize + buff_2),
                            freq,
                            amp_2,
                            1.0,
                            820.0,
                        );
                        background_curves[IND_BACKGROUND_BACK][(BG_CURVES_SIZE - 1) as usize] =
                            chunk_2;
                    }

                    // Decrease min_spawn_gap to inscrease spawn rates based on score
                    // These numbers are probably terrible, we should mess around with it
                    if score > 10000 {
                        min_spawn_gap = 15;
                    } else if score > 20000 {
                        min_spawn_gap = 20;
                    } else if score > 30000 {
                        min_spawn_gap = 25;
                    } else if score > 40000 {
                        min_spawn_gap = 30;
                    } else if score > 50000 {
                        min_spawn_gap = 35;
                    } else if score > 60000 {
                        min_spawn_gap = 40;
                    } else if score > 70000 {
                        min_spawn_gap = 45;
                    } else if score > 80000 {
                        min_spawn_gap = 50;
                    } else if score > 90000 {
                        min_spawn_gap = 60;
                    } else if score > 100000 {
                        min_spawn_gap = 70;
                    }

                    // Generate new objects
                    let curr_num_objects = all_obstacles.len() + all_coins.len() + all_powers.len();
                    let spawn_trigger = rng.gen_range(0..MAX_NUM_OBJECTS);
                    if spawn_timer > 0 {
                        spawn_timer -= spawn_dec;
                    } else if spawn_trigger >= curr_num_objects {
                        object = Some(proceduralgen::choose_static_object());
                        spawn_timer = min_spawn_gap;
                    } else if spawn_trigger < curr_num_objects {
                        // Min spawn gap can be replaced with basically any value for this random
                        // range. Smaller values will spawn objects more often
                        spawn_timer = rng.gen_range(0..min_spawn_gap);
                    }
                    /*
                    if tick % spawn_timer == 0 {
                        let num_active = all_obstacles.len() + all_coins.len() + all_powers.len();
                        let spawn_check = rng.gen_range(0..=10);

                        if spawn_check > num_active {
                            object = Some(proceduralgen::choose_static_object());

                            object_count += 1;
                        } else {
                            object = None;
                        }
                    }
                    */

                    // Shift background images & sine waves?
                    if bg_tick % 10 == 0 {
                        bg_buff -= 1;
                    }

                    //creates a single obstacle/coin or overwrites the old one
                    //everytime one a new one is spawned & adds it to corresponding vector
                    //not a good impl bc will not work when > 1 obstacle/coin spawned at a time
                    if (object_count > 0) {
                        match object {
                            Some(StaticObject::Statue) => {
                                let obstacle = Obstacle::new(
                                    rect!(CAM_W, 0, 0, 0),
                                    50.0,
                                    texture_creator.load_texture("assets/statue.png")?,
                                    ObstacleType::Statue,
                                );
                                all_obstacles.push(obstacle);
                                object_count -= 1;
                                // object = None;
                            }
                            Some(StaticObject::Coin) => {
                                let coin = Coin::new(
                                    rect!(CAM_W, 0, 0, 0),
                                    texture_creator.load_texture("assets/coin.png")?,
                                    1000,
                                );
                                all_coins.push(coin);
                                object_count -= 1;
                                // object = None;
                            }
                            Some(StaticObject::Spring) => {
                                let obstacle = Obstacle::new(
                                    rect!(CAM_W, 0, 0, 0),
                                    1.0,
                                    texture_creator.load_texture("assets/temp_spring.jpg")?,
                                    ObstacleType::Spring,
                                );
                                all_obstacles.push(obstacle);
                                object_count -= 1;
                                // object = None;
                            }
                            Some(StaticObject::Power) => {
                                next_power = Some(proceduralgen::choose_power_up());
                                let pow = Power::new(
                                    rect!(CAM_W, 0, 0, 0),
                                    texture_creator.load_texture("assets/powerup.png")?,
                                );
                                all_powers.push(pow);
                                object_count -= 1;
                                // object = None;
                            }
                            _ => {}
                        }
                    }

                    // Object spawning
                    match object {
                        Some(proceduralgen::StaticObject::Statue) => {
                            //update physics obstacle position
                            for obs in all_obstacles.iter_mut() {
                                //this is hacky & dumb (will only work if one obstacle spawned
                                // at a time)
                                if !obs.collided() && obs.mass > 1.0 {
                                    let mut y_offset = all_terrain[0].x()
                                        + ((player.x() as usize)
                                            / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                                            as i32
                                        + obs.x();
                                    y_offset = if y_offset > 1280 {
                                        all_terrain[1].curve()[(y_offset % 1280) as usize].1
                                    } else {
                                        all_terrain[0].curve()[y_offset as usize].1
                                    };

                                    //once it collides we can't draw it like this
                                    obs.hitbox = rect!(
                                        obs.x(),
                                        CAM_H as i32
                                        - y_offset
                                        // - y pos at terrain segment under player
                                        - TILE_SIZE as i32,
                                        TILE_SIZE,
                                        TILE_SIZE
                                    );
                                    obs.pos = (obs.hitbox.x() as f64, obs.hitbox.y() as f64);
                                    println!("obstacles {:?}", obs.pos);
                                }
                            }
                        }
                        Some(proceduralgen::StaticObject::Coin) => {
                            //update physics coins position
                            for coin in all_coins.iter_mut() {
                                let mut y_offset = all_terrain[0].x()
                                    + ((player.x() as usize)
                                        / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                                        as i32
                                    + coin.x();
                                y_offset = if y_offset > 1280 {
                                    all_terrain[1].curve()[(y_offset % 1280) as usize].1
                                } else {
                                    all_terrain[0].curve()[y_offset as usize].1
                                };

                                //hacky "soln" part 2
                                coin.hitbox = rect!(
                                    coin.x(),
                                    CAM_H as i32 - y_offset - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                );
                                coin.pos = (coin.hitbox.x() as f64, coin.hitbox.y() as f64);
                                println!("coins {:?}", coin.pos);
                            }
                        }
                        Some(proceduralgen::StaticObject::Spring) => {
                            //update physics obstacle position
                            for obs in all_obstacles.iter_mut() {
                                //this is hacky & dumb (will only work if one obstacle spawned
                                // at a time)
                                if !obs.collided() && obs.mass < 2.0 {
                                    let mut y_offset = all_terrain[0].x()
                                        + ((player.x() as usize)
                                            / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                                            as i32
                                        + obs.x();
                                    y_offset = if y_offset > 1280 {
                                        all_terrain[1].curve()[(y_offset % 1280) as usize].1
                                    } else {
                                        all_terrain[0].curve()[y_offset as usize].1
                                    };

                                    //gaurantees spring for now
                                    //once it collides we can't draw it like this
                                    obs.hitbox = rect!(
                                        obs.x(),
                                        (CAM_H as i32 - y_offset - (TILE_SIZE / 4) as i32),
                                        TILE_SIZE,
                                        TILE_SIZE / 4
                                    );
                                    obs.pos = (obs.hitbox.x() as f64, obs.hitbox.y() as f64);
                                    println!("springs {:?}", obs.pos);
                                }
                            }
                        }
                        Some(proceduralgen::StaticObject::Power) => {
                            //update physics power position
                            for power in all_powers.iter_mut() {
                                let mut y_offset = all_terrain[0].x()
                                    + ((player.x() as usize)
                                        / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                                        as i32
                                    + power.x();
                                y_offset = if y_offset > 1280 {
                                    all_terrain[1].curve()[(y_offset % 1280) as usize].1
                                } else {
                                    all_terrain[0].curve()[y_offset as usize].1
                                };

                                power.pos = rect!(
                                    power.x(),
                                    CAM_H as i32 - y_offset - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                );

                                println!("powers {:?}", power.pos);
                            }
                        }
                        _ => {}
                    }
                }

                /* Update score, and increase object spawn rates
                 * if score passes a milestone
                 */
                // This should be placed after hitbox updates but before drawing
                if !game_over {
                    curr_step_score *= power_multiplier;
                    score += curr_step_score;
                }

                /*
                // Wouldn't it be nice if there was some way to communicate the purpose
                // of a code block?
                spawn_timer = if score > 10000 && score < 20000 {
                    390
                } else if score > 20000 && score < 30000 {
                    380
                } else if score > 30000 && score < 40000 {
                    370
                } else if score > 40000 && score < 50000 {
                    360
                } else if score > 50000 && score < 60000 {
                    350
                } else if score > 60000 && score < 70000 {
                    340
                } else if score > 70000 && score < 80000 {
                    330
                } else if score > 80000 && score < 90000 {
                    320
                } else if score > 90000 && score < 100000 {
                    300 // Cap?
                } else {
                    400 // Default
                };
                */

                /* Update ground / object positions to move player forward
                 * by the distance they should move this single iteration of the game loop
                 */
                let iteration_distance: i32 = MIN_SPEED + player.vel_x() as i32;
                for ground in all_terrain.iter() {
                    ground.travel_update(iteration_distance);
                }
                /*  travel_update needs to be implemented in physics.rs
                    for obstacles, coins and power ups.
                    See terrain segment implementation in proceduralgen.rs,
                    it should be almost exactly the same

                for obs in all_obstacles.iter() {
                    obs.travel_update(iteration_distance);
                }
                for coin in all_coins.iter() {
                    coin.travel_update(iteration_distance);
                }
                for powerUp in all_powers.iter() {
                    powerUp.travel_update(iteration_distance);
                }
                */

                /* ~~~~~~ Begin Camera Section ~~~~~~ */
                /* This should be the very last section of calcultions,
                 * as the camera position relies upon updated math for
                 * EVERYTHING ELSE. Below the camera section we have
                 * removal of offscreen objects from their vectors,
                 * animation updates, the drawing section, and FPS calculation only.
                 */
                let mut camera_adj_x: i32 = 0;
                let mut camera_adj_y: i32 = 0;

                // Adjust camera horizontally if updated player x pos is out of bounds
                if player.x() < PLAYER_LEFT_BOUND {
                    let camera_adj_x = PLAYER_LEFT_BOUND - player.x();
                } else if (current_ground.x() + TILE_SIZE as i32) > PLAYER_RIGHT_BOUND {
                    let camera_adj_x = PLAYER_RIGHT_BOUND - player.x();
                }

                // Adjust camera vertically based on y/height of the ground
                if current_ground.y() < PLAYER_UPPER_BOUND {
                    let camera_adj_y = PLAYER_UPPER_BOUND - current_ground.y();
                } else if (current_ground.y() + TILE_SIZE as i32) > PLAYER_LOWER_BOUND {
                    let camera_adj_y = PLAYER_LOWER_BOUND - current_ground.y();
                }

                // Add adjustment to terrain
                for ground in all_terrain.iter() {
                    ground.camera_adj(camera_adj_x, camera_adj_y);
                }

                /*  camera_adj needs to be implemented in physics.rs
                    for obstacles, coins and power ups, and the player.
                    See terrain segment implementation in proceduralgen.rs,
                    it should be almost exactly the same.

                // Add adjustment to obstacles
                for obs in all_obstacles.iter() {
                    obs.travel_update(iteration_distance);
                }

                // Add adjustment to coins
                for coin in all_coins.iter() {
                    coin.travel_update(iteration_distance);
                }
                // Add adjustment to power ups
                for powerUp in all_powers.iter() {
                    powerUp.travel_update(iteration_distance);
                }

                // Add adjustment to player
                player.camera_adj(camera_adj_x, camera_adj_y);
                */
                /* ~~~~~~ End Camera Section ~~~~~~ */

                /* ~~~~~~ Remove stuff which is now offscreen ~~~~~~ */
                // Terrain
                let mut ind = 0;
                for ground in all_terrain.iter() {
                    if ground.x() + ground.w() <= 0 {
                        all_terrain.remove(ind);
                    }
                    ind += 1;
                }

                //  Obstacles
                ind = 0;
                for obs in all_obstacles.iter() {
                    if obs.x() + TILE_SIZE as i32 <= 0 {
                        all_obstacles.remove(ind);
                    }
                    ind += 1;
                }

                // Coins
                ind = 0;
                for coin in all_coins.iter() {
                    if coin.x() + TILE_SIZE as i32 <= 0 {
                        all_coins.remove(ind);
                    }
                    ind += 1;
                }

                // Power ups
                ind = 0;
                for power in all_powers.iter() {
                    if power.x() + TILE_SIZE as i32 <= 0 {
                        all_powers.remove(ind);
                    }
                    ind += 1;
                }
                /*
                let mut to_remove: Vec<_> = Vec::new();
                let mut counter = 0;

                for o in obstacles.iter_mut() {
                    let y_pos = if (o.x() - camera_adj_x.abs()) < CAM_W as i32 {
                        CAM_H as i32
                            - ground_buffer[((((o.x() - camera_adj_x.abs()) + TILE_SIZE as i32)
                                as usize)
                                / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                                % 128]
                                .1
                            - TILE_SIZE as i32
                    } else {
                        0
                    };

                    o.pos = ((o.x() - camera_adj_x.abs()) as f64, y_pos as f64);
                    o.align_hitbox_to_pos();
                    if o.x() <= 0 {
                        to_remove.push(counter);
                    }

                    // println!("obstacles {:?}", o.hitbox());
                    counter += 1;
                }

                for r in to_remove.iter_mut() {
                    obstacles.remove(*r);
                }

                //Coins
                let mut to_remove: Vec<_> = Vec::new();
                let mut counter = 0;

                for c in coins.iter_mut() {
                    let y_pos = if (c.x() - camera_adj_x.abs()) < CAM_W as i32 {
                        CAM_H as i32
                            - ground_buffer[((((c.x() - camera_adj_x.abs()) + TILE_SIZE as i32)
                                as usize)
                                / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                                % 128]
                                .1
                            - TILE_SIZE as i32
                    } else {
                        0
                    };

                    c.pos = ((c.x() - camera_adj_x.abs()) as f64, y_pos as f64);
                    c.align_hitbox_to_pos();
                    if c.x() <= 0 {
                        to_remove.push(counter);
                    }
                    // println!("coins {:?}", c.hitbox());
                    counter += 1;
                }

                for r in to_remove.iter_mut() {
                    coins.remove(*r);
                }

                //PowerUps
                let mut to_remove: Vec<_> = Vec::new();
                let mut counter = 0;

                for p in powers.iter_mut() {
                    let y_pos = if (p.x() - camera_adj_x.abs()) < CAM_W as i32 {
                        CAM_H as i32
                            - ground_buffer[((((p.x() - camera_adj_x.abs()) + TILE_SIZE as i32)
                                as usize)
                                / (CAM_W / BG_CURVES_SIZE as u32) as usize)
                                % 128]
                                .1
                            - TILE_SIZE as i32
                    } else {
                        0
                    };

                    p.pos = rect!(
                        (p.x() - camera_adj_x.abs()) as f64,
                        y_pos as f64,
                        TILE_SIZE,
                        TILE_SIZE
                    ); // Don't know why this one needs a full rect declearation to update pos
                       // println!("powers {:?}", p.hitbox());
                    if p.x() <= 0 {
                        to_remove.push(counter);
                    }
                    counter += 1;
                }

                for r in to_remove.iter_mut() {
                    powers.remove(*r);
                }
                */

                tick += 1;

                if tick % 2 == 0 {
                    player_anim += 1;
                    player_anim %= 4;
                }

                coin_anim += 1;
                coin_anim %= 60;

                if tick % 3 == 0 && tick % 5 == 0 && tick % spawn_timer == 0 {
                    tick = 0;
                }

                if -bg_buff == CAM_W as i32 {
                    bg_buff = 0;
                }

                /* ~~~~~~ Draw All Elements ~~~~~~ */
                core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                core.wincan.fill_rect(rect!(0, 470, CAM_W, CAM_H))?;

                // Background
                core.wincan
                    .copy(&tex_bg, None, rect!(bg_buff, -150, CAM_W, CAM_H))?;
                core.wincan.copy(
                    &tex_bg,
                    None,
                    rect!(bg_buff + (CAM_W as i32), -150, CAM_W, CAM_H),
                )?;

                // Sky
                core.wincan
                    .copy(&tex_sky, None, rect!(bg_buff, 0, CAM_W, CAM_H / 3))?;
                core.wincan.copy(
                    &tex_sky,
                    None,
                    rect!(CAM_W as i32 + bg_buff, 0, CAM_W, CAM_H / 3),
                )?;

                // Background perlin noise curves
                for i in 0..background_curves[IND_BACKGROUND_MID].len() - 1 {
                    // Furthest back perlin noise curves
                    core.wincan.set_draw_color(Color::RGBA(128, 51, 6, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / BG_CURVES_SIZE + CAM_W as usize / BG_CURVES_SIZE / 2,
                        CAM_H as i16 - background_curves[IND_BACKGROUND_BACK][i],
                        CAM_W as usize / BG_CURVES_SIZE,
                        CAM_H as i16
                    ))?;

                    // Midground perlin noise curves
                    core.wincan.set_draw_color(Color::RGBA(96, 161, 152, 255));
                    core.wincan.fill_rect(rect!(
                        i * CAM_W as usize / BG_CURVES_SIZE + CAM_W as usize / BG_CURVES_SIZE / 2,
                        CAM_H as i16 - background_curves[IND_BACKGROUND_MID][i],
                        CAM_W as usize / BG_CURVES_SIZE,
                        CAM_H as i16
                    ))?;
                }

                // Terrain
                for ground in all_terrain.iter() {
                    core.wincan.set_draw_color(ground.color());
                    core.wincan.fill_rect(ground.pos())?;
                }
                /*
                let mut ground_ct = 0;
                for i in ground_buffer.iter_mut() {
                    core.wincan.set_draw_color(all_terrain[0].color());
                    core.wincan.fill_rect(rect! {
                        ground_ct * CAM_W as usize / BG_CURVES_SIZE + CAM_W as usize / BG_CURVES_SIZE / 2,
                        CAM_H as i32 - i.1,
                        CAM_W as usize / BG_CURVES_SIZE,
                        CAM_H as i32
                    })?;

                    ground_ct += 1;
                }
                */

                /*
                // Power assets
                if power_tick > 0 {
                    match curr_power {
                        Some(proceduralgen::Powers::SpeedBoost) => {
                            core.wincan.copy(
                                &tex_speed,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(proceduralgen::Powers::ScoreMultiplier) => {
                            core.wincan.copy(
                                &tex_multiplier,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(proceduralgen::Powers::BouncyShoes) => {
                            core.wincan.copy(
                                &tex_bouncy,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(proceduralgen::Powers::LowerGravity) => {
                            core.wincan.copy(
                                &tex_floaty,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(proceduralgen::Powers::Shield) => {
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
                */

                // Set player texture
                let mut tex_player = player.texture(); // Default
                if shielded {
                    tex_player = &tex_shielded;
                } /* else if ... {
                      Other player textures
                  } */

                // Player
                core.wincan.copy_ex(
                    tex_player,
                    rect!(player_anim * TILE_SIZE as i32, 0, TILE_SIZE, TILE_SIZE),
                    rect!(player.x(), player.y(), TILE_SIZE, TILE_SIZE),
                    player.theta() * 180.0 / std::f64::consts::PI,
                    None,
                    false,
                    false,
                )?;
                core.wincan.set_draw_color(Color::BLACK);

                // Player's hitbox
                for h in player.hitbox().iter() {
                    core.wincan.draw_rect(*h)?;
                }

                // Obstacles
                for obs in obstacles.iter() {
                    // What is the purpose of this conditional?
                    // All obstacles should be drawn no matter their position onscreen
                    if (obs.x() > 50 && obs.y() > 20) {
                        //hacky - will not work if more than one obstacle spawned
                        //println!("XXXXX ypos{} vyo{} ayo{}  ", o.pos.1, o.velocity.1, o.accel.1
                        // );
                        match obs.o_type {
                            ObstacleType::Statue => {
                                core.wincan.copy_ex(
                                    obs.texture(),
                                    None,
                                    rect!(obs.pos.0, obs.pos.1, TILE_SIZE, TILE_SIZE),
                                    obs.theta(),
                                    None,
                                    false,
                                    false,
                                )?;
                                core.wincan.set_draw_color(Color::RED);
                                core.wincan.draw_rect(obs.hitbox())?;
                                break;
                            }
                            ObstacleType::Spring => {
                                core.wincan.copy_ex(
                                    obs.texture(),
                                    None,
                                    rect!(obs.pos.0, obs.pos.1, TILE_SIZE, TILE_SIZE / 4),
                                    obs.theta(),
                                    None,
                                    false,
                                    false,
                                )?;
                                core.wincan.set_draw_color(Color::BLUE);
                                core.wincan.draw_rect(obs.hitbox())?;
                            }
                            _ => {}
                        }
                    }
                    /*else{
                        drop(obs);
                        object_count-= 1;
                    }*/
                }

                // Coins
                for c in all_coins.iter() {
                    //need a method to delete it from vector, possibly something like this
                    // Should be handled after hitbox updates but before draw section
                    /*if c.collected(){
                        coins.retain(|x| x != c.collected);
                    }*/

                    // What is the purpose of this conditional?
                    // All coins should be drawn no matter their position onscreen
                    if !c.collected() && c.x() > 50 {
                        //hacky - will not work if more than one coin spawned
                        core.wincan.copy_ex(
                            c.texture(),
                            rect!(coin_anim * TILE_SIZE as i32, 0, TILE_SIZE, TILE_SIZE),
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

                // Powers
                for p in powers.iter() {
                    //need a method to delete it from vector, possibly somwthing like this
                    // Should be handled after hitbox updates but before draw section
                    /*if p.collected(){
                        powers.retain(|x| x != p.collected);
                    }*/

                    // What is the purpose of this conditional?
                    // All powers should be drawn no matter their position onscreen
                    if !p.collected() && p.x() > 50 {
                        //hacky - will not work if more than one power spawned
                        core.wincan.copy_ex(
                            p.texture(),
                            rect!(0, 0, TILE_SIZE, TILE_SIZE),
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

                // Setup for the text of the score to be displayed
                let tex_score = font
                    .render(&format!("{:08}", score))
                    .blended(Color::RGBA(255, 0, 0, 100))
                    .map_err(|e| e.to_string())?;

                // Display score
                let score_texture = texture_creator
                    .create_texture_from_surface(&tex_score)
                    .map_err(|e| e.to_string())?;
                core.wincan
                    .copy(&score_texture, None, Some(rect!(10, 10, 100, 50)))?;

                if game_over {
                    // decrement the amount of frames until the game ends in order to demonstrate
                    // the collision
                    let game_over_texture = texture_creator
                        .create_texture_from_surface(
                            &font
                                .render("GAME OVER")
                                .blended(Color::RGBA(255, 0, 0, 255))
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?;

                    // Cleaned up calculation of texture position
                    // Check previous versions if you want those calculations
                    core.wincan
                        .copy(&game_over_texture, None, Some(rect!(239, 285, 801, 149)))?;
                }

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

            /* ~~~~~~ FPS Calculation ~~~~~~ */
            // Time taken to display the last frame
            let raw_frame_time = last_raw_time.elapsed().as_secs_f64();
            let delay = FRAME_TIME - raw_frame_time;
            // If the amount of time to display the last frame was less than expected, sleep
            // until the expected amount of time has passed
            if delay > 0.0 {
                // Using sleep to delay will always cause slightly more delay than intended due
                // to CPU scheduling; possibly find a better way to delay
                sleep(Duration::from_secs_f64(delay));
            }
            all_frames += 1;
            let time_since_last_measurement = last_measurement_time.elapsed();
            // Measures the FPS once per second
            if time_since_last_measurement > Duration::from_secs(1) {
                all_frames = 0;
                last_measurement_time = Instant::now();
            }

            // The very last thing in the game loop
            // Is this some kind of physics thing that I'm too proceduralgen to understand?
            player.reset_accel();
        }

        /* ~~~~~~ Helper Functions ~~~~~ */
        fn get_current_ground(all_terrain: Vec<TerrainSegment>, player_x: i32) -> TerrainSegment {
            for ground in all_terrain.iter() {
                if (ground.x() <= player_x) & (ground.x() + ground.w() >= player_x) {
                    return *ground;
                }
            }
            return *all_terrain.get(0).unwrap(); // Probably a bad idea... but
                                                 // idk what else to use do
        }

        Ok(GameState {
            status: Some(next_status),
            score: score,
        })
    }
}
