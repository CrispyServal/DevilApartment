local DevUI = {
    extends = "Control",

    dev_add_pixel = signal("x", "y", "p")
}

function DevUI:setup(node_dict)
    self.root = node_dict.root
end

function DevUI:_on_Button0_toggled(button_pressed)
    self.selected_pixel_type = 1
end

function DevUI:_on_Button1_toggled(button_pressed)
    self.selected_pixel_type = 2
end

function DevUI:_process(delta)
    if not self.selected_pixel_type then
        return
    end
    local mouse_pos = self.root:get_global_mouse_position()
    if Input:is_action_pressed("mouse_left") then
        self:emit_signal("dev_add_pixel", mouse_pos.x, mouse_pos.y, self.selected_pixel_type)
    end
end

return DevUI
