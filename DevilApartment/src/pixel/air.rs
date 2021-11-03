use super::Pixel;

#[derive(Copy, Clone)]
pub struct Air;

impl Pixel for Air {
    fn get_id(&self) -> u8 {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn try_move_self(
        &mut self,
        _world_buffer: &crate::world_buffer::WorldBuffer,
        _self_x: usize,
        _self_y: usize,
    ) -> Option<(usize, usize)> {
        None
    }
}
