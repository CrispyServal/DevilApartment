use crate::consts::*;
use crate::Range2d;
use crate::{ImageChunk, PixelWorld};
use gdnative::prelude::*;
use rayon::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct ImageUpdater {
    range: Range2d,
}

#[methods]
impl ImageUpdater {
    pub fn new(_owner: &Reference) -> Self {
        Self {
            range: Range2d::new(0, 0, 0, 0),
        }
    }

    #[export]
    pub fn update_visible_range(
        &mut self,
        _owner: &Reference,
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
    ) {
        self.range = Range2d::new(min_x, max_x, min_y, max_y);
    }

    #[export]
    pub fn draw_all(
        &self,
        _owner: &Reference,
        pixel_world: Ref<Reference>,
        image_grid: Vec<Vec<Ref<Sprite>>>,
    ) {
        let pixel_world_tref = unsafe { pixel_world.assume_safe() };
        let pixel_world_instance = pixel_world_tref
            .cast_instance::<PixelWorld>()
            .expect("cast to pixel world failed");
        //godot_print!("{}", image_grid.len());
        let mut works = vec![];
        for y in self.range.min_y..self.range.max_y {
            for x in self.range.min_x..self.range.max_x {
                if y >= image_grid.len() {
                    continue;
                }
                let image_row = &image_grid[y];
                if x >= image_row.len() {
                    continue;
                }
                let world_x = x * TEXTURE_SIZE;
                let world_y = y * TEXTURE_SIZE;
                let mut work = ImageUpdateWork {
                    image_chunk: image_row[x],
                    sub_works: vec![],
                };
                for yy in 0..CHUNK_PER_TEXTURE {
                    for xx in 0..CHUNK_PER_TEXTURE {
                        let active = pixel_world_instance
                            .map(|pixel_world, owner| {
                                pixel_world.is_chunk_active(
                                    &owner,
                                    x * CHUNK_PER_TEXTURE + xx,
                                    y * CHUNK_PER_TEXTURE + yy,
                                )
                            })
                            .unwrap();
                        if active {
                            work.sub_works.push(ImageUpdateSubWork {
                                start_world_x: world_x,
                                start_world_y: world_y,
                                offset_x: xx * CHUNK_SIZE,
                                offset_y: yy * CHUNK_SIZE,
                            })
                        }
                    }
                }
                if !work.sub_works.is_empty() {
                    works.push(work);
                }
            }
        }
        works.iter().for_each(|work| {
            let image_chunk_tref = unsafe { work.image_chunk.assume_safe() };
            let image_chunk_instance = image_chunk_tref.cast_instance::<ImageChunk>().unwrap();
            image_chunk_instance
                .map(|image_chunk, owner| {
                    for sub_work in work.sub_works.iter() {
                        image_chunk.update_image(
                            &owner,
                            pixel_world.clone(),
                            sub_work.start_world_x,
                            sub_work.start_world_y,
                            sub_work.offset_x,
                            sub_work.offset_y,
                        );
                    }
                })
                .unwrap();
        })
    }
}

struct ImageUpdateWork {
    pub image_chunk: Ref<Sprite>,
    pub sub_works: Vec<ImageUpdateSubWork>,
}

struct ImageUpdateSubWork {
    pub start_world_x: usize,
    pub start_world_y: usize,
    pub offset_x: usize,
    pub offset_y: usize,
}
