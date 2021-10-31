extends Node2D

var thread_pool: Array

const WORLD_HEIGHT:int = Consts.WORLD_HEIGHT
const WORLD_WIDTH:int = Consts.WORLD_WIDTH
const CHUNK_SIZE:int = Consts.CHUNK_SIZE
const HALF_CHUNK_SIZE: int = Consts.HALF_CHUNK_SIZE
const THREAD_COUNT: int = Consts.THREAD_COUNT
const TEXTURE_SIZE: int = Consts.TEXTURE_SIZE
const HC_PER_TEXTURE: int = Consts.HC_PER_TEXTURE


# Called when the node enters the scene tree for the first time.
func _ready():
    assert(TEXTURE_SIZE >= HALF_CHUNK_SIZE)
    init_thread_pool(THREAD_COUNT)
    init_chunks()
    prepare_thread()
    
func _physics_process(delta):
    print("tick")
    tick_simulate()
    draw_all()

func init_thread_pool(n: int):
    thread_pool = []
    for _i in range(n):
        var thread = Thread.new()
        thread_pool.append(thread)

func init_chunks():
    init_half_chunks()
    init_chunk_updaters()
    init_textures()
    
var half_chunk_grid: Array

func init_half_chunks():
    var HalfChunk = load("res://main/fallingsand/HalfChunk.gd")
    half_chunk_grid = []
    var row_count: int = WORLD_HEIGHT / HALF_CHUNK_SIZE
    var col_count: int = WORLD_WIDTH / HALF_CHUNK_SIZE
    half_chunk_grid.resize(row_count)
    for row in range(row_count):
        var half_chunks_row = []
        half_chunks_row.resize(col_count)
        for col in range(col_count):
            var half_chunk = HalfChunk.new()
            half_chunk.init(HALF_CHUNK_SIZE, row, col)
            half_chunks_row[col] = half_chunk
        half_chunk_grid[row] = half_chunks_row

var chunk_updaters: Array

func init_chunk_updaters():
    var ChunkUpdater = load("res://main/fallingsand/ChunkUpdater.gd")
    chunk_updaters = []
    var row_count = WORLD_HEIGHT / HALF_CHUNK_SIZE / 2
    var col_count = WORLD_WIDTH / HALF_CHUNK_SIZE / 2
    chunk_updaters.resize(row_count)
    for chunk_row in range(row_count):
        var hc_row = chunk_row * 2
        var updater_row_array = []
        updater_row_array.resize(col_count)
        chunk_updaters[chunk_row] = updater_row_array
        for chunk_col in range(col_count):
            var hc_col = chunk_col * 2
            var updater = ChunkUpdater.new()
            updater.init(hc_row, hc_col, half_chunk_grid, row_count * 2, col_count * 2)
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
    for hc_row_array in half_chunk_grid:
        for hc in hc_row_array:
            hc.pre_simulate()

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
            if updater.need_simulate() and hc_row % 2 == row_mode and hc_col % 2 == col_mode:
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
            if y >= texture_grid.size():
                return
            var t = texture_grid[y][x]
            for hc_y in range(HC_PER_TEXTURE):
                for hc_x in range(HC_PER_TEXTURE):
                    var hc = half_chunk_grid[y * HC_PER_TEXTURE + hc_y][x * HC_PER_TEXTURE + hc_x]
                    if hc.active:
                        t.update_image(hc, hc_x * HALF_CHUNK_SIZE, hc_y * HALF_CHUNK_SIZE)

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
        var hc: HalfChunk = half_chunk_grid[iy / HALF_CHUNK_SIZE][ix / HALF_CHUNK_SIZE]
        hc.set_pixel(ix % HALF_CHUNK_SIZE, iy % HALF_CHUNK_SIZE, p)
    add_queue = []
