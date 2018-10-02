use event::*;
use glfw::{Glfw, Window};
use nanovg::Image;
use std::collections::HashMap;
pub type Script = Vec<u8>;
pub type NanoContext = ::nanovg::Context;
pub struct Context<'ctx: 'tx, 'tx> {
    pub textures: HashMap<String, Image<'tx>>,
    pub ctx: &'ctx NanoContext,
}

impl<'ctx: 'tx, 'tx> Context<'ctx, 'tx> {
    pub fn put_tx(&mut self, key: String, data: Vec<u8>) {
        let img = ::nanovg::Image::new(&self.ctx)
            .build_from_memory(&data)
            .unwrap();
        self.textures.insert(key, img);
    }
    pub fn free_tx(&mut self, key: String){
        self.textures.remove(&key);
    }
}

pub type Message = Vec<u8>;

pub struct WindowData {
    pub window: Window,
    pub window_size: (i32, i32),
    pub frame_size: (i32, i32),
    pub keep_going: bool,
    pub redraw: bool,
    pub input_flags: u32,
    pub last_cursor_pos: (f32, f32),
    pub scripts: HashMap<u32, Script>,
    pub root_script: i32,
}

impl WindowData {
    pub fn put_script(&mut self, id: u32, script: Script) {
        self.scripts.insert(id, script);
    }
    pub fn get_script(&self, id: u32) -> Option<&Script> {
        self.scripts.get(&id)
    }
    pub fn delete_script(&mut self, id: u32) {
        self.scripts.remove(&id);
    }
    pub fn get_ratio(&self) -> (f32, f32) {
        (
            self.frame_size.0 as f32 / self.window_size.0 as f32,
            self.frame_size.1 as f32 / self.window_size.1 as f32,
        )
    }
    pub fn get_window_size_float(&self) -> (f32, f32) {
        let (width, height) = self.window_size;
        (width as f32, height as f32)
    }
}

pub fn setup_window_data(window: Window, width: i32, height: i32) -> WindowData {
    let mut re = WindowData {
        window: window,
        keep_going: true,
        redraw: false,
        input_flags: 0xFFFF,
        last_cursor_pos: (-1f32, -1f32),
        scripts: Default::default(),
        root_script: -1,
        window_size: (width, height),
        frame_size: (0, 0),
    };

    let size = re.window.get_framebuffer_size();
    reshape_framebuffer(&mut re, size.0, size.1);

    let size = re.window.get_size();
    reshape_window(&mut re, size.0, size.1);

    re.window.set_framebuffer_size_polling(true);
    re.window.set_size_polling(true);
    re.window.set_key_polling(true);
    re.window.set_char_mods_polling(true);
    re.window.set_cursor_pos_polling(true);
    re.window.set_cursor_enter_polling(true);
    re.window.set_mouse_button_polling(true);
    re.window.set_scroll_polling(true);
    re.window.set_close_polling(true);
    unsafe {
        ::gl::ClearColor(0f32, 0f32, 0f32, 1f32);
    }
    re
}
