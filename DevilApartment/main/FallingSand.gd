extends Node2D

var thread_pool: Array

const WORLD_HEIGHT:int = 512
const WORLD_WIDTH:int = 2048 # maybe larger
const CHUNK_SIZE:int = 64
const HALF_CHUNK_SIZE: int = 32
const THREAD_COUNT: int = 8

# Called when the node enters the scene tree for the first time.
func _ready():
    init_thread_pool(THREAD_COUNT)
    init_chunks()

func init_thread_pool(n: int):
    thread_pool = []
    for _i in range(n):
        var thread = Thread.new()
        thread_pool.append(thread)

func init_chunks():
    init_half_chunks()
    init_chunk_updaters()
    
    tick_simulate()
    
var half_chunks: Array

func init_half_chunks():
    var HalfChunk = load("res://main/fallingsand/HalfChunk.gd")
    half_chunks = []
    var row_count: int = WORLD_HEIGHT / HALF_CHUNK_SIZE
    var col_count: int = WORLD_WIDTH / HALF_CHUNK_SIZE
    half_chunks.resize(row_count)
    for row in range(row_count):
        var half_chunks_row = []
        half_chunks_row.resize(col_count)
        for col in range(col_count):
            var half_chunk = HalfChunk.new()
            half_chunk.init(HALF_CHUNK_SIZE, row, col)
            half_chunks_row[col] = half_chunk
        half_chunks[row] = half_chunks_row

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
            updater.init(hc_row, hc_col, half_chunks, row_count * 2, col_count * 2)
            updater_row_array[chunk_col] = updater
            
func tick_simulate():
    simulate_phase(0, 0)
    simulate_phase(0, 1)
    simulate_phase(1, 1)
    simulate_phase(1, 0)
    
var updater_queues: Array
func simulate_phase(row_mode, col_mode):
    updater_queues = []
    updater_queues.resize(THREAD_COUNT)
    for i in range(THREAD_COUNT):
        updater_queues[i] = []
    var row_count = WORLD_HEIGHT / HALF_CHUNK_SIZE / 2
    var col_count = WORLD_WIDTH / HALF_CHUNK_SIZE / 2
    var thread_index = 0
    for chunk_row in range(row_count):
        var hc_row = chunk_row * 2
        for chunk_col in range(col_count):
            var hc_col = chunk_col * 2
            if hc_row % 2 == row_mode && hc_col % 2 == col_mode:
                updater_queues[thread_index].append(chunk_updaters[chunk_row][chunk_col])
                thread_index = (thread_index + 1) % THREAD_COUNT
    comsume_updater_queue()
    
func comsume_updater_queue():
    for i in range(THREAD_COUNT):
        var thread = thread_pool[i]
        thread.start(self, "simulate_worker", updater_queues[i])
    for i in range(THREAD_COUNT):
        var thread = thread_pool[i]
        thread.wait_to_finish()
    
func simulate_worker(updaters):
    for updater in updaters:
        updater.simulate()
