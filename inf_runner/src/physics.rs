use crate::rect;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

const SPEED_LIMIT: i32 = 5;

struct Player<'a> {
    pos: Rect,
    velocity: (i32, i32),
    accel: (i32, i32),
    mass: u32,
    texture: Texture<'a>,
}

impl<'a> Player<'a> {
    fn new(pos: Rect, mass: u32, texture: Texture<'a>) -> Player {
        Player {
            pos: pos,
            velocity: (0, 0),
            accel: (0, 0),
            texture: texture,
            mass: mass,
        }
    }

    fn x(&self) -> i32 {
        self.pos.x()
    }

    fn y(&self) -> i32 {
        self.pos.y()
    }

    fn vel_x(&self) -> i32 {
        self.velocity.0
    }

    fn vel_y(&self) -> i32 {
        self.velocity.1
    }

    fn update_pos(&mut self, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
        self.pos
            .set_x((self.pos.x() + self.velocity.0).clamp(x_bounds.0, x_bounds.1));
        self.pos
            .set_y((self.pos.y() + self.velocity.1).clamp(y_bounds.0, y_bounds.1));
    }

    fn update_vel(&mut self) {
        // Update to make the TOTAL MAX VELOCITY constant
        // Right now it's SPEED_LIMIT in one direction and SPEED_LIMIT*sqrt(2) diagonally
        self.vel.0 = (self.vel.0 + self.accel.0).clamp(0, SPEED_LIMIT);
        self.vel.1 = (self.vel.1 + self.accel.1).clamp(0, SPEED_LIMIT);
    }

    fn force(&mut self) {
        // Update acceleration based on force and mass
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }
}
