local MainScene = {
    extends = "Node2D",

    map = nil,
    main_ui = nil
}

function MainScene:_ready()
    self.map = self:get_node("Map")
    self.main_ui = self:get_node("Control/MainUI")

    self.main_ui:connect("start_game", self, "start_game")
end

function MainScene:start_game()
    print("main scene start game")

    self.map:clear_all()
    self.main_ui:set_visible(false)
end

return MainScene
