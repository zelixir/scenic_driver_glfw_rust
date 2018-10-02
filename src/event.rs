use comms::*;
use types::*;
use glfw::WindowEvent;
pub fn handle_window_event(window_data: &mut WindowData, event: WindowEvent){

}

pub fn reshape_framebuffer(window: &mut WindowData, width: i32, height: i32) {
    window.frame_size = (width, height);
    window.redraw = true;
    
}
pub fn reshape_window(window: &mut WindowData, width: i32, height: i32) {
    window.window_size = (width, height);
    send_reshape(width, height, width, height);
    window.redraw = true;
}
