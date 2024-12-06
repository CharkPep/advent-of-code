local graph = {}
local ans = 0
local iter = io.lines("./input.txt", "l")

function table.reverse(tabl)
	for i = 1, math.floor(#tabl / 2) + 1, 1 do
		tabl[i], tabl[#tabl - i + 1] = tabl[#tabl - i + 1], tabl[i]
	end

	return tabl
end

function table.shallow_copy(tabl)
	local copy = {}
	for _, v in ipairs(tabl) do
		copy[#copy + 1] = v
	end

	return copy
end

local function sort(g, root, include)
	local order = {}
	local visited = {}
	local function dfs(node)
		if visited[node] then
			return
		end

		visited[node] = true
		if g[node] then
			for _, visit in ipairs(g[node]) do
				if include[visit] then
					dfs(visit)
				end
			end
		end

		order[#order + 1] = node
	end

	dfs(root)
	table.reverse(order)
	return order
end

-- Find node that does not have any incomming edges.
local function find_start(g, include)
	local nodes = {}
	for n, edges in pairs(g) do
		if include[n] then
			-- There is outcomming edge, if none incomming were not recorded
			if not nodes[n] then
				nodes[n] = false
			end

			for _, en in ipairs(edges) do
				if include[en] then
					nodes[en] = true
				end
			end
		end
	end

	for node, incommming in pairs(nodes) do
		if not incommming then
			return node
		end
	end

	return nil
end

local function ord_func(sorted)
	local ord = {}
	for i, n in ipairs(sorted) do
		ord[n] = {}
		for j = i + 1, #sorted, 1 do
			ord[n][sorted[j]] = true
		end
	end

	return function(a, b)
		return ord[a][b] ~= nil
	end
end

for l in iter do
	if l == "" then
		break
	end

	local page, before = l:match("(%d+)|(%d+)")
	if not graph[page] then
		graph[page] = {}
	end

	table.insert(graph[page], before)
end

-- Part 1
-- local function seen_before(seen, pages)
-- 	for _, page in ipairs(pages) do
-- 		if seen[page] then
-- 			return true
-- 		end
-- 	end
--
-- 	return false
-- end
--
-- local less = {}
-- for l in iter do
-- 	local seen = {}
-- 	local printed = {}
-- 	for d in l:gmatch("%d+") do
-- 		table.insert(printed, d)
-- 	end
--
-- 	ans = ans + printed[math.floor(#printed / 2) + 1]
-- 	for _, d in ipairs(printed) do
-- 		if g[d] and seen_before(seen, g[d]) then
-- 			ans = ans - printed[math.floor(#printed / 2) + 1]
-- 			break
-- 		end
--
-- 		seen[d] = true
-- 	end
-- end

-- Part 2

for l in iter do
	local include = {}
	local pages = {}
	for d in l:gmatch("%d+") do
		include[d] = true
		pages[#pages + 1] = d
	end

	local root = find_start(graph, include)
	local order = sort(graph, root, include)
	local tmp = table.shallow_copy(pages)
	table.sort(pages, ord_func(order))
	if table.concat(pages) ~= table.concat(tmp) then
		ans = ans + pages[math.floor(#pages / 2) + 1]
	end
end

print(ans)
