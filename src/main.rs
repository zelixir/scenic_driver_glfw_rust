extern crate byteorder;
extern crate gl;
extern crate glfw;
extern crate nanovg;

#[macro_use]
mod util;
mod defines;
mod event;
mod script;
mod types;
mod comms;

use comms::*;
use event::*;
use glfw::{Context, Glfw, WindowHint, WindowMode};
use script::*;
use types::*;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 6 {
        print!(
            "\r\nscenic_driver_glfw should be launched via the Scenic.Driver.Glfw library.\r\n\r\n"
        );
        return;
    }

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let width = args[1].parse::<i32>().unwrap();
    let height = args[2].parse::<i32>().unwrap();
    // let block_size = args[5].parse::<i32>().unwrap();
    set_window_hints(&mut glfw, &args[4]);
    let (window, events) =
        glfw.create_window(width as u32, height as u32, &args[3], WindowMode::Windowed)
            .expect("cannot create window");

    glfw.make_context_current(Some(&window));
    let ctx = ::nanovg::ContextBuilder::new()
        .antialias()
        .stencil_strokes()
        .debug()
        .build()
        .expect("Could not init nanovg!!!");
    let mut context = types::Context {
        ctx: &ctx,
        textures: Default::default(),
    };

    let mut window_data = setup_window_data(window, width, height);
    send_ready(0);

    let (std_channel_send, mut std_channel_recv) = ::std::sync::mpsc::channel::<Message>();
    start_read_stdin_thread(std_channel_send);

    while window_data.keep_going && !is_caller_down() {
        if window_data.redraw
            || handle_stdio_in(
                &mut window_data,
                &mut glfw,
                &mut context,
                &mut std_channel_recv,
            ) {
            window_data.redraw = false;
            unsafe {
                ::gl::Clear(::gl::COLOR_BUFFER_BIT);
            }
            let root_script = window_data.root_script;
            ctx.frame(
                window_data.get_window_size_float(),
                window_data.get_ratio().0,
                |mut frame| {
                    if root_script > 0 {
                        run_scripts(
                            &mut window_data,
                            root_script as u32,
                            &mut context,
                            &mut frame,
                        );
                    }
                },
            );
            window_data.window.swap_buffers();
        }
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window_data, event);
        }
    }
    cleanup_window(window_data);
    ::glfw::terminate();
}

fn set_window_hints(glfw: &mut Glfw, resizable: &String) {
    if resizable != "true" {
        glfw.window_hint(WindowHint::Resizable(false));
    }
    glfw.window_hint(WindowHint::Focused(true));
    glfw.window_hint(WindowHint::ContextVersionMajor(2));
    glfw.window_hint(WindowHint::ContextVersionMinor(0));
}

fn cleanup_window(window_data: WindowData) {}

fn is_caller_down() -> bool {
    false
}
