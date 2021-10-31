use std::sync::Mutex;

use crate::consts::*;
use crate::pixel::Pixel;

pub struct HalfChunkInner {
    pub x: usize,
    pub y: usize,
    pub data: Vec<Vec<Pixel>>,
}

impl HalfChunkInner {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            data: vec![vec![Pixel::default(); HALF_CHUNK_SIZE]; HALF_CHUNK_SIZE],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.data[y][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.data[y][x] = pixel;
    }
}

pub struct HalfChunk {
    data: Mutex<HalfChunkInner>,
}

impl HalfChunk {
    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.data.lock().unwrap().get_pixel(x, y)
    }

    pub fn set_pixel(&self, x: usize, y: usize, pixel: Pixel) {
        self.data.lock().unwrap().set_pixel(x, y, pixel)
    }
}

#[derive(Default)]
pub struct ChunkInner {
    is_active: bool,
    is_active_next: bool,
}

pub struct Chunk {
    data: Mutex<ChunkInner>,
}

impl Chunk {
    pub fn pre_simulate(&self) {
        let mut inner = self.data.lock().unwrap();
        inner.is_active = inner.is_active_next;
        inner.is_active_next = false;
    }

    pub fn is_active(&self) -> bool {
        self.data.lock().unwrap().is_active
    }

    pub fn set_active_next(&self) {
        self.data.lock().unwrap().is_active_next = true;
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

    pub fn set_chunk_active_next(&self, chunk_x: usize, chunk_y: usize) {
        self.chunk_grid[chunk_y][chunk_x].set_active_next()
    }

    pub fn get_pixel(&self, world_x: usize, world_y: usize) -> Pixel {
        self.half_chunk_grid[world_y / HALF_CHUNK_SIZE][world_x / HALF_CHUNK_SIZE]
            .get_pixel(world_x % HALF_CHUNK_SIZE, world_y % HALF_CHUNK_SIZE)
    }

    pub fn set_pixel(&self, world_x: usize, world_y: usize, pixel: Pixel) {
        //println!("set {}, {} to {}", world_x, world_y, pixel.id);
        self.half_chunk_grid[world_y / HALF_CHUNK_SIZE][world_x / HALF_CHUNK_SIZE].set_pixel(
            world_x % HALF_CHUNK_SIZE,
            world_y % HALF_CHUNK_SIZE,
            pixel,
        );
        self.set_chunk_active_next(world_x / CHUNK_SIZE, world_y / CHUNK_SIZE);
    }
}