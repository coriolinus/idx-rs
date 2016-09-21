use std::io;
use std::io::Read;
use std::marker::PhantomData;

mod casts;
use casts::ValueExtractor;

#[derive(Debug)]
pub enum IdxErr {
    IOError(io::Error),
    WrongHeader,
    UnknownDataType,
}

pub struct IdxReader<R: Read, T> {
    source: R,
    data_type: PhantomData<T>,
    dimensions: Vec<u32>,
}

impl<R: Read, T> IdxReader<R, T> {
    /// Creates a new IdxReader from the given reader, immediately parsing the header.
    ///
    /// If an error is encountered while parsing the Idx header, an error is returned.
    pub fn new(mut reader: R) -> Result<IdxReader<R, T>, IdxErr> {
        let mut byte32: [u8; 4] = [0; 4];
        // get magic number and header data
        try!(reader.read_exact(&mut byte32).map_err(|e| IdxErr::IOError(e)));

        // first two bytes must always be 0
        if byte32[0] != 0 || byte32[1] != 0 {
            return Err(IdxErr::WrongHeader);
        }

        // The goal is to instantiate an IdxReader<R, T>, where T depends on the contents of
        // byte 3 of the Idx file. Ways that I know don't work:
        //
        // 1. Just `struct IdxReader<R>`. The problem with this is that we need a method of signature
        //    `pub fn extract_next(&mut self) -> T`, which produces an appropriate T.
        // 2. `struct IdxReader<R, T>`. The problem with this is that we don't know ahead of time which T
        //    will be produced. No matter how many layers of indirection we go back, at some point we need
        //    to allocate an IdxReader, but we don't know what type it will have in advance.
        // 3. `trait IdxReader { type Item; }; impl IdxReader for ActualIdxReader`. If we could do
        //    codegen / realtime compilation and code insertion, this approach might have promise. We'd
        //    just emit appropriate code to hide the relevant type behind a trait, and then things would
        //    work. Unfortunately, I know of no way to do codegen / code insertion in this language
        //    at runtime.
        //
        // Ways which might work but I don't like:
        // 1. ```
        //    pub enum DataType {
        //        U8(u8),
        //        I8(i8),
        //        ...
        //    }
        //    ```
        //    We could create a struct which wrapped each data type, then always return a struct member.
        //    However, I don't want to take that approach, for two reasons.
        //
        //    A. It makes the consumers of this library have to jump through some ugly hoops, matching
        //       on the datatype whenever they want to actually use it.
        //    B. It's insanely wasteful of space. An enum takes, bare minimum,
        //       `1 + size_of(largest_contained_member)` bytes. This means that even when we're
        //       actually working with simple `u8`s, each one takes at least 9 bytes of memory because the
        //       enum has to contain `f64`s. Once aligned, that becomes 16.
        // 2. Write a macro which duplicates implementation of `trait IdxReader { type Item; }` for
        //    all relevant `$t:ty`, and just return an error if the header byte doesn't match the expected
        //    type. This is the least worst option I've come up with so far, but it's still pretty bad,
        //    just because the ergonomics are terrible. The user just has to know in advance what sort of
        //    file it is.
        //
        //    There's one way that I can see around it: have the macro also write a function
        //    with the signature `create_reader<R: Read + Clone>(reader: R) -> Result<Box<IdxReader>, IdxErr>`.
        //    (It would be much nicer if we could return `-> Result<impl IdxReader, IdxErr>`, but that's
        //    still yet to come, unfortunately.) This function simply attempts to create the reader using
        //    every generated implementation, and filters out all the ones which fail. The key line
        //    probably looks something like
        //    `$implementations.iter().map(|i| i.try_create(reader)).filter(|result| result.is_ok())`.
        //
        //    I'm not sure if that's actually possible, but if it is, it might clear up a lot of the
        //    UI pain I'm going through now, so I think I'll attack it from that direction next.
        //
        // I'm honestly not sure if this is a soluble problem right now in this language. I may end
        // up producing a partial implementation specialized for `u8`s, as I still need to read some
        // IDX files, but that's really not a satisfactory solution to the problem.

        // 0x08: unsigned byte
        // 0x09: signed byte
        // 0x0B: short (2 bytes)
        // 0x0C: int (4 bytes)
        // 0x0D: float (4 bytes)
        // 0x0E: double (8 bytes)
        // let dt = match byte32[2] {
        //     0x08 => DataType::U8,
        //     0x09 => DataType::I8,
        //     0x0B => DataType::I16,
        //     0x0C => DataType::I32,
        //     0x0D => DataType::F32,
        //     0x0E => DataType::F64,
        //     _ => return Err(IdxErr::UnknownDataType),
        // };

        let n_dimensions = byte32[3];

        let mut dimensions = Vec::with_capacity(n_dimensions as usize);
        for _ in 0..n_dimensions {
            dimensions.push(try!(reader.extract().map_err(|e| IdxErr::IOError(e))));
        }

        Ok(IdxReader {
            source: reader,
            data_type: PhantomData,
            dimensions: dimensions,
        })
    }
}
