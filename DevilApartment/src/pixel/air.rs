use super::Pixel;
pub struct Air;

impl Pixel for Air {
    fn get_id(&self) -> u8 {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn step(&self, world_buffer: &crate::world_buffer::WorldBuffer) {
        
    }
}
