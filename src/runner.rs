use crate::physics::Body;
use crate::physics::Coin;
use crate::physics::Collectible;
use crate::physics::Entity;
use crate::physics::Obstacle;
use crate::physics::Physics;
use crate::physics::Player;
use crate::physics::Power;

use crate::proceduralgen;
use crate::proceduralgen::ProceduralGen;
use crate::proceduralgen::TerrainSegment;

use crate::rect;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;
use inf_runner::ObstacleType;
use inf_runner::PowerType;
use inf_runner::SDLCore;
use inf_runner::StaticObject;
use inf_runner::TerrainType;

use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

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
const TERRAIN_UPPER_BOUND: i32 = 2 * TILE_SIZE as i32;
const TERRAIN_LOWER_BOUND: i32 = CAM_H as i32 - TERRAIN_UPPER_BOUND;
const PLAYER_X: i32 = 2 * TILE_SIZE as i32;

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

        let tex_statue = texture_creator.load_texture("assets/obstacles/statue.png")?;
        let tex_balloon = texture_creator.load_texture("assets/obstacles/balloon.png")?;
        let tex_chest = texture_creator.load_texture("assets/obstacles/box.png")?;
        let tex_coin = texture_creator.load_texture("assets/obstacles/coin.png")?;
        let tex_powerup = texture_creator.load_texture("assets/obstacles/powerup.png")?;

        let tex_speed = texture_creator.load_texture("assets/powers/speed.png")?;
        let tex_multiplier = texture_creator.load_texture("assets/powers/multiplier.png")?;
        let tex_bouncy = texture_creator.load_texture("assets/powers/bouncy.png")?;
        let tex_floaty = texture_creator.load_texture("assets/powers/floaty.png")?;
        let tex_shield = texture_creator.load_texture("assets/powers/shield.png")?;

        let tex_player = texture_creator.load_texture("assets/player/player.png")?;
        let tex_shielded = texture_creator.load_texture("assets/player/shielded_player.png")?;
        let tex_winged = texture_creator.load_texture("assets/player/winged_player.png")?;
        let tex_springed = texture_creator.load_texture("assets/player/bouncy_player.png")?;
        let tex_fast = texture_creator.load_texture("assets/player/speed_player.png")?;
        let tex_rich = texture_creator.load_texture("assets/player/multiplier_player.png")?;

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

        let game_over_texture = texture_creator
            .create_texture_from_surface(
                &font
                    .render("GAME OVER")
                    .blended(Color::RGBA(255, 0, 0, 255))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;

        // Create player at default position
        let mut player = Player::new(
            rect!(
                PLAYER_X,
                TERRAIN_UPPER_BOUND, // + TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE
            ),
            3.0, // mass of player
            &tex_player,
        );

        let mut power_timer: i32 = 0; // Current powerup expires when it reaches 0
        let mut coin_timer: i32 = 0; // Timer to show +coin_value
        let mut last_coin_val: i32 = 0; // Last collected coin's value

        // Initialize ground / object vectors
        let mut all_terrain: Vec<TerrainSegment> = Vec::new();
        let mut all_obstacles: Vec<Obstacle> = Vec::new();
        let mut all_coins: Vec<Coin> = Vec::new();
        let mut all_powers: Vec<Power> = Vec::new(); // Refers to powers currently spawned on the
                                                     // ground, not active powers

        // Used to keep track of animation status
        let mut coin_anim: i32 = 0; // 60 frames of animation

        // Score of an entire run
        let mut total_score: i32 = 0;

        let mut game_paused: bool = false;
        let mut initial_pause: bool = false;
        let mut game_over: bool = false;

        // Number of frames to delay the end of the game by for demonstrating player
        // collision this should be removed once the camera tracks the player
        // properly
        let mut game_over_timer = 120;

        // FPS tracking
        let mut all_frames: i32 = 0;
        let mut last_raw_time;
        let mut last_measurement_time = Instant::now();

        // Used to transition to credits or back to title screen
        let mut next_status = GameStatus::Main;

        // Object spawning vars
        let mut spawn_timer: f64 = 500.0; // Can spawn a new object when it reaches 0

        /* ~~~~~~~~ Stuff for background sine waves ~~~~~~~~~~~~~~ */
        // Background & sine wave vars
        let mut bg_buff = 0;
        let mut bg_tick = 0;
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

        // Pre-Generate perlin curves for background hills
        for i in 0..BG_CURVES_SIZE {
            background_curves[IND_BACKGROUND_MID][i] =
                proceduralgen::gen_perlin_hill_point((i + buff_1), freq, amp_1, 0.5, 600.0);
            background_curves[IND_BACKGROUND_BACK][i] =
                proceduralgen::gen_perlin_hill_point((i + buff_2), freq, amp_2, 1.0, 820.0);
        }
        /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

        // Perlin Noise init
        let mut random: [[(i32, i32); 256]; 256] = [[(0, 0); 256]; 256];
        for i in 0..random.len() - 1 {
            for j in 0..random.len() - 1 {
                random[i][j] = (rng.gen_range(0..256), rng.gen_range(0..256));
            }
        }

        // Initialize the starting terrain segments
        // Rectangles
        let mut init_curve_1: Vec<(i32, i32)> = vec![(0, TERRAIN_LOWER_BOUND + TILE_SIZE as i32)];
        for i in 1..CAM_W {
            init_curve_1.push((i as i32, TERRAIN_LOWER_BOUND + TILE_SIZE as i32));
        }
        let cp_1 = [
            init_curve_1[0],
            init_curve_1[init_curve_1.len() / 3],
            init_curve_1[init_curve_1.len() * 2 / 3],
            init_curve_1[init_curve_1.len() - 1],
        ];
        let init_terrain_1 = TerrainSegment::new(
            rect!(0, TERRAIN_LOWER_BOUND + TILE_SIZE as i32, CAM_W, CAM_H),
            init_curve_1,
            0.0,
            TerrainType::Grass,
            Color::RGB(86, 125, 70),
            cp_1,
        );
        all_terrain.push(init_terrain_1);

        /* ~~~~~~ Main Game Loop ~~~~~~ */
        'gameloop: loop {
            last_raw_time = Instant::now(); // FPS tracking

            // Score collected in a single iteration of the game loop
            let mut curr_step_score: f64 = 0.0;

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
                            Keycode::Escape => {
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
                        Event::KeyUp {
                            keycode: Some(k), ..
                        } => match k {
                            Keycode::Space => {
                                game_paused = false;
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
                    game_over_timer -= 1; // Animation buffer
                    if game_over_timer == 0 {
                        break 'gameloop;
                    }
                }

                //  Get ground point at player and TILE_SIZE ahead of player
                let left_ground_point: Point = get_ground_coord(&all_terrain, PLAYER_X); // left of player
                let curr_ground_point: Point =
                    get_ground_coord(&all_terrain, PLAYER_X + (TILE_SIZE as i32) / 2); // middle of player
                let right_ground_point: Point =
                    get_ground_coord(&all_terrain, PLAYER_X + TILE_SIZE as i32); // right of player
                let angle = ((right_ground_point.y() as f64 - left_ground_point.y() as f64)
                    / (TILE_SIZE as f64))
                    .atan(); // slope between left and right of player

                /* ~~~~~~ Handle Input ~~~~~~ */
                let keypress_moment: SystemTime = SystemTime::now();
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
                                    if !player.jumpmoment_lock() {
                                        let keypress_moment = SystemTime::now();
                                        player.set_jumpmoment(keypress_moment);
                                    }
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
                                let jump_moment: SystemTime = player.jump_moment();
                                player.jump(
                                    curr_ground_point,
                                    SystemTime::now().duration_since(jump_moment).unwrap(),
                                );
                                player.stop_flipping();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                //Power handling
                if power_timer == 0 {
                    power_timer -= 1;
                    player.set_power_up(None, &tex_shield); // Texture doesn't matter as power-up is None
                } else if power_timer > 0 {
                    power_timer -= 1;
                }

                // Apply bouncy shoes, if applicable
                // Effectively just repeated jumps, independent of player input
                if let Some(PowerType::BouncyShoes) = player.power_up() {
                    if !player.is_jumping() {
                        player.jump(curr_ground_point, Duration::new(1111, 0));
                    }
                }

                /* ~~~~~~ Handle Player Collisions ~~~~~~ */

                // If the player doesn't land on ther feet, end game
                if !Physics::check_player_upright(&player, angle, curr_ground_point) {
                    game_over = true;
                }

                // Check through all collisions with obstacles
                // End game if crash occurs
                for o in all_obstacles.iter_mut() {
                    if Physics::check_collision(&mut player, o) {
                        if player.collide_obstacle(o) {
                            game_over = true;
                        }
                    }
                }

                // Check for coin collection
                // Add to score if collected
                // Remove coins if player collects them
                let mut to_remove_ind: i32 = -1;
                let mut counter = 0;
                for c in all_coins.iter_mut() {
                    if Physics::check_collision(&mut player, c) {
                        if player.collide_coin(c) {
                            to_remove_ind = counter;
                            curr_step_score += c.value() as f64; //increments the
                                                                 // score based on the
                                                                 // coins value

                            last_coin_val = c.value();
                            coin_timer = 60; // Time to show last_coin_val on
                                             // screen
                        }
                        continue;
                    }
                    counter += 1;
                }
                if to_remove_ind != -1 {
                    all_coins.remove(to_remove_ind as usize);
                }

                // Check for powerup pickups
                // Apply to player and begin countdown if picked up
                let mut to_remove_ind: i32 = -1;
                let mut counter = 0;
                for p in all_powers.iter_mut() {
                    if Physics::check_collision(&mut player, p) {
                        // Get associated powerup for given p.power_type()
                        let p_tex = match (Some(p.power_type())) {
                            Some(PowerType::SpeedBoost) => &tex_speed,
                            Some(PowerType::ScoreMultiplier) => &tex_multiplier,
                            Some(PowerType::BouncyShoes) => &tex_bouncy,
                            Some(PowerType::LowerGravity) => &tex_floaty,
                            Some(PowerType::Shield) => &tex_shield,
                            _ => &tex_shield, // Default needed but None should never happen here
                        };
                        if player.collide_power(p, p_tex) {
                            to_remove_ind = counter;
                            power_timer = 360;
                        }
                        continue;
                    }
                    counter += 1;
                }
                if to_remove_ind != -1 {
                    all_powers.remove(to_remove_ind as usize);
                }

                /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

                /* ~~~~~~ Handle Forces from Physics and move sprites ~~~~~~ */

                // Apply forces on player
                let current_power = player.power_up();
                let curr_terrain_type = get_ground_type(&all_terrain, PLAYER_X); //for physics

                Physics::apply_terrain_forces(
                    // Gravity, normal, and friction
                    &mut player,
                    angle,
                    curr_ground_point,
                    curr_terrain_type,
                    current_power,
                );
                if !game_over {
                    // Propel forward
                    Physics::apply_skate_force(&mut player, angle, curr_ground_point);
                }
                //update player attributes
                player.update_vel(game_over);
                player.update_pos(curr_ground_point, angle, game_over);
                player.flip();

                //DEBUG PLAYER (Plz dont delete, just comment out)
                //println!("A-> vx:{} ax:{}, vy:{}
                // ay:{}",player.vel_x(),player.accel_x(),player.vel_y(),player.accel_y());

                player.reset_accel();

                //DEBUG PLAYER (Plz dont delete, just comment out)
                //println!("B-> vx:{} ax:{}, vy:{}
                // ay:{}",player.vel_x(),player.vel_y(),player.accel_x(),player.accel_y());

                // apply forces to obstacles
                for o in all_obstacles.iter_mut() {
                    // Only actually apply forces after a collision occurs
                    if o.collided() {
                        let object_ground =
                            get_ground_coord(&all_terrain, o.x() + (TILE_SIZE as i32) / 2);
                        let object_terrain_type = get_ground_type(&all_terrain, o.x());
                        // Very small friction coefficient because there's no
                        // "skate force" to counteract friction
                        Physics::apply_terrain_forces(
                            o,
                            angle,
                            object_ground,
                            object_terrain_type,
                            None,
                        );
                        o.update_vel(false);
                        o.update_pos(object_ground, angle, game_over);
                    }
                }

                /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

                // Generate new terrain / objects if player hasn't died
                if !game_over {
                    /* ~~~~~~ Object Generation ~~~~~~ */

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

                    // Value spawn_timer is reset to upon spawning an object.
                    // Decreases to increase spawn rates based on total_score.
                    // These numbers could be terrible, we should mess around with it
                    let min_spawn_gap = if total_score > 100000 {
                        300 // Cap
                    } else if total_score > 50000 {
                        350
                    } else if total_score > 40000 {
                        400
                    } else if total_score > 30000 {
                        450
                    } else if total_score > 20000 {
                        500
                    } else if total_score > 15000 {
                        625
                    } else if total_score > 10000 {
                        550
                    } else if total_score > 7500 {
                        600
                    } else if total_score > 5000 {
                        650
                    } else if total_score > 2500 {
                        700
                    } else {
                        750 // Default
                    };

                    // Choose new object to generate
                    let mut new_object: Option<StaticObject> = None;
                    let curr_num_objects = all_obstacles.len() + all_coins.len() + all_powers.len();
                    let spawn_trigger = rng.gen_range(0..MAX_NUM_OBJECTS);

                    if spawn_timer > 0.0 {
                        spawn_timer -= player.vel_x() / 2.5;
                    } else if spawn_trigger >= curr_num_objects as i32 {
                        new_object = Some(proceduralgen::choose_static_object());
                        spawn_timer = min_spawn_gap as f64;
                    } else if spawn_trigger < curr_num_objects as i32 {
                        // Min spawn gap can be replaced with basically any value for this random
                        // range. Smaller values will spawn objects more often
                        spawn_timer = rng.gen_range(0.0..min_spawn_gap as f64);
                    }

                    // Spawn new object
                    match new_object {
                        Some(StaticObject::Statue) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let obstacle = Obstacle::new(
                                rect!(
                                    spawn_coord.x - (TILE_SIZE as i32) / 2,
                                    spawn_coord.y - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                ),
                                50.0, // mass
                                &tex_statue,
                                ObstacleType::Statue,
                            );
                            all_obstacles.push(obstacle);
                        }
                        Some(StaticObject::Balloon) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let obstacle = Obstacle::new(
                                rect!(
                                    spawn_coord.x - (TILE_SIZE as i32) / 2,
                                    spawn_coord.y - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                ),
                                1.0,
                                &tex_balloon,
                                ObstacleType::Balloon,
                            );
                            all_obstacles.push(obstacle);
                        }
                        Some(StaticObject::Chest) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let obstacle = Obstacle::new(
                                rect!(
                                    spawn_coord.x - (TILE_SIZE as i32) / 2,
                                    spawn_coord.y - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                ),
                                50.0,
                                &tex_chest,
                                ObstacleType::Chest,
                            );
                            all_obstacles.push(obstacle);
                        }
                        Some(StaticObject::Coin) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let coin = Coin::new(
                                rect!(
                                    spawn_coord.x - (TILE_SIZE as i32) / 2,
                                    spawn_coord.y - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                ),
                                &tex_coin,
                                1000, // value
                            );
                            all_coins.push(coin);
                        }
                        Some(StaticObject::Power) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let pow = Power::new(
                                rect!(
                                    spawn_coord.x - (TILE_SIZE as i32) / 2,
                                    spawn_coord.y - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                ),
                                &tex_powerup,
                                proceduralgen::choose_power_up(),
                            );
                            all_powers.push(pow);
                        }
                        // Some(StaticObject::Chest) => {}
                        // ... Add any new types of objects here ...
                        _ => {}
                    }

                    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
                }

                // Update total_score
                // Poorly placed rn, should be after postion / hitbox / collision update
                // but before drawing
                if !game_over {
                    curr_step_score += (player.vel_x() / 1.5); // Increase score by factor of ammount moved that frame
                    if let Some(PowerType::ScoreMultiplier) = player.power_up() {
                        curr_step_score *= 2.0; // Hardcoded power bonus
                        last_coin_val = 2000;
                    }
                    total_score += curr_step_score as i32;
                }

                /* Update ground / object positions to move player forward
                 * by the distance they should move this single iteration of the game loop
                 */
                let travel_update = player.vel_x();
                for ground in all_terrain.iter_mut() {
                    ground.travel_update(travel_update as i32);
                }

                for obs in all_obstacles.iter_mut() {
                    obs.travel_update(travel_update as i32);
                }
                for coin in all_coins.iter_mut() {
                    coin.travel_update(travel_update as i32);
                }
                for power_up in all_powers.iter_mut() {
                    power_up.travel_update(travel_update as i32);
                }

                // Generate new ground when the last segment becomes visible
                let last_seg = all_terrain.get(all_terrain.len() - 1).unwrap();
                if last_seg.x() < CAM_W as i32 {
                    let new_terrain = proceduralgen::ProceduralGen::gen_terrain(
                        &random,
                        &last_seg,
                        CAM_W as i32,
                        CAM_H as i32,
                        false,
                        false,
                        false,
                    );
                    all_terrain.push(new_terrain);
                }

                /* ~~~~~~ Begin Camera Section ~~~~~~ */
                /* This should be the very last section of calcultions,
                 * as the camera position relies upon updated math for
                 * EVERYTHING ELSE. Below the camera section we have
                 * removal of offscreen objects from their vectors,
                 * animation updates, the drawing section, and FPS calculation only.
                 */

                // Adjust camera vertically based on y/height of the ground
                let camera_adj_y = if curr_ground_point.y() < TERRAIN_UPPER_BOUND {
                    TERRAIN_UPPER_BOUND - curr_ground_point.y()
                } else if (curr_ground_point.y() + TILE_SIZE as i32) > TERRAIN_LOWER_BOUND {
                    TERRAIN_LOWER_BOUND - curr_ground_point.y()
                } else {
                    0
                };

                // Add adjustment to terrain
                for ground in all_terrain.iter_mut() {
                    ground.camera_adj(0, camera_adj_y);
                }

                // Add adjustment to obstacles
                for obs in all_obstacles.iter_mut() {
                    obs.camera_adj(0, camera_adj_y);
                }

                // Add adjustment to coins
                for coin in all_coins.iter_mut() {
                    coin.camera_adj(0, camera_adj_y);
                }

                // Add adjustment to power ups
                for power_up in all_powers.iter_mut() {
                    power_up.camera_adj(0, camera_adj_y);
                }

                // Add adjustment to player
                player.camera_adj(0, camera_adj_y);
                /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

                /* ~~~~~~ Remove stuff which is now offscreen ~~~~~~ */
                let mut remove_inds: Vec<i32> = Vec::new();
                let mut ind: i32 = -1;

                // Terrain
                for ground in all_terrain.iter() {
                    ind += 1;
                    if ground.x() + ground.w() <= -1 * TILE_SIZE as i32 {
                        remove_inds.push(ind);
                    }
                }
                for i in remove_inds.iter() {
                    all_terrain.remove(*i as usize);
                }
                remove_inds.clear();

                //  Obstacles
                ind = -1;
                for obs in all_obstacles.iter() {
                    ind += 1;
                    if obs.x() + TILE_SIZE as i32 <= -1 * TILE_SIZE as i32 {
                        remove_inds.push(ind);
                    }
                }
                for i in remove_inds.iter() {
                    all_obstacles.remove(*i as usize);
                }
                remove_inds.clear();

                // Coins
                ind = -1;
                for coin in all_coins.iter() {
                    ind += 1;
                    if coin.x() + TILE_SIZE as i32 <= -1 * TILE_SIZE as i32 {
                        remove_inds.push(ind);
                    }
                }
                for i in remove_inds.iter() {
                    all_coins.remove(*i as usize);
                }
                remove_inds.clear();

                // Power ups
                ind = -1;
                for power in all_powers.iter_mut() {
                    ind += 1;
                    if power.x() + TILE_SIZE as i32 <= -1 * TILE_SIZE as i32 {
                        remove_inds.push(ind);
                    }
                }
                for i in remove_inds.iter() {
                    all_powers.remove(*i as usize);
                }
                /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

                /* ~~~~~~ Animation Updates ~~~~~~ */
                bg_tick += 1;

                // Shift background images & sine waves?
                if bg_tick % 10 == 0 {
                    bg_buff -= 1;
                }

                // Reset sine wave tick (to prevent large values?)
                if bg_tick % 3 == 0 && bg_tick % 5 == 0 {
                    bg_tick = 0;
                }

                // Reset background image buffer upon leftmost bg image moving completely
                // offscreen
                if -bg_buff == CAM_W as i32 {
                    bg_buff = 0;
                }

                // Next frame for coin animation
                coin_anim += 1;
                coin_anim %= 60;
                /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

                /* ~~~~~~ Draw All Elements ~~~~~~ */
                // Wipe screen every frame
                core.wincan.set_draw_color(Color::RGBA(3, 120, 206, 255));
                core.wincan.clear();

                // Bottom layer of background, black skybox
                core.wincan.set_draw_color(Color::RGBA(0, 0, 0, 255));
                core.wincan.fill_rect(rect!(0, 470, CAM_W, CAM_H))?;

                // Sky
                core.wincan
                    .copy(&tex_sky, None, rect!(bg_buff, 0, CAM_W, CAM_H / 3))?;
                core.wincan.copy(
                    &tex_sky,
                    None,
                    rect!(CAM_W as i32 + bg_buff, 0, CAM_W, CAM_H / 3),
                )?;

                // Sunset gradient - doesn't need to scroll left
                core.wincan
                    .copy(&tex_grad, None, rect!(0, -128, CAM_W, CAM_H))?;

                // Background
                core.wincan
                    .copy(&tex_bg, None, rect!(bg_buff, -150, CAM_W, CAM_H))?;
                core.wincan.copy(
                    &tex_bg,
                    None,
                    rect!(bg_buff + (CAM_W as i32), -150, CAM_W, CAM_H),
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

                // Active Power HUD Display
                if player.power_up().is_some() {
                    core.wincan.copy(
                        player.power_up_tex(),
                        None,
                        rect!(10, 100, TILE_SIZE, TILE_SIZE),
                    )?;

                    // Power duration bar
                    let m = power_timer as f64 / 360.0;
                    let r = 256.0 * (1.0 - m);
                    let g = 256.0 * (m);
                    let w = TILE_SIZE as f64 * m;
                    core.wincan.set_draw_color(Color::RGB(r as u8, g as u8, 0));
                    core.wincan.fill_rect(rect!(10, 210, w as u8, 10))?;
                }

                // Terrain
                for ground_seg in all_terrain.iter() {
                    let curve = ground_seg.curve();
                    for curve_ind in 0..ground_seg.w() {
                        // Get Draw Coords
                        let slice_x = curve[curve_ind as usize].0;
                        let slice_y = curve[curve_ind as usize].1;

                        // Don't draw in negative x
                        if slice_x < 0 {
                            continue;
                        }
                        // Stop drawing at CAM_W
                        else if slice_x >= CAM_W as i32 {
                            break;
                        }
                        // Normal drawing
                        else {
                            core.wincan.set_draw_color(ground_seg.color());
                            core.wincan.fill_rect(rect!(
                                slice_x,
                                slice_y,
                                1,
                                CAM_H as i32 - slice_y
                            ))?;
                        }
                    }
                }

                // Obstacles
                for obs in all_obstacles.iter() {
                    // Collapse this match to just one ... all this code is repeated
                    match obs.obstacle_type() {
                        ObstacleType::Statue => {
                            core.wincan.copy_ex(
                                obs.texture(),
                                None,
                                rect!(obs.x(), obs.y(), TILE_SIZE, TILE_SIZE),
                                obs.theta(),
                                None,
                                false,
                                false,
                            )?;
                            core.wincan.set_draw_color(Color::RED);
                            core.wincan.draw_rect(obs.hitbox())?;
                            break;
                        }
                        ObstacleType::Balloon => {
                            core.wincan.copy_ex(
                                obs.texture(),
                                None,
                                rect!(obs.x(), obs.y(), TILE_SIZE, TILE_SIZE),
                                obs.theta(),
                                None,
                                false,
                                false,
                            )?;
                            core.wincan.set_draw_color(Color::BLUE);
                            core.wincan.draw_rect(obs.hitbox())?;
                        }
                        ObstacleType::Chest => {
                            core.wincan.copy_ex(
                                obs.texture(),
                                None,
                                rect!(obs.x(), obs.y(), TILE_SIZE, TILE_SIZE),
                                obs.theta(),
                                None,
                                false,
                                false,
                            )?;
                            core.wincan.set_draw_color(Color::BLUE);
                            core.wincan.draw_rect(obs.hitbox())?;
                        }
                    }
                }

                // Coins
                for coin in all_coins.iter() {
                    core.wincan.copy_ex(
                        coin.texture(),
                        rect!(coin_anim * TILE_SIZE as i32, 0, TILE_SIZE, TILE_SIZE),
                        rect!(coin.x(), coin.y(), TILE_SIZE, TILE_SIZE),
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                    core.wincan.set_draw_color(Color::GREEN);
                    core.wincan.draw_rect(coin.hitbox())?;
                }

                // Powerups (on the ground, not active or collected)
                for power in all_powers.iter() {
                    core.wincan.copy_ex(
                        power.texture(),
                        rect!(0, 0, TILE_SIZE, TILE_SIZE),
                        rect!(power.x(), power.y(), TILE_SIZE, TILE_SIZE),
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                    core.wincan.set_draw_color(Color::YELLOW);
                    core.wincan.draw_rect(power.hitbox())?;
                }

                // Set player texture
                let tex_player = match player.power_up() {
                    Some(PowerType::Shield) => &tex_shielded,
                    Some(PowerType::LowerGravity) => &tex_winged,
                    Some(PowerType::BouncyShoes) => &tex_springed,
                    Some(PowerType::SpeedBoost) => &tex_fast,
                    Some(PowerType::ScoreMultiplier) => &tex_rich,
                    // ... Add more types of powered player textures here ...
                    _ => player.texture(),
                };

                // Assert player.x() == PLAYER_X here

                // Player
                core.wincan.copy_ex(
                    tex_player,
                    rect!(0, 0, TILE_SIZE, TILE_SIZE),
                    rect!(player.x(), player.y(), TILE_SIZE, TILE_SIZE),
                    player.theta() * 180.0 / std::f64::consts::PI,
                    None,
                    false,
                    false,
                )?;

                core.wincan.set_draw_color(Color::BLACK);

                // Player's hitbox
                core.wincan.draw_rect(player.hitbox())?;

                // Setup for the text of the total_score to be displayed
                let tex_score = font
                    .render(&format!("{:08}", total_score))
                    .blended(Color::RGBA(255, 0, 0, 100))
                    .map_err(|e| e.to_string())?;

                // Display total_score
                let tex_score = texture_creator
                    .create_texture_from_surface(&tex_score)
                    .map_err(|e| e.to_string())?;
                core.wincan
                    .copy(&tex_score, None, Some(rect!(10, 10, 100, 50)))?;

                // Display added coin value when coin is collected
                let coin_surface = font
                    .render(&format!("   +{:04}", last_coin_val))
                    .blended(Color::RGBA(100, 0, 200, 100))
                    .map_err(|e| e.to_string())?;
                let tex_coin_val = texture_creator
                    .create_texture_from_surface(&coin_surface)
                    .map_err(|e| e.to_string())?;

                // Only show right after collecting a coin
                if coin_timer > 0 {
                    core.wincan
                        .copy(&tex_coin_val, None, Some(rect!(10, 50, 100, 50)))?;
                    coin_timer -= 1;
                }

                if game_over {
                    // Cleaned up calculation of texture position
                    // Check previous versions if you want those calculations
                    core.wincan
                        .copy(&game_over_texture, None, Some(rect!(239, 285, 801, 149)))?;
                }

                core.wincan.present();
                /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

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
                    //println!("{} FPS", all_frames);
                    // println!(
                    //     "Average FPS: {:.2}",
                    //     (all_frames as f64) / time_since_last_measurement.as_secs_f64()
                    // );
                    all_frames = 0;
                    last_measurement_time = Instant::now();
                }
                /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
            }

            /* ~~~~~~ Helper Functions ~~~~~ */
            // Given the current terrain and an x coordinate of the screen,
            // returns the (x, y) of the ground at that x
            fn get_ground_coord(all_terrain: &Vec<TerrainSegment>, screen_x: i32) -> Point {
                // Loop backwards
                for ground in all_terrain.iter().rev() {
                    // The first segment starting at or behind
                    // the given x, which it must be above
                    if ground.x() <= screen_x {
                        let point_ind: usize = (screen_x - ground.x()) as usize;
                        if point_ind >= ground.curve().len() {
                            println!("{:?} {:?}", ground.x(), screen_x);
                        }
                        return Point::new(
                            ground.curve().get(point_ind).unwrap().0,
                            ground.curve().get(point_ind).unwrap().1,
                        );
                    }
                }
                return Point::new(-1, -1);
            }
            // Given the current terrain and an x coordinate of the screen,
            // returns the (x, y) of the ground at that x
            fn get_ground_type(all_terrain: &Vec<TerrainSegment>, screen_x: i32) -> &TerrainType {
                // Loop backwards
                for ground in all_terrain.iter().rev() {
                    // The first segment starting at or behind
                    // the given x, which it must be above
                    if ground.x() <= screen_x {
                        let point_ind: usize = (screen_x - ground.x()) as usize;
                        return ground.get_type();
                    }
                }
                return &TerrainType::Grass; //default to grass
            }
            /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
        } // End gameloop

        Ok(GameState {
            status: Some(next_status),
            score: total_score,
        })
    } // End run fn
} // End impl
