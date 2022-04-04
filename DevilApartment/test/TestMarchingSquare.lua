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

-- 先把lines展平成数组
-- 每个元素是一条线，需要标明起始和结束
local function segment(from_x, from_y, to_x, to_y)
    return {
        from = {x = from_x, y = from_y},
        to = {x = to_x, y = to_y},
    }
end

local function flatten_lines(lines)
    local flat_lines = {}
    for _, lines_row in ipairs(lines) do
        for _, line in ipairs(lines_row) do
            if line.line then
                for i = 1, #line.line / 2 do
                    local from = line.line[i]
                    local to = line.line[i + 1]
                    table.insert(flat_lines, segment(line.u + from.x, line.v + from.y, line.u + to.x, line.v + to.y))
                end
            end
        end
    end
    return flat_lines
end

local MIN = 0.0001
local function same_number(x1, x2)
    return math.abs(x1 - x2) < MIN
end

local function same_point(p1, p2)
    return same_number(p1.x, p2.x) and same_number(p1.y, p2.y)
end

local function format_point(p)
    return string.format("(%s, %s)", p.x, p.y)
end

-- 需要输出连成线的点集
-- 输入是之前的self.lines
-- 先假设只有一条线，即没有空洞
local function connect(lines)
    local points = {}
    if #lines == 0 then
        return points
    end
    local line0 = table.remove(lines)
    table.insert(points, line0.from)
    table.insert(points, line0.to)
    while #lines > 0 do
        local current = points[#points]
        local found = false
        for i = #lines, 1, -1 do
            local line = lines[i]
            if line.from.x == current.x and line.from.y == current.y then
                if not same_point(current, line.to) then
                    table.insert(points, line.to)
                    table.remove(lines, i)
                end
                found = true
                break
            end
            -- 方向可逆
            if line.to.x == current.x and line.to.y == current.y then
                if not same_point(current, line.from) then
                    table.insert(points, line.from)
                    table.remove(lines, i)
                end
                found = true
                break
            end

        end
        assert(found, string.format("current is (%d, %d)", current.x, current.y))
    end
    return points
end

local function dot(v1, v2)
    return v1.x * v2.x + v1.y * v2.y
end

local function mod(v)
    return math.sqrt(dot(v, v))
end

local function perpendicular_distance(p, line_from, line_to)
    local v_line = {
        x = line_to.x - line_from.x,
        y = line_to.y - line_from.y
    }
    -- 垂线
    local v_line_p = {
        x = - v_line.y,
        y = v_line.x
    }
    local v_p = {
        x = p.x - line_from.x,
        y = p.y - line_from.y,
    }
    if (v_p.x == 0 and v_line_p.x == 0) or same_number( v_p.y / v_p.x, v_line_p.y / v_line_p.x) then
        return 0
    end
    local dis = math.abs(dot(v_p, v_line_p)) / mod(v_line_p)
    return dis
end

-- 从l到r的闭区间
-- index从0开始算
local function rdp(points, e, l, r)
    -- print(string.format("lr: %s,%s", l, r))
    local dmax = 0
    local index = 0
    assert(r > l)
    if r - l <= 1 then
        return {
            points[l],
            points[r],
        }
    end
    -- 跳过起始点
    for i = l + 1, r - 1 do
        local d = perpendicular_distance(points[i], points[l], points[r])
        --[[ print(string.format(
            "dis from %s to [%s, %s] is %s",
            format_point(points[i]),
            format_point(points[l]),
            format_point(points[r]),
            d
        ))]]
        if d > dmax then
            index = i
            dmax = d
        end
    end
    if dmax > e then
        local rec1 = rdp(points, e, l, index)
        local rec2 = rdp(points, e, index, r)

        local result = {}
        for i = 1, #rec1 - 1 do -- -1是去掉index这个点？
            table.insert(result, rec1[i])
        end
        for i = 1, #rec2 do
            table.insert(result, rec2[i])
        end
        print(string.format("lr: %s,%s, d = %s, index = %s", l, r, dmax, index))
        print_r(result)
        return result
    else
        print(string.format("lr: %s,%s, d = %s, index = %s", l, r, dmax, index))
        print_r({
            points[l], points[r]
        })
        return {
            points[l], points[r]
        }
    end
end

function TestMarchingSquare:_ready()
    self.sprite = self:get_node("Sprite")
    local image = self.sprite:get_texture():get_data()
    self.sprite:set_position(image:get_size() / 2)
    local lines = marching_square(image)
    self.lines = flatten_lines(lines)

    local points = connect(self.lines)
    self.points = points
    --self.lines = lines

    --[[ debug
    local ps = {}
    for i = 1, 100 do
        table.insert(ps, {
            x = i,
            y = math.random() * 20
        })
    end

    self.points = ps
    ]]

    --print_r(self.points)
    self.simple_points = rdp(self.points, 0.8, 1, #self.points - 1)
    table.insert(self.simple_points, self.points[#self.points])
    -- print_r(self.simple_points)
    self:update()
end

function TestMarchingSquare:draw_lines()
    if not self.lines then
        return
    end
    print("draw")
    for _, line in ipairs(self.lines) do
        self:draw_line(Vector2(line.from.x, line.from.y), Vector2(line.to.x, line.to.y), Color(1, 0, 0))
    end
end

function TestMarchingSquare:draw_points_inner(points, color, offset)
    if not points then
        return
    end
    print("draw")
    for i = 1, #points - 1 do
        local from = points[i]
        local to = points[i + 1]
        self:draw_line(Vector2(from.x, from.y + offset), Vector2(to.x, to.y + offset), color)
    end

end

function TestMarchingSquare:draw_points()
    self:draw_points_inner(self.points, Color(0, 1, 0), 0)
end

function TestMarchingSquare:draw_simple_points()
    self:draw_points_inner(self.simple_points, Color(1, 0, 0), 0)
end


function TestMarchingSquare:_draw()
    --self:draw_lines()
    self:draw_points()
    self:draw_simple_points()
end

function TestMarchingSquare:_input(event)
    if event:is_class("InputEventMouseMotion") then
        local mouse_pos = self:get_global_mouse_position()
        self:get_node("Control/Label"):set_text(format_point(mouse_pos))
    end
end




return TestMarchingSquare
