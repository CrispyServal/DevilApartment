extends Reference
class_name ChunkUpdater

# 存储4x4的hc的引用（边上可能会空缺）
# 中间2x2是更新主体，边上是可能被更新的内容

var hc_grid: Array
# 标记自己的位置，hc层面的下标
var row: int
var col: int
const HALF_CHUNK_SIZE: int = Consts.HALF_CHUNK_SIZE

func init(hc_row, hc_col, all_hc, max_row, max_col):
    assert(hc_row % 2 == 0)
    assert(hc_col % 2 == 0)
    self.row = hc_row
    self.col = hc_col

    hc_grid = []
    hc_grid.resize(4)
    for row in range(4):
        var my_hc_row = []
        my_hc_row.resize(4)
        hc_grid[row] = my_hc_row
        var row_in_all_hc = hc_row - 1 + row
        if row_in_all_hc < 0 or row_in_all_hc >= max_row:
            continue
        for col in range(4):
            var col_in_all_hc = hc_col - 1 + col
            if col_in_all_hc < 0 or col_in_all_hc >= max_col:
                continue
            var hc = all_hc[row_in_all_hc][col_in_all_hc]
            my_hc_row[col] = hc
        
func simulate():      
    print("updater simulating: %d, %d" % [row, col])
    for y in range(HALF_CHUNK_SIZE * 2 - 1, -1, -1):
        for x in range(HALF_CHUNK_SIZE * 2):
            var p = get_pixel(x, y) # 这里肯定能取到
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
    var hc = hc_grid[(y + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE][(x + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE]
    return hc.get_buffer()[y % HALF_CHUNK_SIZE][x % HALF_CHUNK_SIZE]

func set_pixel(x, y, p):   
    var hc = hc_grid[(y + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE][(x + HALF_CHUNK_SIZE) / HALF_CHUNK_SIZE]
    hc.set_pixel(x % HALF_CHUNK_SIZE, y % HALF_CHUNK_SIZE, p)
    
func can_get_pixel(x, y):
    if self.row == 0 and y < 0:
        return false
    if self.row == Consts.HALF_CHUNK_COUNT_Y - 2 and y >= HALF_CHUNK_SIZE * 2:
        return false
    if self.col == 0 and x < 0:
        return false
    if self.col == Consts.HALF_CHUNK_COUNT_X - 2 and x >= HALF_CHUNK_SIZE * 2:
        return false
    return true

func need_simulate():
    return hc_grid[1][1].active or hc_grid[1][2].active or hc_grid[2][1].active or hc_grid[2][2].active
