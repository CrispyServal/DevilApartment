local Controller = {
    extends = "Node",

    target = nil,
}

function Controller:_ready()
    self.target = self:get_parent()
end




return Controller
