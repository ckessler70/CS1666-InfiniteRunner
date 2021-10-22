// University of Pittsburgh
// CS 1666 - Fall 2021
// Infinite Runner

mod credits;
mod demo;
mod utils;
// mod physics;
// mod proc_gen;

use inf_runner::Game;

const TITLE: &str = "Urban Odyssey";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

// A container for all the segments of our game
pub struct UrbanOdyssey {
    core: inf_runner::SDLCore,
    // title,
    demo: demo::Demo,
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

            print!("\tRunning...");
            // Run all segments one-by-one using contents.segment.run(&mut (contents.core),
            // ...)      [Perhaps this will make less sense in the future if the
            // segments switch      back and forth between each other, but this
            // is just a starting point]

            // TITLE SCREEN RUN
            // GAME PLAY RUN
            loop {
                match contents.demo.run(&mut (contents.core)) {
                    Err(e) => println!("\n\t\tEncountered error while running: {}", e),
                    Ok(game_status) => {
                        if game_status.restart {
                            //Let the loop happen again
                        } else {
                            println!("DONE\nExiting cleanly");
                            // CREDITS RUN

                            // Ownership is tough ... maybe there's a smarter way to do this
                            // using smart pointers, but for now, looks like we'll be passing
                            // around the SDLCore to each segment manually.
                            match contents.credits.run(&mut (contents.core)) {
                                Err(e) => println!("\n\t\tEncountered error while running: {}", e),
                                Ok(credit_status) => {
                                    if credit_status.restart {
                                        //let the loop happen again
                                    } else {
                                        println!("DONE\nExiting cleanly");
                                        break;
                                    }
                                }
                            };
                        }

                        //Do something with the score idk
                        // let score = game_status.score;
                    }
                };
            }
        }
    };
}

fn init() -> Result<UrbanOdyssey, String> {
    let core = inf_runner::SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
    // title
    let demo = demo::Demo::init()?;
    let credits = credits::Credits::init()?;
    // physics?
    // procedural generation?

    Ok(UrbanOdyssey {
        core,
        demo,
        credits,
    })
}
