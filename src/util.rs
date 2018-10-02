use comms::*;
use gl::*;
pub fn check_gl_error(msg: String) {
    loop {
        let err = unsafe { GetError() };
        match err {
            NO_ERROR => return,

            INVALID_ENUM => send_puts(format!("{} INVALID_ENUM", msg)),
            INVALID_VALUE => send_puts(format!("{} INVALID_VALUE", msg)),
            INVALID_OPERATION => send_puts(format!("{} INVALID_OPERATION", msg)),
            OUT_OF_MEMORY => send_puts(format!("{} OUT_OF_MEMORY", msg)),
            STACK_UNDERFLOW => send_puts(format!("{} STACK_UNDERFLOW", msg)),
            STACK_OVERFLOW => send_puts(format!("{} STACK_OVERFLOW", msg)),
            INVALID_FRAMEBUFFER_OPERATION => {
                send_puts(format!("{} INVALID_FRAMEBUFFER_OPERATION", msg))
            }
            _ => send_puts(format!("{} GL_OTHER: {}", msg, err)),
        }
    }
}
