local time = 0

function radiant.load()
	print("Lua game loaded!")
	radiant.graphics.clear(0.2, 0.3, 0.8, 1.0)
end

function radiant.update(dt)
	time = time + dt

	-- Cycle through colors
	local r = (math.sin(time) + 1) / 2
	local g = (math.sin(time * 1.3) + 1) / 2
	local b = (math.sin(time * 0.7) + 1) / 2

	radiant.graphics.clear(r * 0.5, g * 0.5, b * 0.8, 1.0)
end

function radiant.draw()
	-- Drawing logic would go here
end

function radiant.keypressed(key)
	print("Key pressed: " .. key)

	if key == "r" then
		radiant.graphics.clear(1.0, 0.2, 0.2, 1.0)
	elseif key == "g" then
		radiant.graphics.clear(0.2, 1.0, 0.2, 1.0)
	elseif key == "b" then
		radiant.graphics.clear(0.2, 0.2, 1.0, 1.0)
	end
end

function radiant.keyreleased(key)
	print("Key released: " .. key)
end

function radiant.mousepressed(x, y, button)
	print("Mouse clicked at (" .. x .. ", " .. y .. ") with button " .. button)

	-- Set color based on mouse position (assuming 800x600 window)
	local r = x / 800
	local g = y / 600
	radiant.graphics.clear(r, g, 0.5, 1.0)
end
