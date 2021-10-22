use crate::rect;
// use crate::Physics;

use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct ProceduralGen;

pub struct TerrainSegment<'a> {
    pos: Rect,
    // curve: Bezier Curve,
    texture: &'a Texture<'a>,
}

impl<'a> TerrainSegment<'a> {
    pub fn new(pos: Rect, texture: &'a Texture<'a>) -> TerrainSegment {
        TerrainSegment { pos, texture }
    }

    pub fn x(&self) -> i32 {
        self.pos.x()
    }

    pub fn y(&self) -> i32 {
        self.pos.y()
    }

    pub fn w(&self) -> i32 {
        self.pos.width() as i32
    }

    pub fn h(&self) -> i32 {
        self.pos.height() as i32
    }

    pub fn pos(&self) -> &Rect {
        &self.pos
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn update_pos(&mut self, x_adj: i32, y_adj: i32) {
        self.pos.set_x(self.pos.x() + x_adj);
        self.pos.set_y(self.pos.y() + y_adj);
    }
}

impl ProceduralGen {
    pub fn init_terrain<'a>(
        cam_w: i32,
        cam_h: i32,
        texture: &'a Texture<'a>,
    ) -> TerrainSegment<'a> {
        TerrainSegment::new(rect!(0, cam_h * 2 / 3, cam_w, cam_h / 3), &texture)
    }

    pub fn gen_land<'a>(
        prev_segment: &TerrainSegment,
        cam_w: i32,
        cam_h: i32,
        is_pit: bool,
        is_flat: bool,
        is_cliff: bool,
        texture: &'a Texture<'a>,
    ) -> TerrainSegment<'a> {
        //TODO
        //prev_point - Last point of the previouly generated bit of land
        //length - length of next batch of generated land
        //is_pit - binary tick, next batch of land will have a pit in it
        //is_flat - binary tick, next batch of land will be flat or mostly flat
        // (shallow curve) is_cliff - binary tick, next batch of land
        // will have a point where it drops down into a cliff face
        TerrainSegment::new(
            rect!(
                prev_segment.x() + prev_segment.w(),
                prev_segment.y(),
                cam_w,
                cam_h / 3
            ),
            &texture,
        )
    }

    fn gen_noise() -> bool {
        //TODO
        //Perlin noise generation
        false
    }

    fn gen_curve() -> bool {
        //TODO
        //Bezier curve
        false
    }
}
