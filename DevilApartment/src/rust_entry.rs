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
    draw_range: (usize, usize, usize, usize),
}

#[methods]
impl RustEntry {
    fn new(_owner: &Node2D) -> Self {
        let num_of_image_1d = WORLD_SIZE / IMAGE_SIZE;
        let mut image_grid = Vec::with_capacity(num_of_image_1d);
        for _ in 0..num_of_image_1d {
            let mut image_row = Vec::with_capacity(num_of_image_1d);
            for _ in 0..num_of_image_1d {
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
            draw_range: (0, 0, 0, 0),
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
    pub fn add_pixel(&mut self, _owner: &Node2D, x: usize, y: usize, p: u8) {
        self.world.set(x, y, p);
        self.draw_range.0 = self.draw_range.0.min(x / IMAGE_SIZE);
        self.draw_range.1 = self.draw_range.1.max(x / IMAGE_SIZE + 1);
        self.draw_range.2 = self.draw_range.2.min(y / IMAGE_SIZE);
        self.draw_range.3 = self.draw_range.3.max(y / IMAGE_SIZE + 1);
    }

    #[export]
    pub fn update_camera_rect(&mut self, _owner: &Node2D, camera_rect: Rect2) {
        let min_chunk_x = (camera_rect.origin.x.max(0.) as usize) / IMAGE_SIZE;
        let min_chunk_y = (camera_rect.origin.y.max(0.) as usize) / IMAGE_SIZE;
        let max_chunk_x = (((camera_rect.origin.x + camera_rect.size.width) as usize) / IMAGE_SIZE
            + 1)
        .min(WORLD_SIZE / IMAGE_SIZE);
        let max_chunk_y =
            (((camera_rect.origin.y + camera_rect.size.height) as usize) / IMAGE_SIZE + 1)
                .min(WORLD_SIZE / IMAGE_SIZE);
        self.draw_range = (min_chunk_x, max_chunk_x, min_chunk_y, max_chunk_y);
        println!("{:?} -> {:?}", camera_rect, self.draw_range);
    }

    // 目前只draw第0行0的chunk
    fn draw(&mut self, owner: &Node2D) {
        if self.draw_range.1 - self.draw_range.0 > 3 || self.draw_range.3 - self.draw_range.2 > 3 {
            return;
        }
        unsafe {
            for chunk_y in self.draw_range.2..self.draw_range.3 {
                for chunk_x in self.draw_range.0..self.draw_range.1 {
                    let world_image = self.image_grid[chunk_y][chunk_x].image.assume_safe();
                    world_image.lock();
                    for y in 0usize..IMAGE_SIZE {
                        for x in 0usize..IMAGE_SIZE {
                            world_image.set_pixel(
                                x as i64,
                                y as i64,
                                self.world
                                    .fetch(x + chunk_x * IMAGE_SIZE, y + chunk_y * IMAGE_SIZE)
                                    .to_color(),
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
