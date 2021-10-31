extends Reference
class_name WorldBuffer

# 尝试单buffer情况
var buffer: Array

var chunk_grid: Array
const CHUNK_SIZE = Consts.CHUNK_SIZE

func init():
    buffer = []
    buffer.resize(Consts.WORLD_HEIGHT)
    for y in range(Consts.WORLD_HEIGHT):
        var bytes_row = PoolIntArray([])
        for _x in range(Consts.WORLD_WIDTH):
            bytes_row.push_back(0)
        buffer[y] = bytes_row

    chunk_grid = []
    chunk_grid.resize(Consts.CHUNK_COUNT_Y)
    for y in range(Consts.CHUNK_COUNT_Y):
        var chunk_row_array = []
        chunk_row_array.resize(Consts.CHUNK_COUNT_X * 2)
        for x in range(Consts.CHUNK_COUNT_X):
            chunk_row_array[x + 2] = false
            chunk_row_array[x * 2 + 1] = false
        chunk_grid[y] = chunk_row_array
    
func pre_simulate():
    #print("pre simulate")
    for y in range(chunk_grid.size()):
        var chunk_row_array = chunk_grid[y]
        for x in range(Consts.CHUNK_COUNT_X):
            #if chunk_row_array[x * 2] and !chunk_row_array[x * 2 + 1]:
            #    print("freeze chunk: (%d, %d)" % [x, y])
            #if !chunk_row_array[x * 2] and chunk_row_array[x * 2 + 1]:
            #    print("activate chunk: (%d, %d)" % [x, y])
            chunk_row_array[x * 2] = chunk_row_array[x * 2 + 1]
            chunk_row_array[x * 2 + 1] = false

            
func is_chunk_active(chunk_x: int, chunk_y: int):
    var is_active: bool = chunk_grid[chunk_y][chunk_x * 2]
    #if chunk_x == 0 and chunk_y == 7 and is_active:
        #print("check (%d, %d): %s" % [chunk_x ,chunk_y, is_active])
    return is_active
    
func set_chunk_active(chunk_x: int, chunk_y: int):
    chunk_grid[chunk_y][chunk_x * 2] = true
    
func set_chunk_next_active(chunk_x, chunk_y):
    #print("(%d, %d) active next" % [chunk_x ,chunk_y])
    chunk_grid[chunk_y][chunk_x * 2 + 1] = true

func get_buffer():
    return buffer

func get_pixel(x, y):
    return buffer[y][x]

func set_pixel(x, y, p: int):
    #print("set (%d, %d) to %x" % [x, y, p])
    buffer[y][x] = p
    set_chunk_next_active(x / CHUNK_SIZE, y / CHUNK_SIZE)
    
