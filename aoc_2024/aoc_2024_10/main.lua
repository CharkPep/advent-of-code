iter = io.lines("./input.txt", "*l")
local n, m = 47, 47
local g = {}
local map = {}

map[1] = {}
map[n + 2] = {}
for i = 1, n + 1 do
	map[1][i] = -1e9
	map[n + 2][i] = -1e9
end

for i = 1, n * m do
	g[i] = {}
end

local function count_reacheble(g, start, goal)
	local count = 0
	local visited = {}
	local function search(node)
		if visited[node] then
			return
		end

		-- to solve part 2 remove comment out this line
		visited[node] = true
		if map[node // n + 2][node % m + 1] == goal then
			count = count + 1
		end

		for _, n in ipairs(g[node]) do
			search(n)
		end
	end

	search(start)
	return count
end

local function iiter(iter, init, step)
	local i = init - 1 or 0
	return function()
		local v = { iter() }
		if v[1] == nil then
			return nil
		end

		i = i + (step or 1)
		return i, v[1]
	end
end

for i, line in iiter(iter, 2) do
	map[i] = {}
	map[i][1] = -1e9
	map[i][m + 2] = -1e9
	for j, c in iiter(line:gmatch("%S"), 2) do
		map[i][j] = tonumber(c)
	end
end

for i = 2, #map - 1 do
	for j = 2, #map[i] - 1 do
		local cur = (i - 2) * n + j - 1
		if map[i + 1][j] - map[i][j] == 1 then
			table.insert(g[cur], cur + n)
		end
		if map[i - 1][j] - map[i][j] == 1 then
			table.insert(g[cur], cur - n)
		end
		if map[i][j - 1] - map[i][j] == 1 then
			table.insert(g[cur], cur - 1)
		end
		if map[i][j + 1] - map[i][j] == 1 then
			table.insert(g[cur], cur + 1)
		end
	end
end

local ans = 0

for i = 2, #map - 1 do
	for j = 2, #map[i] - 1 do
		local cur = (i - 2) * n + j - 1
		if map[i][j] == 0 then
			ans = ans + count_reacheble(g, cur, 9)
		end
	end
end

print(ans)
