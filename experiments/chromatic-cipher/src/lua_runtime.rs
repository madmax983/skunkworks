use macroquad::prelude::*;
use mlua::{Function, Lua, Result};

pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();

        {
            let globals = lua.globals();

            globals.set("screen_width", lua.create_function(|_, ()| Ok(screen_width()))?)?;
            globals.set("screen_height", lua.create_function(|_, ()| Ok(screen_height()))?)?;
            globals.set("time", lua.create_function(|_, ()| Ok(get_time()))?)?;

            globals.set(
                "clear_background",
                lua.create_function(|_, color_hex: u32| {
                    let color = hex_to_color(color_hex);
                    clear_background(color);
                    Ok(())
                })?,
            )?;

            globals.set(
                "draw_text",
                lua.create_function(|_, (text, x, y, size, color_hex): (String, f32, f32, f32, u32)| {
                    let color = hex_to_color(color_hex);
                    draw_text(&text, x, y, size, color);
                    Ok(())
                })?,
            )?;

            globals.set(
                "draw_circle",
                lua.create_function(|_, (x, y, r, color_hex): (f32, f32, f32, u32)| {
                    let color = hex_to_color(color_hex);
                    draw_circle(x, y, r, color);
                    Ok(())
                })?,
            )?;

            globals.set(
                "draw_rectangle",
                lua.create_function(|_, (x, y, w, h, color_hex): (f32, f32, f32, f32, u32)| {
                    let color = hex_to_color(color_hex);
                    draw_rectangle(x, y, w, h, color);
                    Ok(())
                })?,
            )?;
        }

        Ok(Self { lua })
    }

    pub fn load_script(&self, script: &str) -> Result<()> {
        self.lua.load(script).exec()
    }

    pub fn call_update(&self) -> Result<()> {
        if let Ok(update) = self.lua.globals().get::<_, Function>("update") {
            update.call::<_, ()>(())?;
        }
        Ok(())
    }

    pub fn call_draw(&self) -> Result<()> {
        if let Ok(draw) = self.lua.globals().get::<_, Function>("draw") {
            draw.call::<_, ()>(())?;
        }
        Ok(())
    }
}

fn hex_to_color(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    Color::new(r, g, b, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_init() {
        let runtime = LuaRuntime::new().unwrap();
        // Verify we can execute basic Lua
        assert!(runtime.load_script("local x = 1 + 1").is_ok());
    }
}
