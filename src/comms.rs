use byteorder::{BigEndian, NativeEndian, ReadBytesExt, WriteBytesExt};
use defines::*;
use gl::*;
use glfw::Glfw;
use std::io::{stdin, stdout, Cursor, Read, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use types::*;
use util::*;
type IOResult = ::std::io::Result<()>;

pub fn send_reshape(window_width: i32, window_height: i32, frame_width: i32, frame_height: i32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_RESHAPE)?;
        w.write_i32::<NativeEndian>(window_width)?;
        w.write_i32::<NativeEndian>(window_height)?;
        w.write_i32::<NativeEndian>(frame_width)?;
        w.write_i32::<NativeEndian>(frame_height)?;
        Ok(())
    });
}
pub fn send_ready(root_id: i32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_READY)?;
        w.write_i32::<NativeEndian>(root_id)?;
        Ok(())
    });
}

fn send_string_cmd(cmd: u32, string: String) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(cmd)?;
        w.write_all(string.as_bytes())?;
        Ok(())
    });
}

pub fn send_puts(string: String) {
    send_string_cmd(MSG_OUT_PUTS, string);
}
pub fn send_write(msg: String) {
    send_string_cmd(MSG_OUT_WRITE, msg);
}
pub fn send_inspect(data: Vec<u8>, length: i32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_INSPECT)?;
        w.write_all(data.as_slice())?;
        Ok(())
    });
}
pub fn send_cache_miss(key: String) {
    send_string_cmd(MSG_OUT_CACHE_MISS, key);
}
pub fn send_font_miss(key: String) {
    send_string_cmd(MSG_OUT_FONT_MISS, key);
}
pub fn send_key(key: i32, scancode: i32, action: i32, mods: i32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_KEY)?;
        w.write_i32::<NativeEndian>(key)?;
        w.write_i32::<NativeEndian>(scancode)?;
        w.write_i32::<NativeEndian>(action)?;
        w.write_i32::<NativeEndian>(mods)?;
        Ok(())
    });
}
pub fn send_codepoint(codepoint: u32, mods: i32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_CODEPOINT)?;
        w.write_u32::<NativeEndian>(codepoint)?;
        w.write_i32::<NativeEndian>(mods)?;
        Ok(())
    });
}
pub fn send_cursor_pos(xpos: f32, ypos: f32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_CURSOR_POS)?;
        w.write_f32::<NativeEndian>(xpos)?;
        w.write_f32::<NativeEndian>(ypos)?;
        Ok(())
    });
}
pub fn send_mouse_button(button: i32, action: i32, mods: i32, xpos: f32, ypos: f32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_MOUSE_BUTTON)?;
        w.write_i32::<NativeEndian>(button)?;
        w.write_i32::<NativeEndian>(action)?;
        w.write_i32::<NativeEndian>(mods)?;
        w.write_f32::<NativeEndian>(xpos)?;
        w.write_f32::<NativeEndian>(ypos)?;
        Ok(())
    });
}
pub fn send_scroll(xoffset: f32, yoffset: f32, xpos: f32, ypos: f32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_MOUSE_SCROLL)?;
        w.write_f32::<NativeEndian>(xoffset)?;
        w.write_f32::<NativeEndian>(yoffset)?;
        w.write_f32::<NativeEndian>(xpos)?;
        w.write_f32::<NativeEndian>(ypos)?;
        Ok(())
    });
}
pub fn send_cursor_enter(entered: i32, xpos: f32, ypos: f32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_CURSOR_ENTER)?;
        w.write_i32::<NativeEndian>(entered)?;
        w.write_f32::<NativeEndian>(xpos)?;
        w.write_f32::<NativeEndian>(ypos)?;
        Ok(())
    });
}
pub fn send_close() {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_CLOSE)?;
        Ok(())
    });
}
pub fn send_draw_ready(id: u32) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_DRAW_READY)?;
        w.write_u32::<NativeEndian>(id)?;
        Ok(())
    });
}

pub fn write_cmd<F>(fun: F)
where
    F: FnOnce(&mut Write) -> IOResult,
{
    let mut buf: Vec<u8> = Vec::with_capacity(64);
    fun(&mut buf).unwrap();
    stdout().write_u32::<BigEndian>(buf.len() as u32).unwrap();
    stdout().write_all(buf.as_slice()).unwrap();
}

const STD_TIMEOUT: u64 = 32000;
pub fn handle_stdio_in<'ctx: 'tx, 'tx>(
    window_data: &mut WindowData,
    glfw: &mut Glfw,
    ctx: &mut Context<'ctx, 'tx>,
    receiver: &mut Receiver<Message>,
) -> bool {
    let start = Instant::now();
    let timeout = Duration::from_micros(STD_TIMEOUT);
    let mut redraw = false;
    // while start.elapsed() < timeout {
    if let Ok(msg) = receiver.try_recv() {
        redraw = dispatch_message(window_data, msg, glfw, ctx) || redraw;
    } else {

    }
    // }
    redraw
}

fn dispatch_message<'ctx: 'tx, 'tx>(
    window_data: &mut WindowData,
    msg: Message,
    glfw: &mut Glfw,
    ctx: &mut Context<'ctx, 'tx>,
) -> bool {
    let mut read = Cursor::new(msg);
    let mut render = false;
    let msg_id = read.read_u32::<BigEndian>().unwrap();
    check_gl_error("starting error: ".to_string());

    match msg_id {
        CMD_QUIT => {
            receive_quit(glfw, window_data);
            return false;
        }
        CMD_RENDER_GRAPH => {
            receive_render(glfw, &mut read, window_data);
            render = true;
        }
        CMD_CLEAR_GRAPH => {
            receive_clear(glfw, &mut read, window_data);
            render = true;
        }
        CMD_SET_ROOT => {
            receive_set_root(glfw, &mut read, window_data);
            render = true;
        }

        CMD_CLEAR_COLOR => {
            receive_clear_color(glfw, &mut read);
            render = true;
        }
        /*
     CMD_UPDATE_GRAPH=> {   receive_update_graph( &mut read, window );    render = false; }
     CMD_CACHE_LOAD=> {     receive_cache_load( &mut read, window );      render = false; }
     CMD_CACHE_RELEASE=> {  receive_cache_release( &mut read, window );   render = false; }
*/
        CMD_INPUT => receive_input(glfw, &mut read, window_data),

        CMD_QUERY_STATS => receive_query_stats(glfw, window_data),

        CMD_RESHAPE => receive_reshape(glfw, &mut read, window_data),

        CMD_POSITION => receive_position(glfw, &mut read, window_data),

        //  CMD_FOCUS=> {          glfwFocusWindow( window );                      }
        CMD_ICONIFY => window_data.window.iconify(),

        //  CMD_MAXIMIZE=> {       glfwMaximizeWindow( window );                   }
        CMD_RESTORE => window_data.window.restore(),

        CMD_SHOW => window_data.window.show(),

        CMD_HIDE => window_data.window.hide(),

        //  CMD_NEW_DL_ID=> {      receive_new_dl_id();                            }
    //  CMD_FREE_DL_ID=> {     receive_free_dl_id( &mut read );              render = true; }
    //  CMD_NEW_TX_ID=> {      receive_new_tx_id();                            }


    // font handling
        CMD_LOAD_FONT_FILE => {
            receive_load_font_file(glfw, &mut read, window_data, ctx);
            render = true;
        }
        CMD_LOAD_FONT_BLOB => {
            receive_load_font_blob(glfw, &mut read, window_data, ctx);
            render = true;
        }
        //  CMD_FREE_FONT=> {      receive_free_font( &mut read, window );       }

    // the next two are in texture.c
        CMD_PUT_TX_BLOB => {
            receive_put_tx_blob(glfw, &mut read, window_data, ctx);
            render = true;
        }
        //  CMD_PUT_TX_RAW=> {     receive_put_tx_raw( &mut read, window );      render = true; }
        CMD_FREE_TX_ID => receive_free_tx_id(glfw, &mut read, window_data, ctx),

        // the next set are in text.c
        //  CMD_PUT_FONT=> {       receive_put_font_atlas( &mut read, window );  render = true; }
        //  CMD_FREE_FONT_ID=> {   receive_free_font_atlas( &mut read, window ); render = true; }
        CMD_CRASH => receive_crash(),

        _ => send_puts(format!("Unknown message: {:#X}", msg_id)),
    }
    let mut remain = String::new();
    if read.read_to_string(&mut remain).unwrap() > 0 {
        check_gl_error(remain);
    }
    render
}

fn read_msg_len() -> u32 {
    stdin().read_u32::<BigEndian>().unwrap()
}

pub fn start_read_stdin_thread(sender: Sender<Message>) {
    ::std::thread::spawn(move || loop {
        let len = read_msg_len() as usize;
        let mut buf = vec![0u8; len];
        stdin().read(buf.as_mut_slice()).unwrap();
        sender.send(buf).unwrap();
    });
}

fn receive_quit(glfw: &mut Glfw, window_data: &mut WindowData) {
    window_data.keep_going = false;
    glfw.post_empty_event();
}
fn receive_render(glfw: &mut Glfw, read: &mut impl Read, window_data: &mut WindowData) {
    let id = read.read_u32::<NativeEndian>().unwrap();
    let mut script = vec![];
    read.read_to_end(&mut script).unwrap();
    window_data.put_script(id, script);
    send_draw_ready(id);
    glfw.post_empty_event();
}
fn receive_clear(_glfw: &mut Glfw, read: &mut impl Read, window_data: &mut WindowData) {
    let id = read.read_u32::<NativeEndian>().unwrap();
    window_data.delete_script(id);
}
fn receive_set_root(glfw: &mut Glfw, read: &mut impl Read, window_data: &mut WindowData) {
    let id = read.read_i32::<NativeEndian>().unwrap();
    window_data.root_script = id;
    glfw.post_empty_event();
}
fn receive_clear_color(_glfw: &mut Glfw, read: &mut impl Read) {
    let mut color = [0u32; 4];
    read.read_u32_into::<NativeEndian>(&mut color).unwrap();
    unsafe {
        ClearColor(
            color[0] as f32 / 255.0f32,
            color[1] as f32 / 255.0f32,
            color[2] as f32 / 255.0f32,
            color[3] as f32 / 255.0f32,
        );
    }
}
// fn receive_update_graph(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
// fn receive_cache_load(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
// fn receive_cache_release(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
fn receive_input(_glfw: &mut Glfw, read: &mut impl Read, window_data: &mut WindowData) {
    let flag = read.read_u32::<NativeEndian>().unwrap();
    window_data.input_flags = flag;
}
fn receive_query_stats(_glfw: &mut Glfw, window_data: &mut WindowData) {
    write_cmd(|w| {
        w.write_u32::<NativeEndian>(MSG_OUT_STATS)?;
        w.write_u32::<NativeEndian>(window_data.input_flags)?;
        let (x, y) = window_data.window.get_pos();
        w.write_i32::<NativeEndian>(x)?;
        w.write_i32::<NativeEndian>(y)?;
        let (width, height) = window_data.window.get_size();
        w.write_i32::<NativeEndian>(width)?;
        w.write_i32::<NativeEndian>(height)?;

        w.write_u32::<NativeEndian>(window_data.window.is_focused() as u32)?;
        w.write_u32::<NativeEndian>(window_data.window.is_resizable() as u32)?;
        w.write_u32::<NativeEndian>(window_data.window.is_iconified() as u32)?;
        w.write_u32::<NativeEndian>(window_data.window.is_maximized() as u32)?;
        w.write_u32::<NativeEndian>(window_data.window.is_visible() as u32)?;
        Ok(())
    });
}
fn receive_reshape(_glfw: &mut Glfw, read: &mut impl Read, window_data: &mut WindowData) {
    let w = read.read_i32::<NativeEndian>().unwrap();
    let h = read.read_i32::<NativeEndian>().unwrap();
    window_data.window.set_size(w, h);
}
fn receive_position(_glfw: &mut Glfw, read: &mut impl Read, window_data: &mut WindowData) {
    let x = read.read_i32::<NativeEndian>().unwrap();
    let y = read.read_i32::<NativeEndian>().unwrap();
    window_data.window.set_pos(x, y);
}
// fn receive_new_dl_id(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
// fn receive_free_dl_id(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
// fn receive_new_tx_id(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
fn receive_load_font_file(
    _glfw: &mut Glfw,
    read: &mut impl Read,
    _window_data: &mut WindowData,
    ctx: &mut Context,
) {
    let name_len = read.read_u32::<NativeEndian>().unwrap();
    let path_len = read.read_u32::<NativeEndian>().unwrap();
    let mut name = String::with_capacity(name_len as usize);
    let mut path = String::with_capacity(path_len as usize);
    unsafe {
        read.read_exact(name.as_mut_vec().as_mut_slice()).unwrap();
        read.read_exact(path.as_mut_vec().as_mut_slice()).unwrap();
    }
    if ::nanovg::Font::find(ctx.ctx, &name).is_err() {
        ::nanovg::Font::from_file(ctx.ctx, &name, &path).unwrap();
    }
}
fn receive_load_font_blob(
    _glfw: &mut Glfw,
    read: &mut impl Read,
    _window_data: &mut WindowData,
    ctx: &mut Context,
) {
    let name_len = read.read_u32::<NativeEndian>().unwrap();
    let data_len = read.read_u32::<NativeEndian>().unwrap();
    let mut name = String::with_capacity(name_len as usize);
    let mut data = Vec::with_capacity(data_len as usize);

    unsafe {
        read.read_exact(name.as_mut_vec().as_mut_slice()).unwrap();
    }
    read.read_exact(data.as_mut_slice()).unwrap();
    if ::nanovg::Font::find(ctx.ctx, &name).is_err() {
        ::nanovg::Font::from_memory(ctx.ctx, &name, &data).unwrap();
    }
}
// fn receive_free_font(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
// fn receive_put_tx_raw(_glfw:&mut  Glfw,window_data: &mut WindowData) {}

fn receive_put_tx_blob<'ctx: 'tx, 'tx>(
    _glfw: &mut Glfw,
    read: &mut impl Read,
    _window_data: &mut WindowData,
    ctx: &mut Context<'ctx, 'tx>,
) {
    let name_len = read.read_u32::<NativeEndian>().unwrap();
    let data_len = read.read_u32::<NativeEndian>().unwrap();
    let mut name = String::with_capacity(name_len as usize);
    let mut data = Vec::with_capacity(data_len as usize);

    unsafe {
        read.read_exact(name.as_mut_vec().as_mut_slice()).unwrap();
    }
    read.read_exact(data.as_mut_slice()).unwrap();
    ctx.put_tx(name, data);
}
fn receive_free_tx_id(
    _glfw: &mut Glfw,
    read: &mut impl Read,
    _window_data: &mut WindowData,
    ctx: &mut Context,
) {
    let name_len = read.read_u32::<NativeEndian>().unwrap();
    let mut name = String::with_capacity(name_len as usize);

    unsafe {
        read.read_exact(name.as_mut_vec().as_mut_slice()).unwrap();
    }

    ctx.free_tx(name);
}
// fn receive_put_font_atlas(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
// fn receive_free_font_atlas(_glfw:&mut  Glfw,window_data: &mut WindowData) {}
fn receive_crash() {
    send_puts("receive_crash - exit".to_string());
    ::std::process::exit(1);
}
