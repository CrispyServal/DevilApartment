local MainScene = {
    extends = "Node2D",

    map = nil,
    main_ui = nil,
    camera = nil,
}

local CONNECT_TABLE = {
    {"dev_ui", "dev_add_pixel", "add_pixel_to_world"},
}


require("main/common")

function MainScene:_ready()
    self.main_ui = self:get_node("CanvasLayer/Control/MainUI")
    self.dev_ui = self:get_node("CanvasLayer/Control/DevUI")
    self.camera = self:get_node("MainCamera")
    self.rust_entry = self:get_node("RustEntry")

    self.main_ui:connect("start_game", self, "start_game")

    self:_setup_debug()
    self:_setup_devui()

    self:connect_all()
end

function MainScene:connect_all()
    for _, t in ipairs(CONNECT_TABLE) do
        self[t[1]]:connect(t[2], self, t[3])
    end
end

function MainScene:start_game()
    print("main scene start game")

    self.main_ui:set_visible(false)
    self.dev_ui:set_visible(true)
end

function MainScene:_setup_debug()
    self:get_node("CanvasLayer/Control/Debug"):setup({
        root = self,
    })
end


function MainScene:_setup_devui()
    self.dev_ui:setup({
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
    if Input:is_action_just_released("ui_zoom_in") then
        print("ui zoom in")
        self.camera:set_zoom(self.camera:get_zoom() * 0.75)
    end
    if Input:is_action_just_released("ui_zoom_out") then
        print("ui zoom out")
        self.camera:set_zoom(self.camera:get_zoom() * 1.25)
    end
end

function MainScene:add_pixel_to_world(x, y, p)
    -- print(string.format("sig: %s, %s", x, y))
    self.rust_entry:add_pixel(int(x), int(y), int(p))
end

-- function MainScene:_input(event)
--     if event:is_class("InputEventKey") then
--         if event:is_pressed() and event:get_scancode() == KEY_ESCAPE then
--         end
--     end
-- end

return MainScene
