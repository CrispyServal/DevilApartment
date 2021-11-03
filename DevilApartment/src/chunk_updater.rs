use crate::{consts::*, WorldBuffer};
pub struct ChunkUpdater {
    chunk_x: usize,
    chunk_y: usize,
    start_world_x: usize,
    start_world_y: usize,
}

impl ChunkUpdater {
    pub fn new(chunk_x: usize, chunk_y: usize) -> Self {
        Self {
            chunk_x,
            chunk_y,
            start_world_x: chunk_x * CHUNK_SIZE,
            start_world_y: chunk_y * CHUNK_SIZE,
        }
    }

    pub fn simulate(&self, world_buffer: &WorldBuffer) {
        let active_range = world_buffer.get_chunk_active_range(self.chunk_x, self.chunk_y);
        for world_y in
            (self.start_world_y + active_range.min_y..self.start_world_y + active_range.max_y).rev()
        {
            for world_x in
                self.start_world_x + active_range.min_x..self.start_world_x + active_range.max_x
            {
                let mut pixel = world_buffer.get_pixel(world_x, world_y);
                if pixel.need_simulate() {
                    pixel.step(world_buffer, world_x, world_y);
                }
                /*
                if pixel.is_fall() {
                    let dy = pixel.get_dy();
                    let mut x_check = vec![world_x];
                    if world_x > 0 && world_buffer.get_pixel(world_x - 1, world_y).is_empty() {
                        x_check.push(world_x - 1);
                    }
                    if world_buffer.get_pixel(world_x + 1, world_y).is_empty() {
                        x_check.push(world_x + 1);
                    }
                    for xx in x_check.into_iter() {
                        let mut is_stop = false;
                        let mut final_y = world_y;
                        for yy in world_y + 1..world_y + dy + 1 {
                            if !Self::can_get_pixel(xx, yy) {
                                is_stop = true;
                                break;
                            }
                            let check_target = world_buffer.get_pixel(xx, yy);
                            if !check_target.is_empty() {
                                is_stop = true;
                                break;
                            }
                            final_y = yy;
                        }
                        if !is_stop {
                            pixel = pixel.add_speed();
                        }
                        if final_y != world_y {
                            let tmp = world_buffer.get_pixel(xx, final_y);
                            assert!(tmp.is_empty());
                            world_buffer.set_pixel(xx, final_y, pixel);
                            world_buffer.set_pixel(world_x, world_y, tmp);
                            break;
                        }
                    }
                }*/
            }
        }
    }
}
