use crate::comms::*;
use crate::defines::*;
use glfw::WindowEvent;
use crate::types::*;
pub fn handle_window_event(window_data: &mut WindowData, event: WindowEvent) {
    match event {
        WindowEvent::FramebufferSize(w, h) => reshape_framebuffer(window_data, w, h),
        WindowEvent::Size(w, h) => reshape_window(window_data, w, h),
        WindowEvent::Key(key, scancode, action, mods) => {
            key_callback(
                window_data,
                key as i32,
                scancode,
                action as i32,
                mods.bits(),
            );
        }
        WindowEvent::CharModifiers(codepoint, mods) => {
            charmods_callback(window_data, codepoint as u32, mods.bits());
        }
        WindowEvent::CursorPos(xpos, ypos) => cursor_pos_callback(window_data, xpos, ypos),
        WindowEvent::MouseButton(button, action, mods) => {
            mouse_button_callback(window_data, button as i32, action as i32, mods.bits())
        }
        WindowEvent::Scroll(xoffset, yoffset) => {
            scroll_callback(window_data, xoffset, yoffset);
        }
        WindowEvent::CursorEnter(entered) => cursor_enter_callback(window_data, entered as i32),
        WindowEvent::Close => window_close_callback(window_data),
        _ => (),
    };
}
pub fn key_callback(window_data: &mut WindowData, key: i32, scancode: i32, action: i32, mods: i32) {
    if window_data.input_flags & MSG_KEY_MASK != 0 {
        send_key(key, scancode, action, mods);
    }
}
pub fn charmods_callback(window_data: &mut WindowData, codepoint: u32, mods: i32) {
    if window_data.input_flags & MSG_CHAR_MASK != 0 {
        send_codepoint(codepoint, mods);
    }
}
pub fn cursor_pos_callback(window_data: &mut WindowData, xpos: f64, ypos: f64) {
    let pos = (xpos as f32, ypos as f32);
    if window_data.last_cursor_pos != pos
    //&& window_data.input_flags & MSG_MOUSE_MOVE_MASK != 0
    {
        send_cursor_pos(pos.0, pos.1);
        window_data.last_cursor_pos = pos;
    }
}
pub fn mouse_button_callback(window_data: &mut WindowData, button: i32, action: i32, mods: i32) {
    if window_data.input_flags & MSG_MOUSE_BUTTON_MASK != 0 {
        let (x, y) = window_data.window.get_cursor_pos();
        send_mouse_button(button, action, mods, x as f32, y as f32);
    }
}
pub fn scroll_callback(window_data: &mut WindowData, xoffset: f64, yoffset: f64) {
    if window_data.input_flags & MSG_MOUSE_SCROLL_MASK != 0 {
        let (x, y) = window_data.window.get_cursor_pos();
        send_scroll(xoffset as f32, yoffset as f32, x as f32, y as f32);
    }
}
pub fn cursor_enter_callback(window_data: &mut WindowData, entered: i32) {
    if window_data.input_flags & MSG_MOUSE_ENTER_MASK != 0 {
        let (x, y) = window_data.window.get_cursor_pos();
        send_cursor_enter(entered, x as f32, y as f32);
    }
}
pub fn window_close_callback(window_data: &mut WindowData) {
    send_close();
    window_data.window.set_should_close(false);
}
pub fn reshape_framebuffer(window_data: &mut WindowData, width: i32, height: i32) {
    window_data.frame_size = (width, height);
    window_data.redraw = true;
}
pub fn reshape_window(window_data: &mut WindowData, width: i32, height: i32) {
    window_data.window_size = (width, height);
    send_reshape(width, height, width, height);
    window_data.redraw = true;
}
