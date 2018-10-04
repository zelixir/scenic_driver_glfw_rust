use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use comms::*;
use gl::*;

use std::io::Read;

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
                send_puts(format!("{} INVALID_FRAMEBUFFER_OPERATION", msg));
            }
            _ => send_puts(format!("{} GL_OTHER: {}", msg, err)),
        }
    }
}
fn trim_null_tail(mut buf: Vec<u8>) -> Vec<u8> {
    let mut i = buf.len() - 1;
    while buf[i] == 0 {
        i = i - 1;
    }
    buf.truncate(i + 1);
    buf
}
pub fn read_string(read: &mut impl Read, len: usize) -> String {
    String::from_utf8(trim_null_tail(read_bytes(read, len))).unwrap()
}
pub fn read_bytes(read: &mut impl Read, len: usize) -> Vec<u8> {
    let mut re: Vec<u8> = vec![0u8; len];
    read.read_exact(re.as_mut_slice()).unwrap();
    re
}

trait ReadTuple: Sized {
    fn read_tuple(read: &mut impl ReadBytesExt) -> ::std::io::Result<Self>;
}
trait WriteTuple: Sized {
    fn write_tuple(write: &mut impl WriteBytesExt, data: Self) -> ::std::io::Result<()>;
}

macro_rules! read_type {
    ($self:expr,i32) => {
        $self.read_i32::<NativeEndian>()
    };
    ($self:expr,u32) => {
        $self.read_u32::<NativeEndian>()
    };
    ($self:expr,usize) => {
        $self
            .read_u32::<NativeEndian>()
            .and_then(|x| Ok(x as usize))
    };
    ($self:expr,f32) => {
        $self.read_f32::<NativeEndian>()
    };
    ($self:expr,Color) => {
        (Ok(()) as ::std::io::Result<()>).and_then(|_| {
            Ok(Color::from_rgba(
                $self.read_u32::<NativeEndian>()? as u8,
                $self.read_u32::<NativeEndian>()? as u8,
                $self.read_u32::<NativeEndian>()? as u8,
                $self.read_u32::<NativeEndian>()? as u8,
            ))
        })
    };
    ($self:expr,bool) => {
        $self.read_u32::<NativeEndian>().and_then(|x| Ok(x != 0))
    };
}

pub trait WriteAny {
    fn write_any(self, &mut impl WriteBytesExt) -> ::std::io::Result<()>;
}
impl WriteAny for bool {
    fn write_any(self, write: &mut impl WriteBytesExt) -> ::std::io::Result<()> {
        write.write_u32::<NativeEndian>(self as u32)
    }
}
impl WriteAny for i32 {
    fn write_any(self, write: &mut impl WriteBytesExt) -> ::std::io::Result<()> {
        write.write_i32::<NativeEndian>(self)
    }
}
impl WriteAny for u32 {
    fn write_any(self, write: &mut impl WriteBytesExt) -> ::std::io::Result<()> {
        write.write_u32::<NativeEndian>(self)
    }
}
impl WriteAny for f32 {
    fn write_any(self, write: &mut impl WriteBytesExt) -> ::std::io::Result<()> {
        write.write_f32::<NativeEndian>(self)
    }
}

macro_rules! read_multi {
    ($read:ident, $tyvar:ident ) => {
        read_type!($read, $tyvar)
    };
    ($read:ident, $($tyvar:ident),* ) => {
         (Ok(()) as ::std::io::Result<()>).and_then(|_|{
            Ok( ( $( (try!(read_type!($read, $tyvar))) ),* ) )
        })
    }
}

macro_rules! write_multi {
    ($write:ident, $data:expr ) => {
        $data.write_any(&mut $write)
    };
    ($write:ident, $head:expr, $($tail:expr),+ ) => {
        write_multi!($write, $head).and_then(|_|write_multi!($write, $($tail), *))
    }
}

#[test]
fn read_write_multi_test() {
    use std::io::Cursor;
    let vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 0xff];
    let mut r = Cursor::new(vec);
    let (a, b) = read_multi!(r, i32, u32).expect("read_multi_test failed");
    assert_eq!(0x04030201, a);
    assert_eq!(0xff070605, b);

    assert!(read_multi!(r, i32).is_err());

    let mut vec: Vec<u8> = vec![];
    write_multi!(vec, 0x04030201i32, 0xff070605u32).expect("write_multi_test failed");
    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 0xff]);
}

pub trait ScenicResult<T> {
    fn expect_or_send(self, msg: &str) -> T;
}

impl<T, E: ::std::fmt::Debug> ScenicResult<T> for Result<T, E> {
    fn expect_or_send(self, msg: &str) -> T {
        self.map_err(|err| {
            send_puts(msg.to_string());
            err
        }).expect(msg)
    }
}
impl<T> ScenicResult<T> for Option<T> {
    fn expect_or_send(self, msg: &str) -> T {
        self.ok_or(()).expect_or_send(msg)
    }
}
