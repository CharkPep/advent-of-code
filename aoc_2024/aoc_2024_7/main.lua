local iter = io.lines("./input.txt", "*l")
local ans = 0

local function base3_digit(number, i)
	local current = number
	local digit = nil
	for _ = 1, i do
		digit = current % 3
		current = math.floor(current / 3)
		if current == 0 and _ < i then
			return 0
		end
	end

	return digit
end

-- Part 1
-- for line in iter do
-- 	local diter = line:gmatch("(%d+)")
-- 	local goal = tonumber(diter())
-- 	local combinations = {}
-- 	for v in diter do
-- 		combinations[#combinations + 1] = tonumber(v)
-- 	end
--
-- 	for i = 0, 1 << (#combinations - 1), 1 do
-- 		local reached = combinations[1]
-- 		for j = 2, #combinations, 1 do
-- 			if i & (1 << (j - 2)) ~= 0 then
-- 				reached = reached * combinations[j]
-- 			else
-- 				reached = reached + combinations[j]
-- 			end
-- 		end
--
-- 		if goal == reached then
-- 			ans = ans + reached
-- 			break
-- 		end
-- 	end
-- end

-- Part 2
for line in iter do
	local diter = line:gmatch("(%d+)")
	local goal = tonumber(diter())
	local combinations = {}
	for v in diter do
		combinations[#combinations + 1] = tonumber(v)
	end

	for i = 0, math.pow(3, (#combinations - 1)), 1 do
		local reached = combinations[1]
		for j = 2, #combinations, 1 do
			comb = base3_digit(i, j - 1)
			if comb == 0 then
				reached = reached * combinations[j]
			elseif comb == 1 then
				reached = tonumber(tostring(reached) .. tostring(combinations[j]))
			else
				reached = reached + combinations[j]
			end
		end

		if goal == reached then
			ans = ans + reached
			break
		end
	end
end

print(ans)
