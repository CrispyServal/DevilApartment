-- 这个世界的所有方块都在这里
local M = {}

M.EMPTY = 1
M.DIRT = 2
M.IRON = 3
M.WOOD = 4

M.data = {
    [M.EMPTY] =  {tile_id = -1, name = "无"},
    [M.DIRT] = {tile_id = 0, name = "普通土"},
    [M.IRON] = {tile_id = 1, name = "光滑的铁"},
    [M.WOOD] = {tile_id = 2, name = "普通木头"},
}

M.reverse_data = {}
for id, table_item in pairs(M.data) do
    M.reverse_data[table_item.tile_id] = id
end

function M:get_id_from_tile_id(tile_id)
    return M.reverse_data[tile_id]
end

TileTable = M
