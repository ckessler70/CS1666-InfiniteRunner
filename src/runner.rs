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
        let tex_ballon = texture_creator.load_texture("assets/obstacles/balloon.png")?;
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
                TERRAIN_UPPER_BOUND + TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE
            ),
            3.0, // mass of player
            &tex_player,
        );

        let mut power_timer: i32 = 0; // Current powerup expires when it reaches 0
        let mut coin_count: i32 = 0; // Total num coins collected

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

        // let mut test_stepper = 0;

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
        let mut spawn_timer: i32 = 500; // Can spawn a new object when it reaches 0

        //For Bezier Curves
        let mut prev_P3: (i32, i32) = (-1, -1);
        let mut prev_P2: (i32, i32) = (-1, -1);

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
        let mut init_curve_1: Vec<(i32, i32)> = vec![(0, CAM_H as i32 * 2 / 3)];
        for i in 1..CAM_W {
            init_curve_1.push((i as i32, CAM_H as i32 * 2 / 3));
        }
        let init_terrain_1 = TerrainSegment::new(
            rect!(0, CAM_H as i32 * 2 / 3, CAM_W, CAM_H as i32 * 2 / 3),
            init_curve_1,
            0.0,
            TerrainType::Grass,
            Color::GREEN,
            (-1, -1),
            (-1, -1),
        );
        let mut init_curve_2: Vec<(i32, i32)> = vec![(CAM_W as i32, CAM_H as i32 * 2 / 3)];
        for i in (CAM_W + 1)..(CAM_W * 2) {
            init_curve_2.push((i as i32, CAM_H as i32 * 2 / 3));
        }
        let init_terrain_2 = TerrainSegment::new(
            rect!(CAM_W, CAM_H as i32 * 2 / 3, CAM_W, CAM_H as i32 * 2 / 3),
            init_curve_2,
            0.0,
            TerrainType::Grass,
            Color::BLUE,
            (-1, -1),
            (-1, -1),
        );
        all_terrain.push(init_terrain_1);
        all_terrain.push(init_terrain_2);

        /* ~~~~~~ Main Game Loop ~~~~~~ */
        'gameloop: loop {
            last_raw_time = Instant::now(); // FPS tracking

            // Score collected in a single iteration of the game loop
            let mut curr_step_score: i32 = 0;

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
                let curr_ground_point: Point = get_ground_coord(&all_terrain, PLAYER_X);
                let next_ground_point: Point =
                    get_ground_coord(&all_terrain, PLAYER_X + TILE_SIZE as i32);
                let angle = ((next_ground_point.y() as f64 - curr_ground_point.y() as f64)
                    / (TILE_SIZE as f64))
                    .atan();

                /* ~~~~~~ Handle Input ~~~~~~ */
                let mut keypress_moment: SystemTime = SystemTime::now();
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
                                        keypress_moment = SystemTime::now();
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
                                let mut jump_moment: SystemTime = player.jump_moment();
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
                    player.set_power_up(None);
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
                            coin_count += 1;
                            curr_step_score += c.value(); //increments the
                                                          // score based on the
                                                          // coins value
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
                        if player.collide_power(p) {
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
                Physics::apply_skate_force(&mut player, angle, curr_ground_point); // Propel forward

                //update player attributes
                player.update_vel(game_over);
                player.update_pos(curr_ground_point, angle, game_over);
                player.flip();
                player.reset_accel();

                // apply forces to obstacles
                for o in all_obstacles.iter_mut() {
                    // Only actually apply forces after a collision occurs
                    if o.collided() {
                        let object_ground = get_ground_coord(&all_terrain, o.x());
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
                    } else if total_score > 90000 {
                        320
                    } else if total_score > 80000 {
                        340
                    } else if total_score > 70000 {
                        360
                    } else if total_score > 60000 {
                        380
                    } else if total_score > 50000 {
                        400
                    } else if total_score > 40000 {
                        420
                    } else if total_score > 30000 {
                        440
                    } else if total_score > 20000 {
                        460
                    } else if total_score > 10000 {
                        480
                    } else {
                        500 // Default
                    };

                    // Choose new object to generate
                    let mut new_object: Option<StaticObject> = None;
                    let curr_num_objects = all_obstacles.len() + all_coins.len() + all_powers.len();
                    let spawn_trigger = rng.gen_range(0..MAX_NUM_OBJECTS);

                    if spawn_timer > 0 {
                        spawn_timer -= 1;
                    } else if spawn_trigger >= curr_num_objects as i32 {
                        new_object = Some(proceduralgen::choose_static_object());
                        spawn_timer = min_spawn_gap;
                    } else if spawn_trigger < curr_num_objects as i32 {
                        // Min spawn gap can be replaced with basically any value for this random
                        // range. Smaller values will spawn objects more often
                        spawn_timer = rng.gen_range(0..min_spawn_gap);
                    }

                    // Spawn new object
                    match new_object {
                        Some(StaticObject::Statue) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let obstacle = Obstacle::new(
                                rect!(
                                    spawn_coord.x,
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
                        Some(StaticObject::Spring) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let obstacle = Obstacle::new(
                                rect!(
                                    spawn_coord.x,
                                    spawn_coord.y - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                ),
                                1.0,
                                &tex_ballon,
                                ObstacleType::Spring,
                            );
                            all_obstacles.push(obstacle);
                        }
                        Some(StaticObject::Chest) => {
                            let spawn_coord: Point =
                                get_ground_coord(&all_terrain, (CAM_W as i32) - 1);
                            let obstacle = Obstacle::new(
                                rect!(
                                    spawn_coord.x,
                                    spawn_coord.y - TILE_SIZE as i32,
                                    TILE_SIZE,
                                    TILE_SIZE
                                ),
                                1.0,
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
                                    spawn_coord.x,
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
                                    spawn_coord.x,
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
                    curr_step_score += 1; // Hardcoded score increase per frame
                    if let Some(PowerType::ScoreMultiplier) = player.power_up() {
                        curr_step_score *= 2; // Hardcoded power bonus
                    }
                    total_score += curr_step_score;
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
                // All of this code is placeholder

                /*
                let last_seg = all_terrain.get(all_terrain.len() - 1).unwrap();
                if last_seg.x() < CAM_W as i32 {
                    let last_x = last_seg.curve().get(last_seg.curve().len() - 1).unwrap().0;
                    let last_y = last_seg.curve().get(last_seg.curve().len() - 1).unwrap().1;
                    let mut new_curve: Vec<(i32, i32)> = vec![(last_x + 1, last_y)];

                    let mut tempa: (i32, i32) = last_seg.get_p2();
                    let mut tempb: (i32, i32) = last_seg.get_p3();

                    println!("Prev P2 is: {},{}", tempa.0, tempa.1);
                    println!("Prev P3 is: {},{}", tempb.0, tempb.1);

                    for i in (last_x + 2)..(last_x + CAM_W as i32 + 1) {
                        new_curve.push((i as i32, last_y));
                    }
                    let new_terrain = TerrainSegment::new(
                        rect!(last_x + 1, last_y, CAM_W, CAM_H * 2 / 3),
                        new_curve,
                        0.0,
                        TerrainType::Grass,
                        Color::GREEN,
                        last_seg.get_p2(),
                        last_seg.get_p3(),
                    );
                    all_terrain.push(new_terrain);
                }
                */

                //Generate Control points
                let mut points: Vec<(i32, i32)> = proceduralgen::gen_control_points(
                    (prev_P3.0 as f64, prev_P3.1 as f64),
                    &random,
                    CAM_W as i32,
                    CAM_H as i32,
                    100,
                );

                //Sloppy implementation of ensuring the control points will work for smooth
                // curves. Will make better next week.
                if (prev_P2.0 > 0) {
                    points[1] = prev_P2;
                    while (points[1].0 > points[2].0 || points[2].0 > points[3].0) {
                        let temp: i32 = rng.gen::<i32>() * 25 + 25; //0-50
                        if (points[1].0 > points[2].0) {
                            points[2].0 += temp;
                        } else if (points[2].0 > points[3].0) {
                            points[2].0 -= temp;
                        }
                    }
                }

                //input prevp3, prevp2, currp2, currp3
                let mut curvePoints: Vec<(i32, i32)> = proceduralgen::extend_cubic_bezier_curve(
                    (prev_P3.0 as f64, prev_P3.1 as f64),
                    (prev_P2.0 as f64, prev_P2.1 as f64),
                    (points[2].0 as f64, points[2].1 as f64),
                    (points[3].0 as f64, points[3].1 as f64),
                );

                prev_P2 = points[2];
                prev_P3 = points[3];

                //curvepoints is the curve.

                //Now that

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
                    match player.power_up() {
                        Some(PowerType::SpeedBoost) => {
                            core.wincan.copy(
                                &tex_speed,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(PowerType::ScoreMultiplier) => {
                            core.wincan.copy(
                                &tex_multiplier,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(PowerType::BouncyShoes) => {
                            core.wincan.copy(
                                &tex_bouncy,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(PowerType::LowerGravity) => {
                            core.wincan.copy(
                                &tex_floaty,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        Some(PowerType::Shield) => {
                            core.wincan.copy(
                                &tex_shield,
                                None,
                                rect!(10, 100, TILE_SIZE, TILE_SIZE),
                            )?;
                        }
                        _ => {}
                    }

                    // Power duration bar
                    let m = power_timer as f64 / 360.0;
                    let r = 256.0 * (1.0 - m);
                    let g = 256.0 * (m);
                    let w = TILE_SIZE as f64 * m;
                    core.wincan.set_draw_color(Color::RGB(r as u8, g as u8, 0));
                    core.wincan.fill_rect(rect!(10, 210, w as u8, 10))?;
                }

                // Terrain
                for ground in all_terrain.iter() {
                    core.wincan.set_draw_color(ground.color());
                    core.wincan.fill_rect(ground.pos())?;
                }

                // Set player texture
                let tex_player = match player.power_up() {
                    Some(PowerType::Shield) => &tex_shielded,
                    Some(PowerType::LowerGravity) => &tex_winged,
                    Some(PowerType::BouncyShoes) => &tex_springed,
                    Some(PowerType::SpeedBoost) => &tex_fast,
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
                        ObstacleType::Spring => {
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

                // Setup for the text of the total_score to be displayed
                let tex_score = font
                    .render(&format!("{:08}", total_score))
                    .blended(Color::RGBA(255, 0, 0, 100))
                    .map_err(|e| e.to_string())?;

                // Display total_score
                let score_texture = texture_creator
                    .create_texture_from_surface(&tex_score)
                    .map_err(|e| e.to_string())?;
                core.wincan
                    .copy(&score_texture, None, Some(rect!(10, 10, 100, 50)))?;

                // Display num coins collected
                let coin_surface = font
                    .render(&format!("{:03}", coin_count))
                    .blended(Color::RGBA(100, 0, 200, 100))
                    .map_err(|e| e.to_string())?;
                let coin_count_texture = texture_creator
                    .create_texture_from_surface(&coin_surface)
                    .map_err(|e| e.to_string())?;
                core.wincan
                    .copy(&coin_count_texture, None, Some(rect!(160, 10, 80, 50)))?;

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
