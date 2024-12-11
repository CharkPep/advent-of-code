local VOID, VISITED, EMPTY, OBSITICLE, UP, DOWN, LEFT, RIGHT = 1, 2, 3, 4, 5, 6, 7, 8
local iter = io.lines("./input.txt", "*l")
Game = {
	map = {},
	player = {},
}

function iiter(iter, init, step)
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

function Game:peek()
	if self.player.direction == UP then
		return self.map[self.player.position.i - 1][self.player.position.j]
	elseif self.player.direction == DOWN then
		return self.map[self.player.position.i + 1][self.player.position.j]
	elseif self.player.direction == LEFT then
		return self.map[self.player.position.i][self.player.position.j - 1]
	elseif self.player.direction == RIGHT then
		return self.map[self.player.position.i][self.player.position.j + 1]
	end
end

function Game:advance()
	if self.player.direction == UP then
		self.player.position.i = self.player.position.i - 1
	elseif self.player.direction == DOWN then
		self.player.position.i = self.player.position.i + 1
	elseif self.player.direction == LEFT then
		self.player.position.j = self.player.position.j - 1
	elseif self.player.direction == RIGHT then
		self.player.position.j = self.player.position.j + 1
	end
end

function Game:change_direction()
	if self.player.direction == UP then
		self.player.direction = RIGHT
	elseif self.player.direction == DOWN then
		self.player.direction = LEFT
	elseif self.player.direction == LEFT then
		self.player.direction = UP
	elseif self.player.direction == RIGHT then
		self.player.direction = DOWN
	end
end

function Game:solve()
	local ans = 0
	local player_md = {
		init = {
			position = { i = self.player.position.i, j = self.player.position.j },
			direction = self.player.direction,
		},
	}
	while self.map[self.player.position.i][self.player.position.j] ~= VOID do
		if self.map[self.player.position.i][self.player.position.j] ~= VISITED then
			self.map[self.player.position.i][self.player.position.j] = VISITED
			player_md.last = {
				direction = self.player.direction,
				position = { i = self.player.position.i, j = self.player.position.j },
			}
			ans = ans + 1
		elseif
			player_md.last ~= nil
			and player_md.last.position.i == self.player.position.i
			and player_md.last.position.j == self.player.position.j
			and player_md.last.direction == self.player.direction
		then
			ans = -1
			break
		end

		while self:peek() == OBSITICLE do
			self:change_direction()
		end

		self:advance()
	end

	self.player = player_md.init
	return ans
end

function Game:print_map()
	for i in ipairs(self.map) do
		print(table.concat(self.map[i], ", "))
	end
end

function Game:reset()
	for i in ipairs(self.map) do
		for j in ipairs(self.map[i]) do
			if self.map[i][j] == VISITED then
				self.map[i][j] = EMPTY
			end
		end
	end
end

function find_loops(g)
	local ans = 0
	for i in ipairs(g.map) do
		for j in ipairs(g.map[i]) do
			if g.map[i][j] == EMPTY and (i ~= g.player.position.i or j ~= g.player.position.j) then
				g.map[i][j] = OBSITICLE
				if g:solve() == -1 then
					ans = ans + 1
				end
				g:reset()
				g.map[i][j] = EMPTY
			end
		end
	end

	return ans
end

for i, line in iiter(iter) do
	Game.map[i + 1] = {}
	for j, c in iiter(string.gmatch(line, "%S")) do
		if c == "#" then
			Game.map[i + 1][j + 1] = OBSITICLE
		else
			Game.map[i + 1][j + 1] = EMPTY
		end
		if c == "^" then
			Game.player.position = { i = i + 1, j = j + 1 }
			Game.player.direction = UP
		elseif c == "v" then
			Game.player.position = { i = i + 1, j = j + 1 }
			Game.player.direction = DOWN
		elseif c == ">" then
			Game.player.position = { i = i + 1, j = j + 1 }
			Game.player.direction = RIGHT
		elseif c == "<" then
			Game.player.position = { i = i + 1, j = j + 1 }
			Game.player.direction = LEFT
		end
	end

	Game.map[i + 1][#Game.map[i + 1] + 1] = VOID
	Game.map[i + 1][1] = VOID
end

Game.map[1] = {}
Game.map[#Game.map + 1] = {}
for i = 1, #Game.map[2], 1 do
	Game.map[1][i] = VOID
	Game.map[#Game.map][i] = VOID
end

-- Holy Brute Force
-- lua main.lua  44.87s user 0.01s system 99% cpu 44.989 total
-- On AMD Ryzen 5 5600 4.280 GHz
print(find_loops(Game))
