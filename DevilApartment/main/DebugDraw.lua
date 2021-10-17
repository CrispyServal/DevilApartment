local DebugDraw = {
    extends = "Node2D",
    enabled = true,
}

function DebugDraw:_ready()
    self.to_draw = {}
end

function DebugDraw:_draw()
    -- self:draw_rect(Rect2(Vector2(0, 0), Vector2(200, 200)), Color(1.0, 0, 0), false)
    for _, to_draw_chunk in ipairs(self.to_draw) do
        local cx = to_draw_chunk[1]
        local cy = to_draw_chunk[2]
        local rect = Rect2(Vector2(cx * 64, cy * 64), Vector2(64, 64))
        self:draw_rect(rect, Color(1.0, 0, 0), false)
        -- print(string.format("draw rect %s", rect))
    end
    self.to_draw = {}
    self.update_next = true
end

function DebugDraw:_on_RustEntry_debug(cx, cy)
    if not self.enabled then
        return
    end
    -- print(string.format("active_chunk %s, %s",cx, cy))
    table.insert(self.to_draw, {cx, cy})
    -- self:update()
end

function DebugDraw:_physics_process()
    if next(self.to_draw) or self.update_next then
        self:update()
    end
end

return DebugDraw
