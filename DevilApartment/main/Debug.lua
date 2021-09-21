local Debug = {
    extends = "VBoxContainer"
}

require("table_data/tile_table")

function Debug:_ready()
    self.tile_label = self:get_node("Tile")
end

function Debug:setup(node_dict)
    self.map = node_dict.map
    self.root = node_dict.root
end

function Debug:_input(event)
    if event:is_class("InputEventMouseMotion") then
        local mouse_pos = self.root:get_global_mouse_position()
        local x = mouse_pos.x
        local y = mouse_pos.y
        local cell = self.map:get_cell(x / TILE_SIZE, y / TILE_SIZE)
        local id = TileTable:get_id_from_tile_id(cell)
        if not id then
            return
        end
        local tile_name = TileTable.data[id].name
        self.tile_label:set_text(
            string.format("(%s,%s):%s", x, y, tile_name)
        )
    end
end

return Debug
