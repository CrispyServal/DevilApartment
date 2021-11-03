use std::sync::Mutex;

use crate::consts::*;
use crate::Pixel;
use crate::Range2d;
use crate::pixel::default_pixel;

pub struct HalfChunkInner {
    pub x: usize,
    pub y: usize,
    pub data: Vec<Vec<Box<dyn Pixel>>>,
}

impl HalfChunkInner {
    pub fn new(x: usize, y: usize) -> Self {
        let mut pixel_grid = Vec::with_capacity(HALF_CHUNK_SIZE);
        for _ in 0..HALF_CHUNK_SIZE {
            let mut pixel_row = Vec::with_capacity(HALF_CHUNK_SIZE);
            for _ in 0..HALF_CHUNK_SIZE {
                pixel_row.push(default_pixel());
            }
            pixel_grid.push(pixel_row);
        }
        Self {
            x,
            y,
            data: pixel_grid,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &dyn Pixel {
        self.data[y][x].as_ref()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Box<dyn Pixel>) {
        self.data[y][x] = pixel;
    }
}

pub struct HalfChunk {
    data: Mutex<HalfChunkInner>,
}

impl HalfChunk {
    pub fn get_pixel(&self, x: usize, y: usize) -> &dyn Pixel {
        self.data.lock().unwrap().get_pixel(x, y)
    }

    pub fn set_pixel(&self, x: usize, y: usize, pixel: Box<dyn Pixel>) {
        self.data.lock().unwrap().set_pixel(x, y, pixel)
    }
}

#[derive(Default)]
pub struct ChunkInner {
    is_active: bool,
    is_active_next: bool,
    active_range: Range2d,
    active_range_next: Range2d,
}

pub struct Chunk {
    data: Mutex<ChunkInner>,
}

impl Chunk {
    pub fn pre_simulate(&self) {
        let mut inner = self.data.lock().unwrap();
        inner.is_active = inner.is_active_next;
        inner.is_active_next = false;
        inner.active_range = inner.active_range_next;
        inner.active_range_next = Range2d::default();
    }

    pub fn is_active(&self) -> bool {
        self.data.lock().unwrap().is_active
    }

    pub fn set_active_next(&self, x_in_chunk: usize, y_in_chunk: usize) {
        let mut inner = self.data.lock().unwrap();
        inner.is_active_next = true;
        inner.active_range_next.min_x = inner.active_range_next.min_x.min(x_in_chunk);
        inner.active_range_next.max_x = inner.active_range_next.max_x.max(x_in_chunk + 1);
        inner.active_range_next.min_y = inner.active_range_next.min_y.min(y_in_chunk);
        inner.active_range_next.max_y = inner.active_range_next.max_y.max(y_in_chunk + 1);
    }

    pub fn get_active_range(&self) -> Range2d {
        self.data.lock().unwrap().active_range
    }
}

pub struct WorldBuffer {
    half_chunk_grid: Vec<Vec<HalfChunk>>,
    chunk_grid: Vec<Vec<Chunk>>,
}

impl WorldBuffer {
    pub fn new() -> Self {
        let mut hc_grid = Vec::with_capacity(HALF_CHUNK_COUNT_Y);
        for y in 0..HALF_CHUNK_COUNT_Y {
            let mut hc_row = Vec::with_capacity(HALF_CHUNK_COUNT_X);
            for x in 0..HALF_CHUNK_COUNT_X {
                let hc = HalfChunk {
                    data: Mutex::new(HalfChunkInner::new(x, y)),
                };
                hc_row.push(hc);
            }
            hc_grid.push(hc_row);
        }

        let mut chunk_grid = Vec::with_capacity(CHUNK_COUNT_Y);
        for y in 0..CHUNK_COUNT_Y {
            let mut chunk_row = Vec::with_capacity(CHUNK_COUNT_X);
            for x in 0..CHUNK_COUNT_X {
                let chunk = Chunk {
                    data: Mutex::new(ChunkInner::default()),
                };
                chunk_row.push(chunk);
            }
            chunk_grid.push(chunk_row);
        }

        Self {
            half_chunk_grid: hc_grid,
            chunk_grid,
        }
    }

    pub fn pre_simulate(&self) {
        self.chunk_grid
            .iter()
            .for_each(|row| row.iter().for_each(|ch| ch.pre_simulate()));
    }

    pub fn is_chunk_active(&self, chunk_x: usize, chunk_y: usize) -> bool {
        self.chunk_grid[chunk_y][chunk_x].is_active()
    }

    pub fn set_chunk_active_next(&self, chunk_x: usize, chunk_y: usize, x_in_chunk: usize, y_in_chunk: usize) {
        self.chunk_grid[chunk_y][chunk_x].set_active_next(x_in_chunk, y_in_chunk);
    }

    pub fn get_chunk_active_range(&self, chunk_x: usize, chunk_y: usize) -> Range2d {
        self.chunk_grid[chunk_y][chunk_x].get_active_range()
    }

    pub fn get_pixel(&self, world_x: usize, world_y: usize) -> &dyn Pixel {
        self.half_chunk_grid[world_y / HALF_CHUNK_SIZE][world_x / HALF_CHUNK_SIZE]
            .get_pixel(world_x % HALF_CHUNK_SIZE, world_y % HALF_CHUNK_SIZE)
    }

    pub fn set_pixel(&self, world_x: usize, world_y: usize, pixel: Box<dyn Pixel>) {
        //println!("set {}, {} to {}", world_x, world_y, pixel.id);
        self.half_chunk_grid[world_y / HALF_CHUNK_SIZE][world_x / HALF_CHUNK_SIZE].set_pixel(
            world_x % HALF_CHUNK_SIZE,
            world_y % HALF_CHUNK_SIZE,
            pixel,
        );
        let chunk_x = world_x / CHUNK_SIZE;
        let chunk_y = world_y / CHUNK_SIZE;
        let x_in_chunk = world_x % CHUNK_SIZE;
        let y_in_chunk = world_y % CHUNK_SIZE;
        self.set_chunk_active_next(chunk_x, chunk_y, x_in_chunk, y_in_chunk);
    }
}
