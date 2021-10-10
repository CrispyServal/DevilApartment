local Debug = {
    extends = "VBoxContainer"
}

require("table_data/tile_table")

function Debug:_ready()
    self.tile_label = self:get_node("Tile")
    self.fps_label = self:get_node("Fps")
end

function Debug:setup(node_dict)
    self.root = node_dict.root
end

function Debug:_input(event)
    if event:is_class("InputEventMouseMotion") then
        local mouse_pos = self.root:get_global_mouse_position()
        local x = mouse_pos.x
        local y = mouse_pos.y
        self.tile_label:set_text(
            string.format("(%s,%s)", x, y)
        )
    end
end

function Debug:_process(delta)
    self.fps_label:set_text(
        string.format("%s", Engine:get_frames_per_second() )
    )
end

return Debug
