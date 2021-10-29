use gdnative::api::{ImageTexture, ShaderMaterial};
use gdnative::prelude::*;

const WORLD_SIZE: usize = 16384usize;
const IMAGE_SIZE: usize = 256usize;
const IMAGE_CHUNK_COUNT: usize = WORLD_SIZE / IMAGE_SIZE;
const LOGIC_CHUNK_SIZE: usize = 64usize;
const LOGIC_CHUNK_COUNT: usize = WORLD_SIZE / LOGIC_CHUNK_SIZE;

const POS_BEHAVIOR_DRAW: usize = 0;
const POS_BEHAVIOR_FALL: usize = 1;
const BEHAVIOR_DRAW: u8 = 1 << POS_BEHAVIOR_DRAW;
const BEHAVIOR_FALL: u8 = 1 << POS_BEHAVIOR_FALL;

pub type PixelId = u8;
pub type Behavior = u8;

pub const fn pixel_behavior(pid: PixelId) -> Behavior {
    match pid {
        0 => 0,
        1 => BEHAVIOR_DRAW | BEHAVIOR_FALL,
        2 => BEHAVIOR_DRAW,
        _ => 0,
    }
}

fn id_to_color(pid: PixelId) -> Color {
    Color {
        r: (pid as f32) / 255.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    }
}

pub fn pixel_color(pid: PixelId) -> Color {
    id_to_color(pid)
}

#[derive(Clone, Copy)]
pub struct Pixel {
    id: PixelId,
    behavior: Behavior,
    horizontal_move: u8,
}

impl Default for Pixel {
    fn default() -> Self {
        Self { id: 0, behavior: 0, horizontal_move: 0 }
    }
}

impl Pixel {
    pub fn to_color(&self) -> Color {
        pixel_color(self.id)
    }
}

#[derive(Default, Clone, Copy)]
struct ChunkStatus {
    active: bool,
    active_next_frame: bool,
}

pub struct World {
    world: Vec<Vec<Pixel>>,
    chunk_status: Vec<Vec<ChunkStatus>>,
}

impl World {
    pub fn new_empty() -> Self {
        Self {
            world: vec![vec![Pixel::default(); WORLD_SIZE]; WORLD_SIZE],
            chunk_status: vec![vec![ChunkStatus::default(); LOGIC_CHUNK_COUNT]; LOGIC_CHUNK_COUNT],
        }
    }

    pub fn fetch(&self, x: usize, y: usize) -> &Pixel {
        &self.world[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, pid: PixelId) {
        let p = &mut self.world[y][x];
        p.id = pid;
        p.behavior = pixel_behavior(pid);
    }

    pub fn active_by_world(&mut self, cx: usize, cy: usize) {
        self.chunk_status[cy][cx].active_next_frame = true;
    }

    pub fn simulate_falling(&mut self) {
        self.setup_active();
        self.simulate_falling_for_active_chunks();
    }

    fn setup_active(&mut self) {
        for crow in self.chunk_status.iter_mut() {
            for chunk in crow.iter_mut() {
                chunk.active = chunk.active_next_frame;
                chunk.active_next_frame = false;
            }
        }
    }

    fn simulate_falling_for_active_chunks(&mut self) {
        let mut to_update = vec![];
        for (cy, crow) in self.chunk_status.iter().enumerate().rev() {
            for (cx, chunk) in crow.iter().enumerate() {
                if chunk.active {
                    to_update.push((cx, cy));
                }
            }
        }
        for (cx, cy) in to_update.iter() {
            self.simulate_falling_for_chunk(*cx, *cy);
        }
    }

    fn simulate_falling_for_chunk(&mut self, cx: usize, cy: usize) {
        for y in (cy * LOGIC_CHUNK_SIZE..(cy + 1) * LOGIC_CHUNK_SIZE).rev() {
            for x in cx * LOGIC_CHUNK_SIZE..(cx + 1) * LOGIC_CHUNK_SIZE {
                let p = self.world[y][x];
                if y + 1 < WORLD_SIZE && (p.behavior & BEHAVIOR_FALL != 0) {
                    if self.world[y + 1][x].behavior & BEHAVIOR_DRAW == 0 {
                        unsafe {
                            let pa: *mut Pixel = &mut self.world[y][x];
                            let pb: *mut Pixel = &mut self.world[y + 1][x];
                            std::ptr::swap(pa, pb);
                        }
                        self.chunk_status[(y + 1) / LOGIC_CHUNK_SIZE][cx].active_next_frame = true;
                        self.chunk_status[y / LOGIC_CHUNK_SIZE][cx].active_next_frame = true;
                    }
                }
            }
        }
    }
}

pub struct ImageChunk {
    image: Ref<Image>,
    texture: Ref<ImageTexture>,
    draw_item: Ref<Sprite>,
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct RustEntry {
    image_grid: Option<Vec<Vec<ImageChunk>>>,
    world: World,
    draw_range: (usize, usize, usize, usize),
    #[property]
    chunk_scene: Option<Ref<PackedScene>>,
}

#[methods]
impl RustEntry {
    fn new(_owner: &Node2D) -> Self {
        Self {
            image_grid: None,
            world: World::new_empty(),
            draw_range: (0, 0, 0, 0),
            chunk_scene: None,
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "debug",
            args: &[
                SignalArgument {
                    name: "cx",
                    default: Variant::from_i64(100),
                    export_info: ExportInfo::new(VariantType::I64),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "cy",
                    default: Variant::from_i64(100),
                    export_info: ExportInfo::new(VariantType::I64),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        godot_print!("Hello, RustEntry.");
        let mut image_grid = Vec::with_capacity(IMAGE_CHUNK_COUNT);
        for chunk_y in 0..IMAGE_CHUNK_COUNT {
            let mut image_row = Vec::with_capacity(IMAGE_CHUNK_COUNT);
            for chunk_x in 0..IMAGE_CHUNK_COUNT {
                let image = Image::new();
                image.create(
                    IMAGE_SIZE as i64,
                    IMAGE_SIZE as i64,
                    false,
                    Image::FORMAT_R8,
                );
                let texture = ImageTexture::new();
                texture.create(
                    IMAGE_SIZE as i64,
                    IMAGE_SIZE as i64,
                    Image::FORMAT_R8,
                    ImageTexture::STORAGE_RAW,
                );
                let node = instance_scene::<Sprite>(unsafe {
                    &self.chunk_scene.as_ref().unwrap().assume_safe()
                });
                node.set_position(Vector2::new(
                    (chunk_x as f32 + 0.5) * (IMAGE_SIZE as f32),
                    (chunk_y as f32 + 0.5) * (IMAGE_SIZE as f32),
                ));
                let shared_node = node.into_shared();
                let chunk = ImageChunk {
                    image: image.into_shared(),
                    texture: texture.into_shared(),
                    draw_item: shared_node,
                };
                let node_ref = unsafe { shared_node.assume_safe() };
                owner.add_child(node_ref, false);
                image_row.push(chunk);
            }
            image_grid.push(image_row);
        }

        self.image_grid.replace(image_grid);
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, delta: f64) {
        self.draw(owner)
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node2D, delta: f64) {
        self.collect_active_chunk_for_debug_draw(owner);
        self.simulate();
    }

    fn simulate(&mut self) {
        self.world.simulate_falling();
    }

    fn active_by_world(&mut self, owner: &Node2D, cx: usize, cy: usize) {
        self.world.active_by_world(cx, cy);
    }

    fn collect_active_chunk_for_debug_draw(&self, owner: &Node2D) {
        for (cy, crow) in self.world.chunk_status.iter().enumerate() {
            for (cx, chunk) in crow.iter().enumerate() {
                if chunk.active {
                    owner.emit_signal(
                        "debug",
                        &[Variant::from_i64(cx as i64), Variant::from_i64(cy as i64)],
                    );
                }
            }
        }
    }

    #[export]
    pub fn add_pixel(&mut self, owner: &Node2D, x: usize, y: usize, pid: u8) {
        self.world.set(x, y, pid);
        // active draw
        self.draw_range.0 = self.draw_range.0.min(x / IMAGE_SIZE);
        self.draw_range.1 = self.draw_range.1.max(x / IMAGE_SIZE + 1);
        self.draw_range.2 = self.draw_range.2.min(y / IMAGE_SIZE);
        self.draw_range.3 = self.draw_range.3.max(y / IMAGE_SIZE + 1);
        self.active_by_world(owner, x / LOGIC_CHUNK_SIZE, y / LOGIC_CHUNK_SIZE);
    }

    #[export]
    pub fn update_camera_rect(&mut self, _owner: &Node2D, camera_rect: Rect2) {
        let min_chunk_x = (camera_rect.origin.x.max(0.) as usize) / IMAGE_SIZE;
        let min_chunk_y = (camera_rect.origin.y.max(0.) as usize) / IMAGE_SIZE;
        let max_chunk_x = (((camera_rect.origin.x + camera_rect.size.width) as usize) / IMAGE_SIZE
            + 1)
        .min(IMAGE_CHUNK_COUNT);
        let max_chunk_y =
            (((camera_rect.origin.y + camera_rect.size.height) as usize) / IMAGE_SIZE + 1)
                .min(IMAGE_CHUNK_COUNT);
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
                    let world_image = self.image_grid.as_ref().unwrap()[chunk_y][chunk_x]
                        .image
                        .assume_safe();
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
                    let world_texture = self.image_grid.as_ref().unwrap()[chunk_y][chunk_x]
                        .texture
                        .assume_safe();
                    world_texture.set_data(world_image);
                    let item = self.image_grid.as_ref().unwrap()[chunk_y][chunk_x]
                        .draw_item
                        .assume_safe();
                    //item.set_texture(world_texture);
                    let material: Ref<ShaderMaterial> = item.material().unwrap().cast().unwrap();
                    material.assume_safe().set_shader_param("my_texture", world_texture);
                }
            }
        }
    }
}

fn instance_scene<Root>(scene: &PackedScene) -> Ref<Root, Unique>
where
    Root: gdnative::object::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .unwrap();
    let instance = unsafe { instance.assume_unique() };

    instance.try_cast::<Root>().unwrap()
}
