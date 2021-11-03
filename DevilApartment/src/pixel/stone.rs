use super::Pixel;

#[derive(Copy, Clone)]
pub struct Stone;

impl Pixel for Stone {
    fn get_id(&self) -> u8 {
        2
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn need_simulate(&self) -> bool {
        false
    }

    fn step(
        &mut self,
        _world_buffer: &crate::world_buffer::WorldBuffer,
        _self_x: usize,
        _self_y: usize,
    ) {
    }
}
