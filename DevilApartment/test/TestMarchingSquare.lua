local TestMarchingSquare = {
    extends = "Node2D",

    sprite = nil,
}

require("main/common")

local function new_cell(u, v, value)
    return {
        u = u,
        v = v,
        value = value,
    }
end

local function decide_cell_value(image, x, y)
    return image:get_pixel(x, y).a > 0
end

local function make_cells_from_image(image)
    local cells = {}
    local image_size = image:get_size()
    image:lock()
    local cells_height = image_size.y - 1
    local cells_width = image_size.x - 1
    for y = 1, cells_height do
        cells[y] = {}
        for x = 1, cells_width do
            cells[y][x] = new_cell(x - 0.5, y - 0.5, decide_cell_value(image, x - 1, y - 1))
        end
    end
    image:unlock()
    print_r(cells[30][30])
    return cells
end

local function make_mask(cells, x, y)
    local mask = 0
    if cells[y][x].value then
        mask = mask + 8
    end
    if cells[y][x + 1].value then
        mask = mask + 4
    end
    if cells[y + 1][x].value then
        mask = mask + 1
    end
    if cells[y + 1][x + 1].value then
        mask = mask + 2
    end
    return mask
end

local function make_line(x, y, mask)
    if mask == 0 then
        return nil
    elseif mask == 1 then
        return {{x=0, y=0.5},{x=0.5, y=1}}
    elseif mask == 2 then
        return {{x=0.5, y=1},{x=1, y=0.5}}
    elseif mask == 3 then
        return {{x=0, y=0.5},{x=1, y=0.5}}
    elseif mask == 4 then
        return {{x=0.5, y=0},{x=1, y=0.5}}
    elseif mask == 5 then
        return {{x=0, y=0.5},{x=0.5, y=0},{x=0.5, y=1},{x=1, y=0.5}}
    elseif mask == 6 then
        return {{x=0.5, y=0},{x=0.5, y=1}}
    elseif mask == 7 then
        return {{x=0.5, y=0},{x=0, y=0.5}}
    elseif mask == 8 then
        return {{x=0.5, y=0},{x=0, y=0.5}}
    elseif mask == 9 then
        return {{x=0.5, y=0},{x=0.5, y=1}}
    elseif mask == 10 then
        return {{x=0, y=0.5},{x=0.5, y=1},{x=0.5, y=0},{x=1, y=0.5}}
    elseif mask == 11 then
        return {{x=0.5, y=0},{x=1, y=0.5}}
    elseif mask == 12 then
        return {{x=0, y=0.5},{x=1, y=0.5}}
    elseif mask == 13 then
        return {{x=0.5, y=1},{x=1, y=0.5}}
    elseif mask == 14 then
        return {{x=0, y=0.5},{x=0.5, y=1}}
    elseif mask == 15 then
        return nil
    end
end

local function make_lines_from_cells(cells)
    local lines = {}
    for y = 1, #cells - 1 do
        lines[y] = {}
        for x = 1, #cells[y] - 1 do
            local cell = cells[y][x]
            local mask = make_mask(cells, x, y)
            local line = make_line(x, y, mask)
            lines[y][x] = {
                u = cell.u,
                v = cell.v,
                line = line,
            }
        end
    end
    return lines
end

local function marching_square(image)
    -- 使用二维数组代替转换得到的场
    local cells = make_cells_from_image(image)
    local lines = make_lines_from_cells(cells)
    print_r(lines[10][19])
    return lines
end


function TestMarchingSquare:_ready()
    self.sprite = self:get_node("Sprite")
    local image = self.sprite:get_texture():get_data()
    self.sprite:set_position(image:get_size() / 2)
    local lines = marching_square(image)
    self.lines = lines
    self:update()
end

function TestMarchingSquare:draw_lines()
    if not self.lines then
        return
    end
    print("draw")
    for _, lines_row in ipairs(self.lines) do
        for _, line in ipairs(lines_row) do
            if line.line then
                for i = 1, #line.line / 2 do
                    local from = line.line[i]
                    local to = line.line[i + 1]
                    self:draw_line(Vector2(line.u + from.x, line.v + from.y), Vector2(line.u + to.x, line.v + to.y), Color(1, 0, 0))
                end
            end
        end
    end
end

function TestMarchingSquare:_draw()
    self:draw_lines()
end



return TestMarchingSquare
