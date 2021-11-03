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

    fn try_move_self(
        &mut self,
        _world_buffer: &crate::world_buffer::WorldBuffer,
        _self_x: usize,
        _self_y: usize,
    ) -> Option<(usize, usize)> {
        None
    }
}
