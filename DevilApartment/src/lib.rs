use gdnative::prelude::*;

mod consts;
mod pixel;
mod world_buffer;
mod chunk_updater;
mod pixel_world;

use world_buffer::WorldBuffer;
use chunk_updater::ChunkUpdater;
use pixel::Pixel;


fn init(handle: InitHandle) {
    handle.add_class::<pixel_world::PixelWorld>();
}

godot_init!(init);
