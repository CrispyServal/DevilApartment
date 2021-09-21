local MainScene = {
    extends = "Node2D",

    map = nil,
    main_ui = nil,
    camera = nil,
}


require("main/common")

function MainScene:_ready()
    self.map = self:get_node("Map")
    self.main_ui = self:get_node("CanvasLayer/Control/MainUI")
    self.camera = self:get_node("MainCamera")

    self.main_ui:connect("start_game", self, "start_game")

    self:_setup_debug()
end

function MainScene:start_game()
    print("main scene start game")

    self.map:clear_all()
    self.map:generate_world()

    self.main_ui:set_visible(false)
    self:move_camera_to_start()
end

function MainScene:move_camera_to_start()
    local x = TILE_SIZE * WORLD_WIDTH / 2
    local y = TILE_SIZE * WORLD_HEIGHT / 4 * 3

    self.camera:set_position(Vector2(x, y))
end

function MainScene:_setup_debug()
    self:get_node("CanvasLayer/Control/Debug"):setup({
        map = self.map,
        root = self,
    })
end

return MainScene
