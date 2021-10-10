use gdnative::api::ImageTexture;
use gdnative::prelude::*;

const WORLD_SIZE: usize = 16384usize;
const IMAGE_SIZE: usize = 256usize;

pub type PixelId = u8;

#[derive(Default, Clone, Copy)]
pub struct Pixel {
    id: PixelId,
}

impl Pixel {
    pub fn to_color(&self) -> Color {
        match self.id {
            0 => Color::rgb(0., 0., 0.),
            1 => Color::rgb(1., 1., 0.),
            _ => Color::rgb(0.3, 0.3, 0.3),
        }
    }
}

pub struct World {
    // world: [[Pixel; WORLD_SIZE]; WORLD_SIZE],
    world: Vec<Vec<Pixel>>,
}

impl World {
    pub fn new_empty() -> Self {
        Self {
            world: vec![vec![Pixel::default(); WORLD_SIZE]; WORLD_SIZE],
        }
    }

    pub fn fetch(&self, x: usize, y: usize) -> &Pixel {
        &self.world[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, pid: PixelId) {
        self.world[y][x].id = pid;
    }
}

pub struct ImageChunk {
    image: Ref<Image>,
    texture: Ref<ImageTexture>,
    sprite: Ref<Sprite>,
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct RustEntry {
    image_grid: Vec<Vec<ImageChunk>>,
    world: World,
}

#[methods]
impl RustEntry {
    fn new(_owner: &Node2D) -> Self {
        let num_of_image_1d = WORLD_SIZE / IMAGE_SIZE;
        let mut image_grid = Vec::with_capacity(num_of_image_1d);
        for y in 0..num_of_image_1d {
            let mut image_row = Vec::with_capacity(num_of_image_1d);
            for x in 0..num_of_image_1d {
                let image = Image::new();
                image.create(
                    IMAGE_SIZE as i64,
                    IMAGE_SIZE as i64,
                    false,
                    Image::FORMAT_RGB8,
                );
                let texture = ImageTexture::new();
                texture.create(
                    IMAGE_SIZE as i64,
                    IMAGE_SIZE as i64,
                    Image::FORMAT_RGB8,
                    ImageTexture::STORAGE_RAW,
                );
                let sprite = Sprite::new();
                let chunk = ImageChunk {
                    image: image.into_shared(),
                    texture: texture.into_shared(),
                    sprite: sprite.into_shared(),
                };
                image_row.push(chunk);
            }
            image_grid.push(image_row);
        }

        Self {
            image_grid,
            world: World::new_empty(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        godot_print!("Hello, RustEntry.");
        let num_of_image_1d = WORLD_SIZE / IMAGE_SIZE;
        for chunk_y in 0..num_of_image_1d {
            for chunk_x in 0..num_of_image_1d {
                let sprite = unsafe { self.image_grid[chunk_y][chunk_x].sprite.assume_safe() };
                sprite.set_position(Vector2::new(
                    (chunk_x as f32 + 0.5) * (IMAGE_SIZE as f32),
                    (chunk_y as f32 + 0.5) * (IMAGE_SIZE as f32),
                ));
                owner.add_child(sprite, false);
            }
        }
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, delta: f64) {
        self.draw(owner)
    }

    #[export]
    fn add_pixel(&mut self, _owner: &Node2D, x: usize, y: usize, p: u8) {
        self.world.set(x, y, p);
    }

    // 目前只draw第0行0的chunk
    fn draw(&mut self, owner: &Node2D) {
        unsafe {
            for chunk_y in 0..2 {
                for chunk_x in 0..3 {
                    let world_image = self.image_grid[chunk_y][chunk_x].image.assume_safe();
                    world_image.lock();
                    for y in 0usize..IMAGE_SIZE {
                        for x in 0usize..IMAGE_SIZE {
                            world_image.set_pixel(
                                x as i64,
                                y as i64,
                                self.world.fetch(x + chunk_x * IMAGE_SIZE, y + chunk_y * IMAGE_SIZE).to_color(),
                            );
                        }
                    }
                    world_image.unlock();
                    let world_texture = self.image_grid[chunk_y][chunk_x].texture.assume_safe();
                    world_texture.set_data(world_image);
                    let sprite = self.image_grid[chunk_y][chunk_x].sprite.assume_safe();
                    sprite.set_texture(world_texture);
                }
            }
        }
    }
}
