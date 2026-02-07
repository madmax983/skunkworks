-- Moonshot Payload
local balls = {}
-- Seed random
math.randomseed(os.time())

for i = 1, 20 do
    table.insert(balls, {
        x = math.random() * screen_width(),
        y = math.random() * screen_height(),
        dx = (math.random() - 0.5) * 5,
        dy = (math.random() - 0.5) * 5,
        r = 10 + math.random() * 20,
        c = math.random(0xFFFFFF)
    })
end

function update()
    local w = screen_width()
    local h = screen_height()

    for _, b in ipairs(balls) do
        b.x = b.x + b.dx
        b.y = b.y + b.dy

        if b.x < 0 or b.x > w then b.dx = -b.dx end
        if b.y < 0 or b.y > h then b.dy = -b.dy end
    end
end

function draw()
    clear_background(0x101010)

    local t = time()

    -- Draw grid
    for x = 0, screen_width(), 50 do
        draw_rectangle(x, 0, 1, screen_height(), 0x222222)
    end
    for y = 0, screen_height(), 50 do
        draw_rectangle(0, y, screen_width(), 1, 0x222222)
    end

    for _, b in ipairs(balls) do
        draw_circle(b.x, b.y, b.r, b.c)
    end

    local text = "STEGANOGRAPHY ACTIVE"
    local offset = math.sin(t * 3) * 10
    draw_text(text, 50 + offset, 100, 50, 0x00FF00)

    draw_text("Running inside an image...", 50, 150, 30, 0x00AAAA)
    draw_text("Press ESC to extract source", 50, screen_height() - 50, 20, 0x888888)
end
