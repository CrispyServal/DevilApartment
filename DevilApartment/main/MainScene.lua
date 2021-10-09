local MainScene = {
    extends = "Node2D",

    map = nil,
    main_ui = nil,
    camera = nil,
}


require("main/common")

function MainScene:_ready()
    self.main_ui = self:get_node("CanvasLayer/Control/MainUI")
    self.camera = self:get_node("MainCamera")

    self.main_ui:connect("start_game", self, "start_game")

    self:_setup_debug()
end

function MainScene:start_game()
    print("main scene start game")

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
        root = self,
    })
end

function MainScene:_process(delta)
    local camera_pos = self.camera:get_position()
    local offset = delta * 100
    if Input:is_action_pressed("ui_left") then
        print("ui left")
        self.camera:set_position(Vector2(camera_pos.x - offset, camera_pos.y))
    end
    camera_pos = self.camera:get_position()
    if Input:is_action_pressed("ui_right") then
        print("ui right")
        self.camera:set_position(Vector2(camera_pos.x + offset, camera_pos.y))
    end
    camera_pos = self.camera:get_position()
    if Input:is_action_pressed("ui_down") then
        print("ui down")
        self.camera:set_position(Vector2(camera_pos.x, camera_pos.y + offset))
    end
    camera_pos = self.camera:get_position()
    if Input:is_action_pressed("ui_up") then
        print("ui up")
        self.camera:set_position(Vector2(camera_pos.x, camera_pos.y - offset))
    end
end

-- function MainScene:_input(event)
--     if event:is_class("InputEventKey") then
--         if event:is_pressed() and event:get_scancode() == KEY_ESCAPE then
--         end
--     end
-- end

return MainScene
