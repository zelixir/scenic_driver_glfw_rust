#![allow(unused)]
pub const   MSG_OUT_CLOSE: u32              = 0x00;
pub const   MSG_OUT_STATS: u32              = 0x01;
pub const   MSG_OUT_PUTS: u32               = 0x02;
pub const   MSG_OUT_WRITE: u32              = 0x03;
pub const   MSG_OUT_INSPECT: u32            = 0x04;
pub const   MSG_OUT_RESHAPE: u32            = 0x05;
pub const   MSG_OUT_READY: u32              = 0x06;
pub const   MSG_OUT_DRAW_READY: u32         = 0x07;

pub const   MSG_OUT_KEY: u32                = 0x0A;
pub const   MSG_OUT_CODEPOINT: u32          = 0x0B;
pub const   MSG_OUT_CURSOR_POS: u32         = 0x0C;
pub const   MSG_OUT_MOUSE_BUTTON: u32       = 0x0D;
pub const   MSG_OUT_MOUSE_SCROLL: u32       = 0x0E;
pub const   MSG_OUT_CURSOR_ENTER: u32       = 0x0F;
pub const   MSG_OUT_DROP_PATHS: u32         = 0x10;
pub const   MSG_OUT_CACHE_MISS: u32         = 0x20;

pub const   MSG_OUT_FONT_MISS: u32          = 0x22;

// pub const   MSG_OUT_NEW_DL_ID: u32          = 0x30;
pub const   MSG_OUT_NEW_TX_ID: u32          = 0x31;
pub const   MSG_OUT_NEW_FONT_ID: u32        = 0x32;



pub const   CMD_RENDER_GRAPH: u32           = 0x01;
pub const   CMD_CLEAR_GRAPH: u32            = 0x02;
pub const   CMD_SET_ROOT: u32               = 0x03;
// pub const   CMD_CACHE_LOAD: u32             = 0x03;
// pub const   CMD_CACHE_RELEASE: u32          = 0x04;

pub const   CMD_CLEAR_COLOR: u32            = 0x05;

pub const   CMD_INPUT: u32                  = 0x0A;

pub const   CMD_QUIT: u32                   = 0x20;
pub const   CMD_QUERY_STATS: u32            = 0x21;
pub const   CMD_RESHAPE: u32                = 0x22;
pub const   CMD_POSITION: u32               = 0x23;
pub const   CMD_FOCUS: u32                  = 0x24;
pub const   CMD_ICONIFY: u32                = 0x25;
pub const   CMD_MAXIMIZE: u32               = 0x26;
pub const   CMD_RESTORE: u32                = 0x27;
pub const   CMD_SHOW: u32                   = 0x28;
pub const   CMD_HIDE: u32                   = 0x29;

// pub const   CMD_NEW_DL_ID: u32              = 0x30;
// pub const   CMD_FREE_DL_ID: u32             = 0x31;

pub const   CMD_NEW_TX_ID: u32              = 0x32;
pub const   CMD_FREE_TX_ID: u32             = 0x33;
pub const   CMD_PUT_TX_BLOB: u32            = 0x34;
pub const   CMD_PUT_TX_RAW: u32             = 0x35;


pub const   CMD_LOAD_FONT_FILE: u32         = 0x37;
pub const   CMD_LOAD_FONT_BLOB: u32         = 0x38;
pub const   CMD_FREE_FONT: u32              = 0x39;

// here to test recovery
pub const   CMD_CRASH: u32                  = 0xFE;
