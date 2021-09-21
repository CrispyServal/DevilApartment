WORLD_WIDTH = 256
WORLD_HEIGHT = 256

TILE_SIZE = 16

local function tprint(tbl, indent)
  if not indent then indent = 0 end
  local toprint = string.rep(" ", indent) .. "{\n"
  indent = indent + 2
  for k, v in pairs(tbl) do
    toprint = toprint .. string.rep(" ", indent)
    if (type(k) == "number") then
      toprint = toprint .. "[" .. k .. "] = "
    elseif (type(k) == "string") then
      toprint = toprint  .. k ..  "= "
    end
    if (type(v) == "number") then
      toprint = toprint .. v .. ",\n"
    elseif (type(v) == "string") then
      toprint = toprint .. "\"" .. v .. "\",\n"
    elseif (type(v) == "table") then
      toprint = toprint .. tprint(v, indent + 2) .. ",\n"
    else
      toprint = toprint .. "\"" .. tostring(v) .. "\",\n"
    end
  end
  toprint = toprint .. string.rep(" ", indent-2) .. "}"
  return toprint
end

print_r = function(t)
    print(tprint(t, 0))
end
