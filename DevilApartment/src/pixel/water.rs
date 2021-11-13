use crate::{HDirection, UVec2, WorldBuffer};

use super::{FallingPixel, Pixel};
use super::{DY_LUT, DY_LUT_LEN};

const HORIZONTAL_MOVE_DISTANCE: usize = 32;

#[derive(Copy, Clone)]
/// 只要有方向，水每帧就会尝试往方向走，LR方向会先左右一起尝试，找到最成功的路径。
/// 成功指：假设有向下的机会，则选择最短的；假设一直平着走，则选择最长的。
enum WaterDirection {
    None,
    LR,
    Left,
    Right,
}

impl WaterDirection {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    pub fn from_hdirection(direction: HDirection) -> Self {
        match direction {
            HDirection::Left => Self::Left,
            HDirection::Right => Self::Right,
        }
    }
}

impl Default for WaterDirection {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Default)]
pub struct Water {
    dy_index: usize,
    direction: WaterDirection,
}

struct WaterHIter<'w> {
    pos: UVec2,
    remain_distance: usize,
    direction: HDirection,
    world_buffer: &'w WorldBuffer,
    died: bool,
}

impl<'w> WaterHIter<'w> {
    pub fn new(
        pos: UVec2,
        direction: HDirection,
        world_buffer: &'w WorldBuffer,
        left_distance: usize,
    ) -> Self {
        Self {
            pos,
            remain_distance: left_distance,
            direction,
            world_buffer,
            died: false,
        }
    }
}

impl<'w> Iterator for WaterHIter<'w> {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.remain_distance == 0 {
            return None;
        }
        let pos_check = match self.direction {
            HDirection::Left => self.pos.move_left(),
            HDirection::Right => self.pos.move_right(),
        };
        if !WorldBuffer::can_get_pixel(pos_check.x, pos_check.y) {
            self.died = true;
            return None;
        }
        let pixel_check = self.world_buffer.get_pixel(pos_check.x, pos_check.y);
        if !pixel_check.is_empty() {
            self.died = true;
            return None;
        }

        if WorldBuffer::can_get_pixel(pos_check.x, pos_check.y + 1)
            && self
                .world_buffer
                .get_pixel(pos_check.x, pos_check.y + 1)
                .is_empty()
        {
            self.pos = pos_check;
            return None;
        }

        self.pos = pos_check;
        self.remain_distance -= 1;
        Some(())
    }
}

impl Water {
    fn try_move_down(
        &mut self,
        world_buffer: &WorldBuffer,
        self_x: usize,
        self_y: usize,
    ) -> Option<UVec2> {
        let dy = self.get_dy();
        let mut is_stop = false;
        // try move down
        let mut final_y = self_y;
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
            return Some(UVec2::new(self_x, final_y));
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
    ) -> Option<UVec2> {
        if let Some(final_pos) = self.try_move_down(world_buffer, self_x, self_y) {
            self.direction = WaterDirection::LR;
            return Some(final_pos);
        }

        let mut iters = [None, None];
        match self.direction {
            WaterDirection::None => {
                return None;
            }
            WaterDirection::LR => {
                let iter_left = WaterHIter::new(
                    UVec2::new(self_x, self_y),
                    HDirection::Left,
                    world_buffer,
                    HORIZONTAL_MOVE_DISTANCE,
                );
                let iter_right = WaterHIter::new(
                    UVec2::new(self_x, self_y),
                    HDirection::Right,
                    world_buffer,
                    HORIZONTAL_MOVE_DISTANCE,
                );
                iters[0] = Some(iter_left);
                iters[1] = Some(iter_right);
            }
            WaterDirection::Left => {
                let iter_left = WaterHIter::new(
                    UVec2::new(self_x, self_y),
                    HDirection::Left,
                    world_buffer,
                    HORIZONTAL_MOVE_DISTANCE,
                );
                iters[0] = Some(iter_left);
            }
            WaterDirection::Right => {
                let iter_right = WaterHIter::new(
                    UVec2::new(self_x, self_y),
                    HDirection::Right,
                    world_buffer,
                    HORIZONTAL_MOVE_DISTANCE,
                );
                iters[1] = Some(iter_right);
            }
        }
        // try move horizontally

        let mut last_pos = None;
        loop {
            let mut have_iter_alive = false;
            for maybe_iter in iters.iter_mut() {
                if let Some(iter) = maybe_iter {
                    have_iter_alive = true;
                    let iter_result = iter.next();
                    if iter_result.is_none() {
                        if !iter.died {
                            self.direction = WaterDirection::from_hdirection(iter.direction);
                            return Some(iter.pos);
                        } else {
                            if iter.remain_distance == 0 {
                                self.direction = WaterDirection::None;
                            }
                            else {
                                self.direction = WaterDirection::from_hdirection(iter.direction);
                            }
                            last_pos = Some(iter.pos);
                            *maybe_iter = None;
                        }
                    }
                }
            }

            if !have_iter_alive {
                return last_pos;
            }
        }
    }
}
