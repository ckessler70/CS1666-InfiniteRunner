// File for important content used across entire project

extern crate float_cmp;
extern crate sdl2;

use sdl2::rect::Rect;

pub struct SDLCore {
    #[allow(dead_code)]
    sdl_cxt: sdl2::Sdl,
    pub wincan: sdl2::render::WindowCanvas,
    pub event_pump: sdl2::EventPump,
    pub cam: Rect,
}

pub enum GameStatus {
    Main,
    Game,
    Credits,
    BezierSim,
}

// Contains all types of terrain
#[derive(Debug)]
pub enum TerrainType {
    Grass,
    Asphalt,
    Sand,
    Water,
}

// Contains all types of objects generated on terrain
pub enum StaticObject {
    Coin,    // Collectable
    Power,   // Collectable
    Statue,  // Obstacle
    Balloon, // Obstacle
    Chest,   // Obstacle
}

// Contains all types of power ups
#[derive(Copy, Clone)]
pub enum PowerType {
    SpeedBoost,
    ScoreMultiplier,
    BouncyShoes,
    LowerGravity,
    Shield,
}

// Contains all types of obstacles
#[derive(Copy, Clone)]
pub enum ObstacleType {
    Statue,
    Balloon,
    Chest,
}

#[allow(dead_code)]
pub struct GameState {
    pub status: Option<GameStatus>,
    pub score: i32,
}

impl SDLCore {
    pub fn init(title: &str, vsync: bool, width: u32, height: u32) -> Result<SDLCore, String> {
        let sdl_cxt = sdl2::init()?;
        let video_subsys = sdl_cxt.video()?;

        let window = video_subsys
            .window(title, width, height)
            .build()
            .map_err(|e| e.to_string())?;

        let wincan = window.into_canvas().accelerated();

        // Check if we should lock to vsync
        let wincan = if vsync {
            wincan.present_vsync()
        } else {
            wincan
        };

        let wincan = wincan.build().map_err(|e| e.to_string())?;

        let event_pump = sdl_cxt.event_pump()?;

        let cam = Rect::new(0, 0, width, height);

        Ok(SDLCore {
            sdl_cxt,
            wincan,
            event_pump,
            cam,
        })
    }
}

pub trait Game {
    fn init() -> Result<Self, String>
    where
        Self: Sized;
    fn run(&mut self, core: &mut SDLCore) -> Result<GameState, String>;
}
