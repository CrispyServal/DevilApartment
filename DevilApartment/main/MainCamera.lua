local MainCamera = {
    extends = "Camera2D",
    view_rect = nil,
}

function MainCamera:setup(node_dict)
    print("im camera setup")
    print_r(node_dict)
    self.rust_entry = node_dict.rust_entry
end

-- 所有的相机控制得在这

function MainCamera:_process(delta)
    local camera_pos = self:get_position()
    local zoom = self:get_zoom()
    local offset = delta * 100 * zoom:length()
    local rect_changed = false
    if Input:is_action_pressed("ui_left") then
        self:set_position(Vector2(camera_pos.x - offset, camera_pos.y))
        rect_changed = true
    end
    camera_pos = self:get_position()
    if Input:is_action_pressed("ui_right") then
        self:set_position(Vector2(camera_pos.x + offset, camera_pos.y))
        rect_changed = true
    end
    camera_pos = self:get_position()
    if Input:is_action_pressed("ui_down") then
        self:set_position(Vector2(camera_pos.x, camera_pos.y + offset))
        rect_changed = true
    end
    camera_pos = self:get_position()
    if Input:is_action_pressed("ui_up") then
        self:set_position(Vector2(camera_pos.x, camera_pos.y - offset))
        rect_changed = true
    end
    if Input:is_action_just_released("ui_zoom_in") then
        self:set_zoom(zoom * 0.75)
        rect_changed = true
    end
    if Input:is_action_just_released("ui_zoom_out") then
        self:set_zoom(zoom * 1.25)
        rect_changed = true
    end
    if rect_changed then
        self:calculate_rect()
    end
end

function MainCamera:calculate_rect()
    local ctrans = self:get_canvas_transform()
    local min_pos = -ctrans:get_origin() / ctrans:get_scale()

    local view_size = self:get_viewport_rect().size / ctrans:get_scale()
    self.view_rect = Rect2(
        min_pos,
        view_size
    )
    self.rust_entry:update_camera_rect(self.view_rect)
end

function MainCamera:get_view_rect()
    return self.view_rect
end

return MainCamera
