extends Reference

# 存储4x4的hc的引用（边上可能会空缺）
# 中间2x2是更新主体，边上是可能被更新的内容

var hc_grid: Array
# 标记自己的位置，hc层面的下标
var row: int
var col: int

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
