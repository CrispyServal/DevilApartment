use crate::{UVec2, WorldBuffer};

use super::{FallingPixel, Pixel};
use super::{DY_LUT, DY_LUT_LEN};

#[derive(Copy, Clone, Default)]
pub struct Sand {
    dy_index: usize,
}

impl FallingPixel for Sand {
    fn get_dy(&self) -> usize {
        DY_LUT[self.dy_index]
    }

    fn add_dy(&mut self) {
        self.dy_index = (self.dy_index + 1).min(DY_LUT_LEN - 1);
    }

    fn reset_dy(&mut self) {
        self.dy_index = 0;
    }
}

impl Pixel for Sand {
    fn get_id(&self) -> u8 {
        1
    }

    fn is_empty(&self) -> bool {
        false
    }
    fn is_liquid(&self) -> bool {
        false
    }
    fn is_solid(&self) -> bool {
        true
    }

    fn try_move_self(
        &mut self,
        world_buffer: &crate::world_buffer::WorldBuffer,
        self_x: usize,
        self_y: usize,
    ) -> Option<UVec2> {
        let dy = self.get_dy();
        let mut x_check = vec![self_x];
        let mut final_x = self_x;
        if self_x > 0 && !world_buffer.get_pixel(self_x - 1, self_y).is_solid() {
            x_check.push(self_x - 1);
        }
        if !world_buffer.get_pixel(self_x + 1, self_y).is_solid() {
            x_check.push(self_x + 1);
        }
        for xx in x_check.into_iter() {
            final_x = xx;
            let mut is_stop = false;
            let mut final_y = self_y;
            let mut is_in_liquid = false;
            for yy in self_y + 1..self_y + dy + 1 {
                if !WorldBuffer::can_get_pixel(xx, yy) {
                    is_stop = true;
                    break;
                }
                let check_target = world_buffer.get_pixel(xx, yy);
                if check_target.is_solid() {
                    is_stop = true;
                    break;
                }
                if check_target.is_liquid() {
                    is_in_liquid = true;
                }
                final_y = yy;
                if is_in_liquid && (yy - self_y) > dy / 8 {
                    break;
                }
            }
            if !is_stop {
                self.add_dy();
            }
            if final_y != self_y {
                return Some(UVec2::new(final_x, final_y));
            }
        }
        None
    }
}
