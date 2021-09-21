local Controller = {
    extends = "Node",

    target = nil,
}

function Controller:_ready()
    self.target = self:get_parent()
end


function Controller:_pysical_process(delta)
    print(delta)
end


return Controller
