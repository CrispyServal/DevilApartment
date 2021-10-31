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
                    let mut final_y = world_y;
                    let mut is_stop = false;
                    for yy in world_y + 1..world_y + dy + 1 {
                        if !Self::can_get_pixel(world_x, yy) {
                            is_stop = true;
                            break;
                        }
                        let check_target = world_buffer.get_pixel(world_x, yy);
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
                        let tmp = world_buffer.get_pixel(world_x, final_y);
                        world_buffer.set_pixel(world_x, final_y, pixel);
                        world_buffer.set_pixel(world_x, world_y, tmp);
                    }
                }
            }
        }
    }

    fn can_get_pixel(world_x: usize, world_y: usize) -> bool {
        world_x >= 0 && world_y >= 0 && world_x < WORLD_WIDTH && world_y < WORLD_HEIGHT
    }
}