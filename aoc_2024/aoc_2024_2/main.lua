local function slice(tbl, first, last, step)
	local sliced = {}
	for i = first or 1, last or #tbl, step or 1 do
		sliced[#sliced + 1] = tbl[i]
	end

	return sliced
end

local function read_input(path)
	local f = io.open(path, "r")
	local iter = f:read("*a"):gmatch("[^\n]+")
	return function()
		local line = iter()
		if line == nil then
			return nil
		end

		local split = line:gmatch("%d+")
		local parsed = {}
		for v in split do
			table.insert(parsed, tonumber(v))
		end

		return parsed
	end
end

local function is_descending(tbl)
	local prev = tbl[1]
	local diff = 0
	for _, v in ipairs(slice(tbl, 2, #tbl, 1)) do
		if v > prev then
			return false
		end

		prev = v
	end

	return true
end

local function is_ascending(tbl)
	local prev = tbl[1]
	for _, v in ipairs(slice(tbl, 2, #tbl, 1)) do
		if v < prev then
			return false
		end

		prev = v
	end

	return true
end

local function check_constrait(tbl)
	for i, v in ipairs(slice(tbl, 2, #tbl, 1)) do
		if math.abs(v - tbl[i]) > 3 or math.abs(v - tbl[i]) == 0 then
			return false
		end
	end

	return true
end

local ans1 = 0
for line in read_input("./input.txt") do
	if check_constrait(line) and (is_descending(line) or is_ascending(line)) then
		ans1 = ans1 + 1
	end
end

print(ans1)
local ans2 = 0
for line in read_input("./input.txt") do
	for i in ipairs(line) do
		local l = slice(line, 1, i - 1)
		for _, v in ipairs(slice(line, i + 1, #line)) do
			table.insert(l, v)
		end

		if check_constrait(l) and (is_descending(l) or is_ascending(l)) then
			ans2 = ans2 + 1
			break
		end
	end
end

print(ans2)
