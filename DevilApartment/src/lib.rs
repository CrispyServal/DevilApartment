use gdnative::prelude::*;

mod consts;
mod pixel;
mod world_buffer;
mod chunk_updater;
mod pixel_world;
mod image_updater;
mod image_chunk;

use world_buffer::WorldBuffer;
use chunk_updater::ChunkUpdater;
use pixel::Pixel;
use pixel_world::PixelWorld;
use image_chunk::ImageChunk;


fn init(handle: InitHandle) {
    handle.add_class::<PixelWorld>();
    handle.add_class::<image_updater::ImageUpdater>();
    handle.add_class::<ImageChunk>();
}

godot_init!(init);
