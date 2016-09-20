//! Unsafe casting from assorted `[u8; N] -> T`.
//!
//! We're going to need to cast a bunch of u8 types as other data-types; this module is
//! designed to limit the unsafety to the extent possible.

pub fn as_u8(n: [u8; 1]) -> u8 {
    n[0]
}

pub fn as_i8(n: [u8; 1]) -> i8 {
    n[0] as i8
}

pub fn as_i16(n: [u8; 2]) -> i16 {
    i16::from_be(unsafe { ::std::mem::transmute(n) })
}

pub fn as_i32(n: [u8; 4]) -> i32 {
    i32::from_be(unsafe { ::std::mem::transmute(n) })
}

pub fn as_u32(n: [u8; 4]) -> u32 {
    u32::from_be(unsafe { ::std::mem::transmute(n) })
}

pub fn as_f32(n: [u8; 4]) -> f32 {
    unsafe { ::std::mem::transmute(n) }
}

pub fn as_f64(n: [u8; 8]) -> f64 {
    unsafe { ::std::mem::transmute(n) }
}
