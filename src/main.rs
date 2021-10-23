// University of Pittsburgh
// CS 1666 - Fall 2021
// Infinite Runner
mod credits;
mod runner;
mod title;
mod utils;
// mod physics;
mod proceduralgen;

use inf_runner::Game;
use inf_runner::GameState;
use inf_runner::GameStatus;

const TITLE: &str = "Urban Odyssey";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

// A container for all the segments of our game
pub struct UrbanOdyssey {
    core: inf_runner::SDLCore,
    title: title::Title,
    runner: runner::Runner,
    credits: credits::Credits,
    /* physics?
     * procedural generation? */
}

fn main() {
    println!("\nRunning {}:", TITLE);
    print!("\tInitting...");

    // Init all segments, wrap into one UrbanOdyssey
    let game = init();
    match game {
        Err(e) => println!("\n\t\tFailed to init: {}", e),
        Ok(mut contents) => {
            println!("DONE");

            // Run all segments one-by-one using contents.segment.run(&mut (contents.core),
            // ...)      [Perhaps this will make less sense in the future if the
            // segments switch      back and forth between each other, but this
            // is just a starting point]

            let mut game_manager = GameState {
                status: Some(GameStatus::Main),
                score: 0,
            };

            loop {
                match game_manager.status {
                    Some(GameStatus::Main) => {
                        println!("\nRunning Title Sequence:");
                        print!("\tRunning...");

                        // TITLE SCREEN RUN
                        match contents.title.run(&mut (contents.core)) {
                            Err(e) => println!("\n\t\tEncountered error while running: {}", e),
                            Ok(title_status) => {
                                game_manager = title_status;
                                println!("DONE\nExiting cleanly");
                            }
                        };
                    }
                    Some(GameStatus::Game) => {
                        println!("\nRunning Game Sequence:");
                        print!("\tRunning...");

                        //GAME PLAY RUN
                        match contents.runner.run(&mut (contents.core)) {
                            Err(e) => println!("\n\t\tEncountered error while running: {}", e),
                            Ok(game_status) => {
                                game_manager = game_status;
                                println!("DONE\nExiting cleanly");
                            }
                        };
                    }
                    Some(GameStatus::Credits) => {
                        println!("\nRunning Credits Sequence:");
                        print!("\tRunning...");

                        // CREDITS RUN

                        // Ownership is tough ... maybe there's a smarter way to do this
                        // using smart pointers, but for now, looks like we'll be passing
                        // around the SDLCore to each segment manually.
                        match contents.credits.run(&mut (contents.core)) {
                            Err(e) => println!("\n\t\tEncountered error while running: {}", e),
                            Ok(credits_status) => {
                                game_manager = credits_status;
                                println!("DONE\nExiting cleanly");
                            }
                        };
                    }
                    None => {
                        break;
                    }
                };
            }
        }
    };
}

fn init() -> Result<UrbanOdyssey, String> {
    let core = inf_runner::SDLCore::init(TITLE, true, CAM_W, CAM_H)?;

    let title = title::Title::init()?;
    let runner = runner::Runner::init()?;
    let credits = credits::Credits::init()?;
    // physics?
    // procedural generation?

    Ok(UrbanOdyssey {
        core,
        title,
        runner,
        credits,
    })
}
