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
        for world_y in (self.start_world_y..self.start_world_y + CHUNK_SIZE).rev() {
            for world_x in (self.start_world_x..self.start_world_x + CHUNK_SIZE) {
                let mut pixel = world_buffer.get_pixel(world_x, world_y);
                if pixel.is_fall() {
                    let dy = pixel.get_dy();
                    let mut x_check = vec![world_x, world_x + 1];
                    if world_x > 0 {
                        x_check.push(world_x - 1);
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
                }
            }
        }
    }

    fn can_get_pixel(world_x: usize, world_y: usize) -> bool {
        world_x < WORLD_WIDTH && world_y < WORLD_HEIGHT
    }
}
