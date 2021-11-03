use crate::world_buffer::WorldBuffer;
use rand::prelude::*;

use super::{FallingPixel, Pixel};
use super::{DY_LUT, DY_LUT_LEN};

const HORIZONTAL_MOVE_DISTANCE: usize = 32;

#[derive(Copy, Clone, Default)]
pub struct Water {
    dy_index: usize,
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

    fn try_move_self(
        &mut self,
        world_buffer: &crate::world_buffer::WorldBuffer,
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
        // try move horizontally
        let move_left: bool = random();
        let direction_x: i32 = if move_left { -1 } else { 1 };
        let left_range = (self_x - (HORIZONTAL_MOVE_DISTANCE.min(self_x))..self_x).into_iter().rev().collect();
        let right_range = (self_x + 1..self_x + HORIZONTAL_MOVE_DISTANCE).collect();
        //println!("{:?}, {:?}", left_range, right_range);
        let range_list: Vec<Vec<usize>> = if move_left {
            vec![left_range, right_range]
        } else {
            vec![right_range, left_range]
        };
        for range in range_list {
            for xx in range {
                if !WorldBuffer::can_get_pixel(xx, self_y) {
                    break;
                }
                let check_target = world_buffer.get_pixel(xx, self_y);
                if !check_target.is_empty() {
                    final_x = ((xx as i32 - direction_x + self_x as i32) / 2) as usize;
                    break;
                }
                if !WorldBuffer::can_get_pixel(xx, self_y + 1) {
                    break;
                }
                let check_target = world_buffer.get_pixel(xx, self_y + 1);
                if check_target.is_empty() {
                    final_x = xx;
                    final_y = self_y + 1;
                    break; // 有空隙往下走
                }
                final_x = xx;
            }
            if final_x != self_x || final_y != self_y {
                return Some((final_x, final_y));
            }
        }

        None
    }
}
