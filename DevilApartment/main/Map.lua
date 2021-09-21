local Map = {
    extends = "TileMap",
}

require("main/common")
require("table_data/tile_table")

local function make_empty_row()
    local row = {}
    for i = 1, WORLD_WIDTH do
        row[i] = -1
    end
    return row
end

-- 方块信息存在这里
-- WORLD_HEIGHT个数组，每个数组WORLD_WIDTH长
Map.world = {}

function Map:clear_all()
    self:clear_world()
    self:clear()
end

function Map:clear_world()
    self.world = {}
    for y = 1, WORLD_HEIGHT do
        self.world[y] = make_empty_row()
    end
end

-- 强制使用数据重新绘制整个地图，非常卡
function Map:force_draw()
    for y = 1, WORLD_HEIGHT do
        for x = 1, WORLD_WIDTH do
            self:set_cell(x, y, self.world[y][x])
        end
    end
end

function Map:generate_world()
    print(TileTable.data[TileTable.DIRT].tile_id)
    for y = WORLD_HEIGHT / 4 * 3, WORLD_HEIGHT do
        for x = 1, WORLD_WIDTH do
            self.world[y][x] = TileTable.DIRT
            self:set_cell(x, y, TileTable.data[TileTable.DIRT].tile_id)
        end
    end
end

return Map
