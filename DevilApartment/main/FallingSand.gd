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
    init_thread_pool(THREAD_COUNT)
    init_chunks()
    prepare_thread()
    
func _physics_process(_delta):
    # print("tick")
    tick_simulate()
    draw_all()

func init_thread_pool(n: int):
    thread_pool = []
    for _i in range(n):
        var thread = Thread.new()
        thread_pool.append(thread)

func init_chunks():
    init_world()
    init_chunk_updaters()
    init_textures()
    
var world_buffer
func init_world():
    var WorldBufferClass = load("res://main/fallingsand/WorldBuffer.gd")
    world_buffer = WorldBufferClass.new()
    world_buffer.init()

var chunk_updaters: Array

func init_chunk_updaters():
    var ChunkUpdater = load("res://main/fallingsand/ChunkUpdater.gd")
    chunk_updaters = []
    var row_count = WORLD_HEIGHT / CHUNK_SIZE
    var col_count = WORLD_WIDTH / CHUNK_SIZE
    chunk_updaters.resize(row_count)
    for chunk_row in range(row_count):
        var updater_row_array = []
        updater_row_array.resize(col_count)
        chunk_updaters[chunk_row] = updater_row_array
        for chunk_col in range(col_count):
            var updater = ChunkUpdater.new()
            updater.init(chunk_row, chunk_col, world_buffer)
            updater_row_array[chunk_col] = updater
            
func tick_simulate():
    handle_debug_input()
    pre_simulate()
    simulate_phase(0, 0)
    #print("0, 0 ok")
    simulate_phase(0, 1)
    #print("0, 1 ok")
    simulate_phase(1, 1)
    #print("1, 1 ok")
    simulate_phase(1, 0)
    #print("1, 0 ok")

func pre_simulate():
    world_buffer.pre_simulate()

var updater_queues: Array # 队列的列表
var mutex_array: Array # 每个锁用来一个队列
var semaphore_array: Array # 控制每个线程的开始或结束
var back_semaphore_array: Array
func prepare_thread():
    updater_queues = []
    updater_queues.resize(THREAD_COUNT)
    mutex_array = []
    mutex_array.resize(THREAD_COUNT)
    semaphore_array = []
    semaphore_array.resize(THREAD_COUNT)
    back_semaphore_array = []
    back_semaphore_array.resize(THREAD_COUNT)
    for i in range(THREAD_COUNT):
        mutex_array[i] = Mutex.new()
        semaphore_array[i] = Semaphore.new()
        back_semaphore_array[i] = Semaphore.new()
        updater_queues[i] = []
    for i in range(THREAD_COUNT):
        thread_pool[i].start(self, "simulate_worker", i)
    
func simulate_phase(row_mode, col_mode):
    for i in range(THREAD_COUNT):
        mutex_array[i].lock()
        updater_queues[i].clear()
        mutex_array[i].unlock()
    var row_count = WORLD_HEIGHT / HALF_CHUNK_SIZE / 2
    var col_count = WORLD_WIDTH / HALF_CHUNK_SIZE / 2
    var thread_index = 0
    for chunk_row in range(row_count):
        var hc_row = chunk_row * 2
        for chunk_col in range(col_count):
            var hc_col = chunk_col * 2
            var updater: ChunkUpdater = chunk_updaters[chunk_row][chunk_col]
            if world_buffer.is_chunk_active(chunk_col, chunk_row) and hc_row % 2 == row_mode and hc_col % 2 == col_mode:
                mutex_array[thread_index].lock()
                updater_queues[thread_index].append(updater)
                mutex_array[thread_index].unlock()
                thread_index = (thread_index + 1) % THREAD_COUNT
    for i in range(THREAD_COUNT):
        # start working
        semaphore_array[i].post()
    for i in range(THREAD_COUNT):
        while true:
            back_semaphore_array[i].wait()
            break
    
func simulate_worker(index):
    while true:
        semaphore_array[index].wait()
        
        mutex_array[index].lock()
        for updater in updater_queues[index]:
            updater.simulate()
        mutex_array[index].unlock()
        
        back_semaphore_array[index].post()

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

# 贴图层面
var visible_row_min: int = 0
var visible_row_max: int = 1
var visible_col_min: int = 0
var visible_col_max: int = 1
func _on_MainCamera_update_camera_rect(rect: Rect2):
    visible_row_min = int(rect.position.y / TEXTURE_SIZE)
    visible_col_min = int(rect.position.x / TEXTURE_SIZE)
    visible_row_max = int((rect.position.y + rect.size.y) / TEXTURE_SIZE) + 1
    visible_col_max = int((rect.position.x + rect.size.x) / TEXTURE_SIZE) + 1
    #prints("rect: ", visible_row_min, visible_row_max, visible_col_min, visible_col_max)

func draw_all():
    for y in range(visible_row_min, visible_row_max):
        for x in range(visible_col_min, visible_col_max):
            if y < 0 or y >= texture_grid.size():
                return
            var texture_row_array = texture_grid[y]
            if x < 0 or x >= texture_row_array.size():
                return
            var t = texture_row_array[x]
            var world_x = x * TEXTURE_SIZE
            var world_y = y * TEXTURE_SIZE
            for yy in range(CHUNK_PER_TEXTURE):
                for xx in range(CHUNK_PER_TEXTURE):
                    if world_buffer.is_chunk_active(x * CHUNK_PER_TEXTURE + xx, y * CHUNK_PER_TEXTURE + yy):
                        t.update_image(world_buffer, world_x, world_y, xx * CHUNK_SIZE, yy * CHUNK_SIZE)

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
        var p = add_op["p"]
        #var updater: ChunkUpdater = chunk_updaters[iy / CHUNK_SIZE][ix / CHUNK_SIZE]
        #var hc: HalfChunk = half_chunk_grid[iy / HALF_CHUNK_SIZE][ix / HALF_CHUNK_SIZE]
        #updater.set_pixel(ix, iy, p)
        world_buffer.set_pixel(ix, iy, p)
    add_queue = []
