local MainUI = {
    extends = "Control",

    start_game = signal(),
}

function MainUI:_on_StartButton_pressed()
    print("start game")
    self:emit_signal("start_game")
end

return MainUI
