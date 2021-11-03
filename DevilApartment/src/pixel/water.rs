use crate::world_buffer::WorldBuffer;

use super::{FallingPixel, Pixel};
use super::{DY_LUT, DY_LUT_LEN};

const HORIZONTAL_MOVE_DISTANCE: usize = 32;

#[derive(Copy, Clone, Default)]
pub struct Water {
    dy_index: usize,
}

impl Water {
    fn try_move_down(
        &mut self,
        world_buffer: &WorldBuffer,
        self_x: usize,
        self_y: usize,
    ) -> Option<(usize, usize)> {
        let dy = self.get_dy();
        let mut is_stop = false;
        let mut final_x = self_x;
        let mut final_y = self_y;
        // try move down
        for yy in self_y + 1..self_y + dy + 1 {
            if !WorldBuffer::can_get_pixel(self_x, yy) {
                is_stop = true;
                break;
            }
            let check_target = world_buffer.get_pixel(self_x, yy);
            if !check_target.is_empty() {
                is_stop = true;
                break;
            }
            final_y = yy;
        }
        if !is_stop {
            self.add_dy();
        }
        if final_y != self_y {
            return Some((final_x, final_y));
        }

        None
    }

    fn try_move_horizontly(
        &mut self,
        world_buffer: &WorldBuffer,
        self_x: usize,
        self_y: usize,
        is_left: bool,
    ) -> Option<(usize, usize)> {
        // 要防止来回跑，但是这样会堆成柱子。TODO: 实际需要防止左右跑，但是允许钻水左右跑
        if is_left
            && (!WorldBuffer::can_get_pixel(self_x + 1, self_y)
                || (world_buffer.get_pixel(self_x + 1, self_y).get_id() != self.get_id()))
        {
            return None;
        }
        if !is_left
            && (self_x == 0 || !WorldBuffer::can_get_pixel(self_x - 1, self_y)
                || (world_buffer.get_pixel(self_x - 1, self_y).get_id() != self.get_id()))
        {
            return None;
        }
        let range = if is_left {
            Self::make_range_to_left(self_x)
        } else {
            Self::make_range_to_right(self_x)
        };

        let mut is_in_water = false;
        let mut final_x = self_x;
        let mut final_y = self_y;
        let mut yy = self_y;
        for xx in range {
            if !WorldBuffer::can_get_pixel(xx, yy) {
                break;
            }
            let check_target = world_buffer.get_pixel(xx, yy);
            if is_in_water {
                if check_target.is_solid() {
                    break;
                }
                if check_target.is_empty() {
                    // 出水了，可换
                    final_y = yy;
                    final_x = xx;
                    break;
                }
            } else {
                if !check_target.is_empty() {
                    if WorldBuffer::can_get_pixel(xx, yy + 1)
                        && world_buffer.get_pixel(xx, yy + 1).is_liquid()
                    {
                        is_in_water = true;
                        yy += 1;
                    }
                } else {
                    break;
                }
            }
            if !is_in_water {
                if !WorldBuffer::can_get_pixel(xx, yy + 1) {
                    break;
                }
                let check_target = world_buffer.get_pixel(xx, yy + 1);
                if !is_in_water && check_target.is_empty() {
                    final_x = xx;
                    final_y = yy + 1;
                    break; // 有空隙往下一格结束
                }
            }
            final_x = xx;
        }

        if final_x != self_x || final_y != self_y {
            return Some((final_x, final_y));
        }

        None
    }

    fn make_range_to_left(self_x: usize) -> Vec<usize> {
        (self_x - (HORIZONTAL_MOVE_DISTANCE.min(self_x))..self_x)
            .into_iter()
            .rev()
            .collect()
    }

    fn make_range_to_right(self_x: usize) -> Vec<usize> {
        (self_x + 1..self_x + HORIZONTAL_MOVE_DISTANCE).collect()
    }
}

impl FallingPixel for Water {
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

impl Pixel for Water {
    fn get_id(&self) -> u8 {
        3
    }

    fn is_empty(&self) -> bool {
        false
    }
    fn is_liquid(&self) -> bool {
        true
    }
    fn is_solid(&self) -> bool {
        false
    }

    fn try_move_self(
        &mut self,
        world_buffer: &WorldBuffer,
        self_x: usize,
        self_y: usize,
    ) -> Option<(usize, usize)> {
        if let Some(final_pos) = self.try_move_down(world_buffer, self_x, self_y) {
            return Some(final_pos);
        }
        // try move horizontally
        let maybe_left_pos = self.try_move_horizontly(world_buffer, self_x, self_y, true);
        let maybe_right_pos = self.try_move_horizontly(world_buffer, self_x, self_y, false);

        if let Some(left_pos) = maybe_left_pos {
            if let Some(right_pos) = maybe_right_pos {
                let dx_left = self_x - left_pos.0;
                let dx_right = right_pos.0 - self_x;
                if dx_left < dx_right {
                    Some(left_pos)
                } else {
                    Some(right_pos)
                }
            } else {
                Some(left_pos)
            }
        } else {
            maybe_right_pos
        }
    }
}
