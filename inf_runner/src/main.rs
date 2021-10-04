// University of Pittsburgh
// CS 1666 - Fall 2021
// Infinite Runner

mod credits;
mod utils;

use inf_runner::Game;

const TITLE: &str = "Urban Odyssey";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct UrbanOdyssey {
    core: inf_runner::SDLCore,
    // title,
    // gameplay,
    credits: credits::Credits,
    // physics
    // procedural generation
}

fn main() {
    println!("\nRunning {}:", TITLE);
    print!("\tInitting...");

    let game = init();
    match game {
        Err(e) => println!("\n\t\tFailed to init: {}", e),
        Ok(mut contents) => {
            println!("DONE");

            print!("\tRunning...");

            // TITLE SCREEN RUN
            // GAME PLAY RUN
            // CREDITS RUN

            // Ownership is tough ... maybe there's a smarter way to do this
            // using smart pointers, but for now, looks like we'll be passing
            // around the SDLCore to each section manually.
            match contents.credits.run(&mut (contents.core)) {
                Err(e) => println!("\n\t\tEncountered error while running: {}", e),
                Ok(_) => println!("DONE\nExiting cleanly"),
            };
        }
    };
}

fn init() -> Result<UrbanOdyssey, String> {
    let core = inf_runner::SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
    let credits = credits::Credits::init()?;

    Ok(UrbanOdyssey { core, credits })
}
