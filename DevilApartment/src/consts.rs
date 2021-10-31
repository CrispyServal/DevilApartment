pub const WORLD_HEIGHT: usize = 512usize;
pub const WORLD_WIDTH: usize = 2048usize;
pub const CHUNK_SIZE: usize = 64usize;
pub const HALF_CHUNK_SIZE: usize = 32usize;
pub const THREAD_COUNT: usize = 8usize;
pub const TEXTURE_SIZE: usize = 256usize;
pub const CHUNK_COUNT_Y: usize = WORLD_HEIGHT / CHUNK_SIZE;
pub const CHUNK_COUNT_X: usize = WORLD_WIDTH / CHUNK_SIZE;
pub const HC_PER_TEXTURE: usize = TEXTURE_SIZE / HALF_CHUNK_SIZE;
pub const HALF_CHUNK_COUNT_Y: usize = WORLD_HEIGHT / HALF_CHUNK_SIZE;
pub const HALF_CHUNK_COUNT_X: usize = WORLD_WIDTH / HALF_CHUNK_SIZE;
pub const CHUNK_PER_TEXTURE: usize = TEXTURE_SIZE / CHUNK_SIZE;