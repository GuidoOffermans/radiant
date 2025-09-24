use super::{LuaGraphics, LuaInput};
use mlua::{Function, Lua, Result as LuaResult, Table};

pub struct LuaEngine {
    lua: Lua,
    graphics: LuaGraphics,
    input: LuaInput,
}

impl LuaEngine {
    pub fn new() -> LuaResult<Self> {
        let lua = Lua::new();
        let graphics = LuaGraphics::new();
        let input = LuaInput::new();

        let radiant_table = lua.create_table()?;

        // Add graphics module
        let graphics_table = lua.create_table()?;
        graphics_table.set(
            "clear",
            lua.create_function({
                let graphics = graphics.clone();
                move |_, (r, g, b, a): (Option<f64>, Option<f64>, Option<f64>, Option<f64>)| {
                    graphics.set_clear_color(wgpu::Color {
                        r: r.unwrap_or(0.0),
                        g: g.unwrap_or(0.0),
                        b: b.unwrap_or(0.0),
                        a: a.unwrap_or(1.0),
                    });
                    Ok(())
                }
            })?,
        )?;
        radiant_table.set("graphics", graphics_table)?;

        // Add keyboard module
        let keyboard_table = lua.create_table()?;
        keyboard_table.set(
            "isDown",
            lua.create_function({
                let input = input.clone();
                move |_, key: String| {
                    Ok(input
                        .keys_pressed
                        .lock()
                        .unwrap()
                        .get(&key)
                        .copied()
                        .unwrap_or(false))
                }
            })?,
        )?;

        radiant_table.set("keyboard", keyboard_table)?;

        // Add mouse module
        let mouse_table = lua.create_table()?;
        mouse_table.set(
            "getPosition",
            lua.create_function({
                let input = input.clone();
                move |_, ()| {
                    let pos = *input.mouse_pos.lock().unwrap();
                    Ok((pos.0, pos.1))
                }
            })?,
        )?;

        mouse_table.set(
            "isDown",
            lua.create_function({
                let input = input.clone();
                move |_, button: String| {
                    Ok(input
                        .mouse_pressed
                        .lock()
                        .unwrap()
                        .get(&button)
                        .copied()
                        .unwrap_or(false))
                }
            })?,
        )?;

        radiant_table.set("mouse", mouse_table)?;

        // Set radiant as global
        lua.globals().set("radiant", radiant_table)?;

        Ok(Self {
            lua,
            graphics,
            input,
        })
    }

    pub fn load_script(&self, script: &str) -> LuaResult<()> {
        self.lua.load(script).exec()
    }

    pub fn load_file(&self, path: &str) -> LuaResult<()> {
        let script = std::fs::read_to_string(path)
            .map_err(|e| mlua::Error::RuntimeError(format!("Failed to read file: {}", e)))?;
        self.load_script(&script)
    }

    pub fn call_radiant_load(&self) -> LuaResult<()> {
        if let Ok(radiant) = self.lua.globals().get::<Table>("radiant") {
            if let Ok(load_fn) = radiant.get::<Function>("load") {
                let _: () = load_fn.call(())?;
            }
        }
        Ok(())
    }

    pub fn call_radiant_update(&self, dt: f32) -> LuaResult<()> {
        if let Ok(radiant) = self.lua.globals().get::<Table>("radiant") {
            if let Ok(update_fn) = radiant.get::<Function>("update") {
                let _: () = update_fn.call(dt)?;
            }
        }
        Ok(())
    }

    pub fn call_radiant_draw(&self) -> LuaResult<()> {
        if let Ok(radiant) = self.lua.globals().get::<Table>("radiant") {
            if let Ok(draw_fn) = radiant.get::<Function>("draw") {
                let _: () = draw_fn.call(())?;
            }
        }
        Ok(())
    }

    pub fn call_key_pressed(&self, key: &str) -> LuaResult<()> {
        if let Ok(radiant) = self.lua.globals().get::<Table>("radiant") {
            if let Ok(key_pressed_fn) = radiant.get::<Function>("keypressed") {
                let _: () = key_pressed_fn.call(key)?;
            }
        }
        Ok(())
    }

    pub fn call_key_released(&self, key: &str) -> LuaResult<()> {
        if let Ok(radiant) = self.lua.globals().get::<Table>("radiant") {
            if let Ok(key_released_fn) = radiant.get::<Function>("keyreleased") {
                let _: () = key_released_fn.call(key)?;
            }
        }
        Ok(())
    }

    pub fn call_mouse_pressed(&self, x: f32, y: f32, button: &str) -> LuaResult<()> {
        if let Ok(radiant) = self.lua.globals().get::<Table>("radiant") {
            if let Ok(mouse_pressed_fn) = radiant.get::<Function>("mousepressed") {
                let _: () = mouse_pressed_fn.call((x, y, button))?;
            }
        }
        Ok(())
    }

    pub fn graphics(&self) -> &LuaGraphics {
        &self.graphics
    }

    pub fn input(&self) -> &LuaInput {
        &self.input
    }
}
