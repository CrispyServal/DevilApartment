use crate::consts::*;
use gdnative::prelude::*;
use rayon::prelude::*;

use crate::{ChunkUpdater, Pixel, WorldBuffer};

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct PixelWorld {
    world_buffer: WorldBuffer,
    updater_grid: Vec<Vec<ChunkUpdater>>,
}

#[methods]
impl PixelWorld {
    pub fn new(_owner: &Reference) -> Self {
        let mut updater_grid = Vec::with_capacity(CHUNK_COUNT_Y);
        for y in 0..CHUNK_COUNT_Y {
            let mut updater_row = Vec::with_capacity(CHUNK_COUNT_X);
            for x in 0..CHUNK_COUNT_X {
                updater_row.push(ChunkUpdater::new(x, y));
            }
            updater_grid.push(updater_row);
        }
        Self {
            world_buffer: WorldBuffer::new(),
            updater_grid,
        }
    }

    #[export]
    pub fn pre_simulate(&self, _owner: &Reference) {
        self.world_buffer.pre_simulate();
    }

    #[export]
    pub fn simulate(&self, _owner: &Reference) {
        self.simulate_phase(0, 0);
        self.simulate_phase(0, 1);
        self.simulate_phase(1, 1);
        self.simulate_phase(1, 0);
    }

    #[export]
    pub fn user_set_pixel(&self, _owner: &Reference, world_x: usize, world_y: usize, id: u8) {
        self.world_buffer
            .set_pixel(world_x, world_y, Pixel::from_id(id))
    }

    // TODO: delete this
    #[export]
    pub fn get_pixel(&self, _owner: &Reference, world_x: usize, world_y: usize) -> u8 {
        self.world_buffer.get_pixel(world_x, world_y).id
    }

    // TODO: delete this
    #[export]
    pub fn is_chunk_active(&self, _owner: &Reference, chunk_x: usize, chunk_y: usize) -> bool {
        self.world_buffer.is_chunk_active(chunk_x, chunk_y)
    }

    fn simulate_phase(&self, x_mode: usize, y_mode: usize) {
        let mut active_updaters = vec![];
        for chunk_y in 0..CHUNK_COUNT_Y {
            for chunk_x in 0..CHUNK_COUNT_X {
                let updater = &self.updater_grid[chunk_y][chunk_x];
                if self.world_buffer.is_chunk_active(chunk_x, chunk_y)
                    && chunk_y % 2 == y_mode
                    && chunk_x % 2 == x_mode
                {
                    active_updaters.push(updater);
                }
            }
        }

        active_updaters
            .into_par_iter()
            .for_each(|updater| updater.simulate(&self.world_buffer));
    }
}
