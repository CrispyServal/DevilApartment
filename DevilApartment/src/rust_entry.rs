use gdnative::api::ImageTexture;
use gdnative::prelude::*;

const WORLD_SIZE: usize = 16384usize;
const IMAGE_SIZE: i64 = 16384i64;

pub type PixelId = u8;

#[derive(Default, Clone, Copy)]
pub struct Pixel {
    id: PixelId,
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
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct RustEntry {
    world_image: Ref<Image>,
    world_texture: Ref<ImageTexture>,
    world: World,
}

#[methods]
impl RustEntry {
    fn new(_owner: &Node2D) -> Self {
        let image = Image::new();
        image.create(IMAGE_SIZE, IMAGE_SIZE, false, Image::FORMAT_RGB8);
        let texture = ImageTexture::new();
        texture.create(
            IMAGE_SIZE,
            IMAGE_SIZE,
            Image::FORMAT_RGB8,
            ImageTexture::STORAGE_RAW,
        );
        Self {
            world_image: image.into_shared(),
            world_texture: texture.into_shared(),
            world: World::new_empty(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("Hello, world.");
        // self.world.replace(World::new_empty());
        unsafe {
            let world_image = self.world_image.assume_safe();
            world_image.lock();
            world_image.fill(Color::rgb(1., 0., 0.));
            world_image.unlock();
            self.world_texture
                .assume_safe()
                .set_data(self.world_image.assume_safe());
            let child: TRef<Sprite> = _owner
                .get_node("WorldSprite")
                .unwrap()
                .assume_safe()
                .cast::<Sprite>()
                .unwrap();
            child.set_texture(self.world_texture.assume_safe());
            child.set_position(Vector2::new(
                (IMAGE_SIZE as f32) / 2.0,
                (IMAGE_SIZE as f32) / 2.0,
            ));
        }
    }
}
