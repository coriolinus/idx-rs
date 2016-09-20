use std::io;
use std::io::Read;

mod casts;

/// The type of data represented in this .idx file.
///
/// A given file will always contain a single data type.
/// The type is specified in the third byte of the file:
///
/// ```text
/// 0x08: unsigned byte
/// 0x09: signed byte
/// 0x0B: short (2 bytes)
/// 0x0C: int (4 bytes)
/// 0x0D: float (4 bytes)
/// 0x0E: double (8 bytes)
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataType {
    U8,
    I8,
    I16,
    I32,
    F32,
    F64,
}

impl DataType {
    /// How many bytes does this datatype take?
    pub fn bytes(&self) -> usize {
        match *self {
            DataType::U8 => 1,
            DataType::I8 => 1,
            DataType::I16 => 2,
            DataType::I32 => 4,
            DataType::F32 => 4,
            DataType::F64 => 8,
        }
    }
}

#[derive(Debug)]
pub enum IdxErr {
    IOError(io::Error),
    WrongHeader,
    UnknownDataType,
}


pub struct IdxReader<R: Read> {
    source: R,
    data_type: DataType,
    dimensions: Vec<u32>,
}

impl<R: Read> IdxReader<R> {
    /// Creates a new IdxReader from the given reader, immediately parsing the header.
    ///
    /// If an error is encountered while parsing the Idx header, an error is returned.
    pub fn new(mut reader: R) -> Result<IdxReader<R>, IdxErr> {
        let mut byte32: [u8; 4] = [0; 4];
        // get magic number and header data
        try!(reader.read_exact(&mut byte32).map_err(|e| IdxErr::IOError(e)));

        // first two bytes must always be 0
        if byte32[0] != 0 || byte32[1] != 0 {
            return Err(IdxErr::WrongHeader);
        }

        // 0x08: unsigned byte
        // 0x09: signed byte
        // 0x0B: short (2 bytes)
        // 0x0C: int (4 bytes)
        // 0x0D: float (4 bytes)
        // 0x0E: double (8 bytes)
        let dt = match byte32[2] {
            0x08 => DataType::U8,
            0x09 => DataType::I8,
            0x0B => DataType::I16,
            0x0C => DataType::I32,
            0x0D => DataType::F32,
            0x0E => DataType::F64,
            _ => return Err(IdxErr::UnknownDataType),
        };

        let n_dimensions = byte32[3];

        let mut dimensions = Vec::with_capacity(n_dimensions as usize);
        for _ in 0..n_dimensions {
            // update byte32 with the next 4 bytes from the input
            try!(reader.read_exact(&mut byte32).map_err(|e| IdxErr::IOError(e)));

            dimensions.push(casts::as_u32(byte32));
        }

        Ok(IdxReader {
            source: reader,
            data_type: dt,
            dimensions: dimensions,
        })
    }
}
