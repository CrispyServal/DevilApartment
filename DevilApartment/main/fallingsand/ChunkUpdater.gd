extends Reference
class_name ChunkUpdater

# 中间2x2是更新主体，边上是可能被更新的内容

# 标记自己的位置，chunk层面的下标
var row: int
var col: int
const HALF_CHUNK_SIZE: int = Consts.HALF_CHUNK_SIZE
const CHUNK_SIZE: int = Consts.CHUNK_SIZE
const CHUNK_COUNT_Y: int = Consts.CHUNK_COUNT_Y
const CHUNK_COUNT_X: int = Consts.CHUNK_COUNT_X
const WORLD_WIDTH: int = Consts.WORLD_WIDTH
const WORLD_HEIGHT: int = Consts.WORLD_HEIGHT

var world_buffer

var start_x: int
var start_y: int


func init(row, col, world_buffer):
    self.row = row
    self.col = col
    self.start_x = col * CHUNK_SIZE
    self.start_y = row * CHUNK_SIZE
    self.world_buffer = world_buffer

func simulate():
    #print("updater simulating: %d, %d" % [col, row])
    #print("y range: (%d, %d); x range: (%d, %d)" % [start_y + CHUNK_SIZE, start_y, start_x, start_x + CHUNK_SIZE])
    for y in range(start_y + CHUNK_SIZE - 1, start_y -1, -1):
        for x in range(start_x, start_x + CHUNK_SIZE):
            var p = get_pixel(x, y) # 这里肯定能取到
            #print("p = %x" % p)
            var is_fall = Pixel.is_fall(p)
            if is_fall:
                var dy = Pixel.get_dy(p)
                #print("(%d, %d) is fall. dy = %d" % [x, y, dy])
                var final_y = y
                var is_stop = false
                for yy in range(y + 1, y + dy + 1):
                    #print("yy = %d" % yy)
                    if not can_get_pixel(x, yy):
                        #print("cannot get")
                        is_stop = true
                        break
                    var check_target = get_pixel(x, yy)
                    #print("checktarget: %x" % check_target)
                    if not Pixel.is_empty(check_target):
                        #print("not empty")
                        is_stop = true
                        break
                    final_y = yy
                #print("final y = %s" % final_y)
                if not is_stop:
                   p = Pixel.add_speed(p)
                if final_y != y:
                    var tmp = get_pixel(x, final_y)
                    set_pixel(x, final_y, p)
                    set_pixel(x, y, tmp)



# 以当前chunk为起点的x和y来索引像素，内部自动找hc
func get_pixel(x, y):
    #assert(x >= -HALF_CHUNK_SIZE)
    #assert(x < 3 * HALF_CHUNK_SIZE)
    #assert(y >= -HALF_CHUNK_SIZE)
    #assert(y < 3 * HALF_CHUNK_SIZE)
    #var hc = hc_grid[(y + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE][(x + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE]
    #return hc.get_buffer()[y % HALF_CHUNK_SIZE][x % HALF_CHUNK_SIZE]
    return world_buffer.get_pixel(x, y)

func set_pixel(x, y, p):
    #var hc = hc_grid[(y + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE][(x + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE]
    world_buffer.set_pixel(x, y, p)

func can_get_pixel(x, y):
    return x >= 0 and y >= 0 and x < WORLD_WIDTH and y < WORLD_HEIGHT
