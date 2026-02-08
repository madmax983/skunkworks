-- Spectral Loading Demo Script
-- This script is embedded in the audio file and executed by the visualizer.

print("Hello from Spectral Loading! Script loaded successfully.")

function draw()
    -- Global variables: fft (table of magnitudes), time (seconds), screen_w, screen_h
    -- Functions: clear_background(color), draw_line(x1, y1, x2, y2, thickness, color), draw_circle(x, y, r, color), draw_text(text, x, y, size, color)
    -- Colors: BLACK, WHITE, RED, GREEN, BLUE (u32 hex)

    clear_background(BLACK)

    draw_text("SPECTRAL LOADING", 20, 50, 40, WHITE)
    draw_text(string.format("Time: %.2f", time), 20, 90, 20, WHITE)

    local bar_width = screen_w / #fft

    for i = 1, #fft do
        local mag = fft[i]

        -- Use magnitude to drive height
        local height = mag * 50.0 -- Adjust scale factor based on observed magnitudes

        local x = (i - 1) * bar_width
        local y = screen_h

        -- Color gradient based on frequency
        local color = GREEN
        if i > #fft / 2 then color = RED end
        if i < #fft / 4 then color = BLUE end

        draw_line(x, y, x, y - height, bar_width * 0.8, color)

        -- Draw circles for high energy
        if mag > 20.0 then
             draw_circle(x, y - height - 10, mag, WHITE)
        end
    end

    -- Pulsing center circle based on bass (low freq)
    local bass = 0
    for i = 1, 10 do
        if fft[i] then bass = bass + fft[i] end
    end
    draw_circle(screen_w / 2, screen_h / 2, bass * 2.0, 0x555555)
end
