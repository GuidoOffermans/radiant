use mlua::{UserData, UserDataMethods};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
pub struct LuaInput {
    pub keys_pressed: Arc<Mutex<HashMap<String, bool>>>,
    pub mouse_pos: Arc<Mutex<(f32, f32)>>,
    pub mouse_pressed: Arc<Mutex<HashMap<String, bool>>>,
}

impl LuaInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_key(&self, key: &str, pressed: bool) {
        self.keys_pressed
            .lock()
            .unwrap()
            .insert(key.to_string(), pressed);
    }

    pub fn set_mouse_pos(&self, x: f32, y: f32) {
        *self.mouse_pos.lock().unwrap() = (x, y);
    }

    pub fn set_mouse_button(&self, button: &str, pressed: bool) {
        self.mouse_pressed
            .lock()
            .unwrap()
            .insert(button.to_string(), pressed);
    }
}

impl UserData for LuaInput {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("isDown", |_, this, key: String| {
            Ok(this
                .keys_pressed
                .lock()
                .unwrap()
                .get(&key)
                .copied()
                .unwrap_or(false))
        });

        methods.add_method("getMousePosition", |_, this, ()| {
            let pos = *this.mouse_pos.lock().unwrap();
            Ok((pos.0, pos.1))
        });

        methods.add_method("isMouseDown", |_, this, button: String| {
            Ok(this
                .mouse_pressed
                .lock()
                .unwrap()
                .get(&button)
                .copied()
                .unwrap_or(false))
        });
    }
}
