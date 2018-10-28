use std::result;

/// Errors that can be returned by encoding and decoding operations.
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    /// An encoded string contained a byte value not valid for the encoding
    /// alphabet.
    InvalidEncodingChar,
    /// The padding is incorrect for the length of the output. Only applicable
    /// to Base64 and Base32.
    BadPadding,
}

pub type Result<T> = result::Result<T, ErrorKind>;
