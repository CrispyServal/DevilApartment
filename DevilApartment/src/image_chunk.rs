use crate::consts::*;
use crate::PixelWorld;
use gdnative::api::ShaderMaterial;
use gdnative::{api::ImageTexture, prelude::*};

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct ImageChunk {
    image: Ref<Image>,
    texture: Ref<ImageTexture>,
}

#[methods]
impl ImageChunk {
    pub fn new(_owner: &Sprite) -> Self {
        let image = Image::new();
        image.create(
            TEXTURE_SIZE as i64,
            TEXTURE_SIZE as i64,
            false,
            Image::FORMAT_R8,
        );
        let texture = ImageTexture::new();
        texture.create(
            TEXTURE_SIZE as i64,
            TEXTURE_SIZE as i64,
            Image::FORMAT_R8,
            ImageTexture::STORAGE_RAW,
        );

        Self {
            image: image.into_shared(),
            texture: texture.into_shared(),
        }
    }

    #[export]
    pub fn update_image(
        &self,
        owner: &Sprite,
        pixel_world: Ref<Reference>,
        start_x: usize,
        start_y: usize,
        offset_x: usize,
        offset_y: usize,
    ) {
        let image = unsafe { self.image.assume_safe() };
        let pixel_world_tref = unsafe { pixel_world.assume_safe() };
        let pixel_world_instance = pixel_world_tref
            .cast_instance::<PixelWorld>()
            .expect("cast to pixel world failed");
        image.lock();
        for y in offset_y..offset_y + CHUNK_SIZE {
            for x in offset_x..offset_x + CHUNK_SIZE {
                let id = pixel_world_instance
                    .map(|pixel_world, owner| {
                        pixel_world.get_pixel(&owner, start_x + x, start_y + y)
                    })
                    .unwrap();
                image.set_pixel(
                    x as i64,
                    y as i64,
                    Color::rgb((id as f32) / (255 as f32), 0.0, 0.0),
                );
            }
        }
        image.unlock();

        let t = unsafe { self.texture.assume_safe() };
        t.set_data(image);

        let material: Ref<ShaderMaterial> = owner.material().unwrap().cast().unwrap();
        unsafe { material.assume_safe() }.set_shader_param("my_texture", t);
    }
}

/*
extends Sprite


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var image: Image
var image_texture: ImageTexture
const TEXTURE_SIZE: int = Consts.TEXTURE_SIZE
const CHUNK_SIZE = Consts.CHUNK_SIZE

# Called when the node enters the scene tree for the first time.
func update_image(pixel_world, start_x, start_y, offset_x, offset_y):
    #print("update image start: (%d, %d), offset: (%d, %d)" % [start_x, start_y, offset_x ,offset_y])
    image.lock()
    for y in range(offset_y, offset_y + CHUNK_SIZE):
        for x in range(offset_x, offset_x + CHUNK_SIZE):
            var id: int = pixel_world.get_pixel(start_x + x, start_y + y)
            #print("(%d, %d) -> %d" % [x, y, pixel])
            image.set_pixel(x, y, Color8(id, 0, 0))
    image.unlock()
    #image.save_png("res://debug_output/texture.png")
    image_texture.set_data(image)

    var mat: ShaderMaterial = material
    mat.set_shader_param("my_texture", image_texture)
    #set_texture(image_texture)

*/
