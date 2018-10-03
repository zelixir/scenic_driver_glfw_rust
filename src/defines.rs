#![allow(unused)]
pub const MSG_OUT_CLOSE: u32 = 0x00;
pub const MSG_OUT_STATS: u32 = 0x01;
pub const MSG_OUT_PUTS: u32 = 0x02;
pub const MSG_OUT_WRITE: u32 = 0x03;
pub const MSG_OUT_INSPECT: u32 = 0x04;
pub const MSG_OUT_RESHAPE: u32 = 0x05;
pub const MSG_OUT_READY: u32 = 0x06;
pub const MSG_OUT_DRAW_READY: u32 = 0x07;

pub const MSG_OUT_KEY: u32 = 0x0A;
pub const MSG_OUT_CODEPOINT: u32 = 0x0B;
pub const MSG_OUT_CURSOR_POS: u32 = 0x0C;
pub const MSG_OUT_MOUSE_BUTTON: u32 = 0x0D;
pub const MSG_OUT_MOUSE_SCROLL: u32 = 0x0E;
pub const MSG_OUT_CURSOR_ENTER: u32 = 0x0F;
pub const MSG_OUT_DROP_PATHS: u32 = 0x10;
pub const MSG_OUT_CACHE_MISS: u32 = 0x20;

pub const MSG_OUT_FONT_MISS: u32 = 0x22;

// pub const   MSG_OUT_NEW_DL_ID: u32          = 0x30;
pub const MSG_OUT_NEW_TX_ID: u32 = 0x31;
pub const MSG_OUT_NEW_FONT_ID: u32 = 0x32;

pub const CMD_RENDER_GRAPH: u32 = 0x01;
pub const CMD_CLEAR_GRAPH: u32 = 0x02;
pub const CMD_SET_ROOT: u32 = 0x03;
// pub const   CMD_CACHE_LOAD: u32             = 0x03;
// pub const   CMD_CACHE_RELEASE: u32          = 0x04;

pub const CMD_CLEAR_COLOR: u32 = 0x05;

pub const CMD_INPUT: u32 = 0x0A;

pub const CMD_QUIT: u32 = 0x20;
pub const CMD_QUERY_STATS: u32 = 0x21;
pub const CMD_RESHAPE: u32 = 0x22;
pub const CMD_POSITION: u32 = 0x23;
pub const CMD_FOCUS: u32 = 0x24;
pub const CMD_ICONIFY: u32 = 0x25;
pub const CMD_MAXIMIZE: u32 = 0x26;
pub const CMD_RESTORE: u32 = 0x27;
pub const CMD_SHOW: u32 = 0x28;
pub const CMD_HIDE: u32 = 0x29;

// pub const   CMD_NEW_DL_ID: u32              = 0x30;
// pub const   CMD_FREE_DL_ID: u32             = 0x31;

pub const CMD_NEW_TX_ID: u32 = 0x32;
pub const CMD_FREE_TX_ID: u32 = 0x33;
pub const CMD_PUT_TX_BLOB: u32 = 0x34;
pub const CMD_PUT_TX_RAW: u32 = 0x35;

pub const CMD_LOAD_FONT_FILE: u32 = 0x37;
pub const CMD_LOAD_FONT_BLOB: u32 = 0x38;
pub const CMD_FREE_FONT: u32 = 0x39;

// here to test recovery
pub const CMD_CRASH: u32 = 0xFE;

pub const MSG_KEY_MASK: u32 = 0x0001;
pub const MSG_CHAR_MASK: u32 = 0x0002;
pub const MSG_MOUSE_MOVE_MASK: u32 = 0x0004;
pub const MSG_MOUSE_BUTTON_MASK: u32 = 0x0008;
pub const MSG_MOUSE_SCROLL_MASK: u32 = 0x0010;
pub const MSG_MOUSE_ENTER_MASK: u32 = 0x0020;
pub const MSG_DROP_PATHS_MASK: u32 = 0x0040;
pub const MSG_RESHAPE_MASK: u32 = 0x0080;

// state control
pub const OP_PUSH_STATE: u32 = 0x01;
pub const OP_POP_STATE: u32 = 0x02;
pub const OP_RESET_STATE: u32 = 0x03;

pub const OP_RUN_SCRIPT: u32 = 0x04;

// RENDER STYLES
pub const OP_PAINT_LINEAR: u32 = 0x06;
pub const OP_PAINT_BOX: u32 = 0x07;
pub const OP_PAINT_RADIAL: u32 = 0x08;
pub const OP_PAINT_IMAGE: u32 = 0x09;

pub const OP_ANTI_ALIAS: u32 = 0x0A;

pub const OP_STROKE_WIDTH: u32 = 0x0C;
pub const OP_STROKE_COLOR: u32 = 0x0D;
pub const OP_STROKE_PAINT: u32 = 0x0E;

pub const OP_FILL_COLOR: u32 = 0x10;
pub const OP_FILL_PAINT: u32 = 0x11;

pub const OP_MITER_LIMIT: u32 = 0x14;
pub const OP_LINE_CAP: u32 = 0x15;
pub const OP_LINE_JOIN: u32 = 0x16;
pub const OP_GLOBAL_ALPHA: u32 = 0x17;

// SCISSORING
pub const OP_SCISSOR: u32 = 0x1B;
pub const OP_INTERSECT_SCISSOR: u32 = 0x1C;
pub const OP_RESET_SCISSOR: u32 = 0x1D;

// PATH OPERATIONS
pub const OP_PATH_BEGIN: u32 = 0x20;

pub const OP_PATH_MOVE_TO: u32 = 0x21;
pub const OP_PATH_LINE_TO: u32 = 0x22;
pub const OP_PATH_BEZIER_TO: u32 = 0x23;
pub const OP_PATH_QUADRATIC_TO: u32 = 0x24;
pub const OP_PATH_ARC_TO: u32 = 0x25;
pub const OP_PATH_CLOSE: u32 = 0x26;
pub const OP_PATH_WINDING: u32 = 0x27;

pub const OP_FILL: u32 = 0x29;
pub const OP_STROKE: u32 = 0x2A;

pub const OP_TRIANGLE: u32 = 0x2C;
pub const OP_ARC: u32 = 0x2D;
pub const OP_RECT: u32 = 0x2E;
pub const OP_ROUND_RECT: u32 = 0x2F;
pub const OP_ROUND_RECT_VAR: u32 = 0x30;
pub const OP_ELLIPSE: u32 = 0x31;
pub const OP_CIRCLE: u32 = 0x32;
pub const OP_SECTOR: u32 = 0x33;

pub const OP_TEXT: u32 = 0x34;

// TRANSFORM OPERATIONS
pub const OP_TX_RESET: u32 = 0x36;
pub const OP_TX_IDENTITY: u32 = 0x37;
pub const OP_TX_MATRIX: u32 = 0x38;
pub const OP_TX_TRANSLATE: u32 = 0x39;
pub const OP_TX_SCALE: u32 = 0x3A;
pub const OP_TX_ROTATE: u32 = 0x3B;
pub const OP_TX_SKEW_X: u32 = 0x3C;
pub const OP_TX_SKEW_Y: u32 = 0x3D;

pub const OP_FONT: u32 = 0x40;
pub const OP_FONT_BLUR: u32 = 0x41;
pub const OP_FONT_SIZE: u32 = 0x42;
pub const OP_TEXT_ALIGN: u32 = 0x43;
pub const OP_TEXT_HEIGHT: u32 = 0x44;

pub const OP_TERMINATE: u32 = 0xFF;

pub const NVG_PI: f32 = 3.14159265358979323846264338327f32;
pub const TAU: f32 = NVG_PI * 2.0;
