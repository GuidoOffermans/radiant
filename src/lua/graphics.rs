use mlua::{UserData, UserDataMethods};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct LuaGraphics {
    clear_color: Arc<Mutex<wgpu::Color>>,
}

impl LuaGraphics {
    pub fn new() -> Self {
        Self {
            clear_color: Arc::new(Mutex::new(wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            })),
        }
    }

    pub fn get_clear_color(&self) -> wgpu::Color {
        *self.clear_color.lock().unwrap()
    }

    pub fn set_clear_color(&self, color: wgpu::Color) {
        *self.clear_color.lock().unwrap() = color;
    }
}

impl UserData for LuaGraphics {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("clear", |_, this, (r, g, b, a): (f64, f64, f64, f64)| {
            *this.clear_color.lock().unwrap() = wgpu::Color { r, g, b, a };
            Ok(())
        });

        methods.add_method("getClearColor", |_, this, ()| {
            let color = *this.clear_color.lock().unwrap();
            Ok((color.r, color.g, color.b, color.a))
        });
    }
}
