extends Node2D

var thread_pool: Array

const WORLD_HEIGHT:int = Consts.WORLD_HEIGHT
const WORLD_WIDTH:int = Consts.WORLD_WIDTH
const CHUNK_SIZE:int = Consts.CHUNK_SIZE
const HALF_CHUNK_SIZE: int = Consts.HALF_CHUNK_SIZE
const THREAD_COUNT: int = Consts.THREAD_COUNT
const TEXTURE_SIZE: int = Consts.TEXTURE_SIZE
const HC_PER_TEXTURE: int = Consts.HC_PER_TEXTURE
const CHUNK_PER_TEXTURE = TEXTURE_SIZE / CHUNK_SIZE


# Called when the node enters the scene tree for the first time.
func _ready():
    assert(TEXTURE_SIZE >= HALF_CHUNK_SIZE)
    init_world()
    init_textures()
    init_image_updater()
    
func _physics_process(_delta):
    # print("tick")
    tick_simulate()
    draw_all()

func init_thread_pool(n: int):
    thread_pool = []
    for _i in range(n):
        var thread = Thread.new()
        thread_pool.append(thread)

#func init_chunks():
    #init_world()
    #init_chunk_updaters()
    #init_textures()
    
var pixel_world
func init_world():
    var Class  = load("res://main/fallingsand_native/PixelWorld.gdns")
    pixel_world = Class.new()
          
var image_updater
func init_image_updater():
    var Class  = load("res://main/fallingsand_native/ImageUpdater.gdns")
    image_updater = Class.new()
    
func tick_simulate():
    handle_debug_input()
    pixel_world.pre_simulate()
    pixel_world.simulate()
    

var texture_grid: Array
export var image_chunk_scene: PackedScene
func init_textures():
    var row_count = WORLD_HEIGHT / TEXTURE_SIZE
    var col_count = WORLD_WIDTH / TEXTURE_SIZE
    texture_grid = []
    texture_grid.resize(row_count)
    for y in range(row_count):
        var texture_row_array = []
        texture_row_array.resize(col_count)
        texture_grid[y] = texture_row_array
        for x in range(col_count):
            var t = image_chunk_scene.instance()
            t.position = Vector2((x + 0.5) * TEXTURE_SIZE, (y + 0.5) * TEXTURE_SIZE)
            add_child(t)
            texture_row_array[x] = t


func _on_MainCamera_update_camera_rect(rect: Rect2):
    var visible_row_min = int(rect.position.y / TEXTURE_SIZE)
    var visible_col_min = int(rect.position.x / TEXTURE_SIZE)
    var visible_row_max = int((rect.position.y + rect.size.y) / TEXTURE_SIZE) + 1
    var visible_col_max = int((rect.position.x + rect.size.x) / TEXTURE_SIZE) + 1
    image_updater.update_visible_range(visible_col_min, visible_col_max, visible_row_min, visible_row_max)
    #prints("rect: ", visible_row_min, visible_row_max, visible_col_min, visible_col_max)

func draw_all():
    image_updater.draw_all(pixel_world, texture_grid)
    #for y in range(visible_row_min, visible_row_max):
     #   for x in range(visible_col_min, visible_col_max):
      #      if y < 0 or y >= texture_grid.size():
       #         return
        #    var texture_row_array = texture_grid[y]
         #   if x < 0 or x >= texture_row_array.size():
          #      return
           # var t = texture_row_array[x]
            #var world_x = x * TEXTURE_SIZE
            #var world_y = y * TEXTURE_SIZE
            #for yy in range(CHUNK_PER_TEXTURE):
            #    for xx in range(CHUNK_PER_TEXTURE):
             #       pass
              #      if pixel_world.is_chunk_active(x * CHUNK_PER_TEXTURE + xx, y * CHUNK_PER_TEXTURE + yy):
               #         t.update_image(pixel_world, world_x, world_y, xx * CHUNK_SIZE, yy * CHUNK_SIZE)

var add_queue: Array = []
func _on_DevUI_dev_add_pixel(x, y, p):
    var ix = int(x)
    var iy = int(y)
    if ix < 0 or ix > WORLD_WIDTH - 1 or iy < 0 or iy > WORLD_HEIGHT - 1:
        return
    add_queue.append({"x": x, "y": y, "p": p})
    
func handle_debug_input():
    for add_op in add_queue:
        var ix = int(add_op["x"])
        var iy = int(add_op["y"])
        var p = int(add_op["p"])
        #var updater: ChunkUpdater = chunk_updaters[iy / CHUNK_SIZE][ix / CHUNK_SIZE]
        #var hc: HalfChunk = half_chunk_grid[iy / HALF_CHUNK_SIZE][ix / HALF_CHUNK_SIZE]
        #updater.set_pixel(ix, iy, p)
        #world_buffer.set_pixel(ix, iy, p)
        pixel_world.user_set_pixel(ix, iy, p)
    add_queue = []
