extends Reference
class_name HalfChunk

# 代表一个chunk和周边的一些hc，作为多线程的基本单元

var buffer: Array
# hc数量上的行列
var row: int
var col: int
var size

var is_draw_dirty: bool = false

func init(size, row, col):
    self.size = size
    self.row = row
    self.col = col
    buffer = []
    buffer.resize(size)
    for i in range(size):
        var bytes_row = PoolByteArray([])
        for _j in range(size):
            bytes_row.push_back(0)
        buffer[i] = bytes_row

func get_buffer():
    return buffer
    
func get_size():
    return size

func set_pixel(x, y, p: int):
    buffer[y][x] = p
    print("hc: (%d, %d) add (%d, %d) %d" % [row, col, x, y, buffer[y][x]])
    is_draw_dirty = true

func simulate():
    pass
