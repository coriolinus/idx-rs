//! Unsafe casting from assorted `[u8; N] -> T`.
//!
//! We're going to need to cast a bunch of u8 types as other data-types; this module is
//! designed to limit the unsafety to the extent possible.

use std::io;
use std::io::Read;

// We need a parameterizied way to actually get values out of a stream of u8s, so
// let's set up a trait here.
pub trait ValueExtractor<R: Read, T> {
    fn extract(&mut self) -> io::Result<T>;
}

// Implementations of ValueExtractor.
//
// I'd really wanted to macroize this, but it looks like this is simply impossible.
// Rust's macros aren't doing simple text substitution, but something a little more
// deeply tied into the type system, and all of the ways I could think of to macroize
// this require simple text substitution. Oh well, it's not that terrible to just
// customize the copy/paste here.
impl<R: Read> ValueExtractor<R, u8> for R {
    fn extract(&mut self) -> io::Result<u8> {
        let mut data_buf = [0; 1];
        try!(self.read_exact(&mut data_buf));
        Ok(data_buf[0])
    }
}

impl<R: Read> ValueExtractor<R, i8> for R {
    fn extract(&mut self) -> io::Result<i8> {
        let mut data_buf = [0; 1];
        try!(self.read_exact(&mut data_buf));
        Ok(data_buf[0] as i8)
    }
}

impl<R: Read> ValueExtractor<R, i16> for R {
    fn extract(&mut self) -> io::Result<i16> {
        let mut data_buf = [0; 2];
        try!(self.read_exact(&mut data_buf));
        Ok(i16::from_be(unsafe { ::std::mem::transmute(data_buf) }))
    }
}

impl<R: Read> ValueExtractor<R, u32> for R {
    fn extract(&mut self) -> io::Result<u32> {
        let mut data_buf = [0; 4];
        try!(self.read_exact(&mut data_buf));
        Ok(u32::from_be(unsafe { ::std::mem::transmute(data_buf) }))
    }
}

impl<R: Read> ValueExtractor<R, i32> for R {
    fn extract(&mut self) -> io::Result<i32> {
        let mut data_buf = [0; 4];
        try!(self.read_exact(&mut data_buf));
        Ok(i32::from_be(unsafe { ::std::mem::transmute(data_buf) }))
    }
}

impl<R: Read> ValueExtractor<R, f32> for R {
    fn extract(&mut self) -> io::Result<f32> {
        let mut data_buf = [0; 4];
        try!(self.read_exact(&mut data_buf));
        Ok(unsafe { ::std::mem::transmute(data_buf) })
    }
}

impl<R: Read> ValueExtractor<R, f64> for R {
    fn extract(&mut self) -> io::Result<f64> {
        let mut data_buf = [0; 8];
        try!(self.read_exact(&mut data_buf));
        Ok(unsafe { ::std::mem::transmute(data_buf) })
    }
}
