use crate::{consts::*, UVec2, WorldBuffer};

#[derive(Debug)]
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
        if active_range.max_x > active_range.min_x && active_range.max_y > active_range.min_y {
            println!(
                "chunk: {}, {} active range: {}, {}, {}, {}",
                self.chunk_x,
                self.chunk_y,
                active_range.min_x,
                active_range.max_x,
                active_range.min_y,
                active_range.max_y
            )
        }
        for world_y in
            (self.start_world_y + active_range.min_y..self.start_world_y + active_range.max_y).rev()
        {
            for world_x in
                self.start_world_x + active_range.min_x..self.start_world_x + active_range.max_x
            {
                let mut pixel = world_buffer.get_pixel(world_x, world_y);
                if let Some(final_pos) = pixel.try_move_self(world_buffer, world_x, world_y) {
                    if !final_pos.same(&UVec2::new(world_x, world_y)) {
                        let tmp = world_buffer.get_pixel(final_pos.x, final_pos.y);
                        world_buffer.set_pixel(final_pos.x, final_pos.y, pixel);
                        world_buffer.set_pixel(world_x, world_y, tmp);
                    }
                }
            }
        }
    }
}
