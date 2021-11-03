use gdnative::prelude::*;

mod chunk_updater;
mod consts;
mod image_chunk;
mod image_updater;
mod pixel;
mod pixel_world;
mod range2d;
mod world_buffer;

use chunk_updater::ChunkUpdater;
use image_chunk::ImageChunk;
use pixel::Pixel;
use pixel_world::PixelWorld;
use range2d::Range2d;
use world_buffer::WorldBuffer;

fn init(handle: InitHandle) {
    handle.add_class::<PixelWorld>();
    handle.add_class::<image_updater::ImageUpdater>();
    handle.add_class::<ImageChunk>();
}

godot_init!(init);
