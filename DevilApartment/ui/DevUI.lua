local DevUI = {
    extends = "Control",

    dev_add_pixel = signal("x", "y", "p"),

    is_drawing = false,
}

function DevUI:setup(node_dict)
    self.root = node_dict.root
end

function DevUI:_on_Button0_toggled(button_pressed)
    self.selected_pixel_type = Pixel.PIXEL_SAND
end

function DevUI:_on_Button1_toggled(button_pressed)
    self.selected_pixel_type = Pixel.PIXEL_STONE
end

function DevUI:_process(delta)
    if self.is_drawing then
        local mouse_pos = self.root:get_global_mouse_position()
        print("draw: " .. self.selected_pixel_type)
        self:emit_signal("dev_add_pixel", mouse_pos.x, mouse_pos.y, self.selected_pixel_type)
    end
end

function DevUI:_input(event)
    if not self.selected_pixel_type then
        return
    end
    local mouse_pos = self.root:get_global_mouse_position()
    if event:is_action_pressed("mouse_left") then
        self.is_drawing = true
        self:emit_signal("dev_add_pixel", mouse_pos.x, mouse_pos.y, self.selected_pixel_type)
    end
    if event:is_action_released("mouse_left") then
        self.is_drawing = false
    end
    if self.is_drawing and event:is_class("InputEventMouseMotion") then
        self:emit_signal("dev_add_pixel", mouse_pos.x, mouse_pos.y, self.selected_pixel_type)
    end
end

return DevUI
