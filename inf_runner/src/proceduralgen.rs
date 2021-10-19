use crate::rect;
use sdl2::rect::Rect;
use sdl2::render::Texture;

// use crate::Physics;

pub struct ProceduralGen;

impl ProceduralGen {
  fn gen_land(Point: &prev_point, i32: &length, bool: &is_pit, bool: &is_flat, bool: &is_cliff) -> ?? {
    //TODO
    //prev_point - Last point of the previouly generated bit of land
    //length - length of next batch of generated land
    //is_pit - binary tick, next batch of land will have a pit in it
    //is_flat - binary tick, next batch of land will be flat or mostly flat (shallow curve)
    //is_cliff - binary tick, next batch of land will have a point where it drops down into a cliff face 
  }

  fn gen_noise() -> ?? {
    //TODO
    //Perlin noise generation
  }

  fn gen_curve() -> ?? {
    //TODO
    //Bezier curve
  }

  
}
