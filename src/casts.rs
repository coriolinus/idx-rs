//! Unsafe casting from assorted `[u8; N] -> T`.
//!
//! We're going to need to cast a bunch of u8 types as other data-types; this module is
//! designed to limit the unsafety to the extent possible.

// We need a parameterizied way to actually get values out of a stream of u8s, so
// let's set up a trait here.
trait ValueExtractor<R: Read, T> {
    fn extract(&mut self) -> io::Result<T>;
}

// single implementation--TODO: replace with a macro for all appropriate types
impl<R: Read> ValueExtractor<R, u8> for R {
    fn extract(&mut self) -> io::Result<u8> {
        let mut data_buf = [0; 1];
        try!(self.read_exact(&mut data_buf));
        Ok(casts::as_u8(data_buf))
    }
}

/// How many bytes are contained in certain primitives.
///
/// Honestly surprised that this isn't apparently in std, but
/// the closest thing I could find was std::mem::size_of, which
/// isn't a macro, and so isn't appropriate for this use case.
macro_rules! bytes_of {
    (u8) => (1);
    (i8) => (1);
    (i16) => (2);
    (i32) => (4);
    (f32) => (4);
    (f64) => (8);
}

/// Detailed transformation implementation
///
/// Here we just match the data type and perform the appropriate transformation
macro_rules! xform {
    ( $buf:ident, u8) => ($buf[0]);
    ( $buf:ident, i8) => ($buf[0] as i8);
    ( $buf:ident, i16) => (i16::from_be(unsafe { ::std::mem::transmute($buf) }));
    ( $buf:ident, i32) => (i32::from_be(unsafe { ::std::mem::transmute($buf) }));
    ( $buf:ident, f32) => (unsafe { ::std::mem::transmute($buf) });
    ( $buf:ident, f64) => (unsafe { ::std::mem::transmute($buf) });
}

/// Automatically implement ValueExtractor for the given type.
///
/// Only works for types for which we've implemented appropriate cases
/// of `bytes_of!($x)` and `casts::as_$x()`.
macro_rules! ve_impl {
    ( $( $x:ty ),* ) => (
        $(
            impl<R: Read> ValueExtractor<R, $x> for R {
                fn extract(&mut self) -> io::Result<$x> {
                    let mut data_buf = [0; bytes_of!($x)];
                    try!(self.read_exact(&mut data_buf));
                    Ok(xform!(data_buf, $x))
                }
            }
        )*
    );
}

ve_impl!{i8, i16, i32, f32, f64}
