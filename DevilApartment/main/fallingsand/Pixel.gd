extends Node

# xx xx xx xx
# ?? [??, 速度] 属性 id

# id: 8bit

# 属性，假设有8种
# ？ ？ ？ ？ ？ ？ ？ 掉落否

# 速度（假设液体掉落先加速，然后达到满速，因此可以记几个状态，查表得dy）
# 暂且假设8帧达到满速，用3bit存
const PIXEL_AIR =   0x0000_0000
const PIXEL_SAND =  0x0000_0101
const PIXEL_STONE = 0x0000_0002

var dy_lut = [
    1, 2, 3, 4, 5, 6, 7, 8
]

func is_empty(p: int):
    return p & 0x0f == 0

func is_fall(p: int):
    return p & 0x100 != 0 
    
func get_speed(p: int):
    return (p & 0x70000) >> 16
    
func get_dy(p: int):
    var speed = get_speed(p)
    return dy_lut[speed]

func add_speed(p: int):
    var speed = max(7, get_speed(p) + 1)
    return (p & ~(0x70000)) | (speed << 16)
