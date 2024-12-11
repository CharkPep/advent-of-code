-- Idk why this works okey
local iter = io.lines("./input.txt", "*l")
local antennas_list = {}
local antennas_idx = {}
local antinodes = {}
local n, m = 50, 50
local ans = 0

local function iiter(iter, init, step)
	local i = init or 0
	return function()
		local v = { iter() }
		if v[1] == nil then
			return nil
		end

		i = i + (step or 1)
		return i, v[1]
	end
end

local function get_diff(a, b)
	return { i = b.i - a.i, j = b.j - a.j }
end

for i = 1, n do
	antennas_idx[i] = {}
	for j = 1, m do
		antennas_idx[i][j] = false
	end
end

for i, line in iiter(iter) do
	for j, c in iiter(line:gmatch("%S")) do
		if c ~= "." then
			if not antennas_list[c] then
				antennas_list[c] = {}
			end

			table.insert(antennas_list[c], { i = i, j = j })
			antennas_idx[i][j] = true
		end
	end
end

for _, antenna in pairs(antennas_list) do
	if #antenna ~= 1 then
		ans = ans + #antenna
	end
	for i in ipairs(antenna) do
		for j = i + 1, #antenna do
			if i ~= j then
				local nodes = {}
				for k = 1, n do
					table.insert(nodes, {
						i = antenna[i].i - k * get_diff(antenna[i], antenna[j]).i,
						j = antenna[i].j - k * get_diff(antenna[i], antenna[j]).j,
					})
					table.insert(nodes, {
						i = antenna[j].i - k * get_diff(antenna[j], antenna[i]).i,
						j = antenna[j].j - k * get_diff(antenna[j], antenna[i]).j,
					})
				end

				for _, antinode in ipairs(nodes) do
					if
						antinode.i > 0
						and antinode.i <= n
						and antinode.j > 0
						and antinode.j <= m
						and not antennas_idx[antinode.i][antinode.j]
					then
						antinodes[antinode.i .. " " .. antinode.j] = true
					end
				end
			end
		end
	end
end

for _ in pairs(antinodes) do
	ans = ans + 1
end

print(ans)
