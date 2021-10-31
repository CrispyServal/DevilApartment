extends Reference
class_name HalfChunk

# 代表一个chunk和周边的一些hc，作为多线程的基本单元

var buffer: Array


# hc数量上的行列
var row: int
var col: int
var size

var active: bool = false
var active_next_frame: bool = false

func init(size, row, col):
    self.size = size
    self.row = row
    self.col = col
    buffer = []
    buffer.resize(size)
    for i in range(size):
        var bytes_row = PoolIntArray([])
        for _j in range(size):
            bytes_row.push_back(0)
        buffer[i] = bytes_row

func get_buffer():
    return buffer
    
func get_size():
    return size

func pre_simulate():
    active = active_next_frame
    active_next_frame = false

func set_pixel(x, y, p: int):
    buffer[y][x] = p
    #print("hc: (%d, %d) set (%d, %d) %x" % [col, row, x, y, buffer[y][x]])
    active_next_frame = true
