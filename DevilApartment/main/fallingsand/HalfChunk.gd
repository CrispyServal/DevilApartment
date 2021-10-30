extends Reference

# 代表一个chunk和周边的一些hc，作为多线程的基本单元

var buffer: Array
# hc数量上的行列
var row: int
var col: int

func init(size, row, col):
    self.row = row
    self.col = col
    buffer = []
    buffer.resize(size)
    for i in range(size):
        var bytes_row = PoolByteArray([])
        for _j in range(size):
            bytes_row.push_back(0)
        buffer[i] = bytes_row

func simulate():
    pass
