mod air;
use air::Air;

use crate::world_buffer::WorldBuffer;

pub trait Pixel: Send + Sync {
    fn get_id(&self) -> u8;
    fn is_empty(&self) -> bool;
    fn step(&self, world_buffer: &WorldBuffer);
}

pub fn default_pixel() -> Box<dyn Pixel> {
    Box::new(Air)
}

pub fn new_from_id(id: u8) -> Box<dyn Pixel> {
    match id {
        // TODO
        _ => default_pixel()
    }
}