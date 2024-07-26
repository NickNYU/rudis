//! Provides a type representing a Redis protocol Protocol as well as utilities for
//! parsing Protocols from a byte array.

use bytes::{Buf, Bytes};
use std::convert::TryInto;
use std::fmt;
use std::io::Cursor;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

/// A Protocol in the Redis protocol.
#[derive(Clone, Debug)]
pub enum Protocol {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Protocol>),
}

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,

    /// Invalid message encoding
    Other(crate::Error),
}

impl Protocol {
    /// Returns an empty array
    pub fn array() -> Protocol {
        Protocol::Array(vec![])
    }

    /// Push a "bulk" Protocol into the array. `self` must be an Array Protocol.
    ///
    /// # Panics
    ///
    /// panics if `self` is not an array
    pub fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Protocol::Array(vec) => {
                vec.push(Protocol::Bulk(bytes));
            }
            _ => panic!("not an array Protocol"),
        }
    }

    /// Push an "integer" Protocol into the array. `self` must be an Array Protocol.
    ///
    /// # Panics
    ///
    /// panics if `self` is not an array
    pub(crate) fn push_int(&mut self, value: u64) {
        match self {
            Protocol::Array(vec) => {
                vec.push(Protocol::Integer(value));
            }
            _ => panic!("not an array Protocol"),
        }
    }

    /// Checks if an entire message can be decoded from `src`
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match get_u8(src)? {
            b'+' => {
                get_line(src)?;
                Ok(())
            }
            b'-' => {
                get_line(src)?;
                Ok(())
            }
            b':' => {
                let _ = get_decimal(src)?;
                Ok(())
            }
            b'$' => {
                if b'-' == peek_u8(src)? {
                    // Skip '-1\r\n'
                    skip(src, 4)
                } else {
                    // Read the bulk string
                    let len: usize = get_decimal(src)?.try_into()?;

                    // skip that number of bytes + 2 (\r\n).
                    skip(src, len + 2)
                }
            }
            b'*' => {
                let len = get_decimal(src)?;

                for _ in 0..len {
                    Protocol::check(src)?;
                }

                Ok(())
            }
            actual => Err(format!("protocol error; invalid Protocol type byte `{}`", actual).into()),
        }
    }

    /// The message has already been validated with `check`.
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Protocol, Error> {
        match get_u8(src)? {
            b'+' => {
                // Read the line and convert it to `Vec<u8>`
                let line = get_line(src)?.to_vec();

                // Convert the line to a String
                let string = String::from_utf8(line)?;

                Ok(Protocol::Simple(string))
            }
            b'-' => {
                // Read the line and convert it to `Vec<u8>`
                let line = get_line(src)?.to_vec();

                // Convert the line to a String
                let string = String::from_utf8(line)?;

                Ok(Protocol::Error(string))
            }
            b':' => {
                let len = get_decimal(src)?;
                Ok(Protocol::Integer(len))
            }
            b'$' => {
                if b'-' == peek_u8(src)? {
                    let line = get_line(src)?;

                    if line != b"-1" {
                        return Err("protocol error; invalid Protocol format".into());
                    }

                    Ok(Protocol::Null)
                } else {
                    // Read the bulk string
                    let len = get_decimal(src)?.try_into()?;
                    let n = len + 2;

                    if src.remaining() < n {
                        return Err(Error::Incomplete);
                    }

                    let data = Bytes::copy_from_slice(&src.chunk()[..len]);

                    // skip that number of bytes + 2 (\r\n).
                    skip(src, n)?;

                    Ok(Protocol::Bulk(data))
                }
            }
            b'*' => {
                let len = get_decimal(src)?.try_into()?;
                let mut out = Vec::with_capacity(len);

                for _ in 0..len {
                    out.push(Protocol::parse(src)?);
                }

                Ok(Protocol::Array(out))
            }
            _ => unimplemented!(),
        }
    }

    /// Converts the Protocol to an "unexpected Protocol" error
    pub(crate) fn to_error(&self) -> crate::Error {
        format!("unexpected Protocol: {}", self).into()
    }
}

impl PartialEq<&str> for Protocol {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Protocol::Simple(s) => s.eq(other),
            Protocol::Bulk(s) => s.eq(other),
            _ => false,
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        match self {
            Protocol::Simple(response) => response.fmt(fmt),
            Protocol::Error(msg) => write!(fmt, "error: {}", msg),
            Protocol::Integer(num) => num.fmt(fmt),
            Protocol::Bulk(msg) => match str::from_utf8(msg) {
                Ok(string) => string.fmt(fmt),
                Err(_) => write!(fmt, "{:?}", msg),
            },
            Protocol::Null => "(nil)".fmt(fmt),
            Protocol::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        // use space as the array element display separator
                        write!(fmt, " ")?;
                    }

                    part.fmt(fmt)?;
                }

                Ok(())
            }
        }
    }
}

fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.chunk()[0])
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u8())
}

fn skip(src: &mut Cursor<&[u8]>, n: usize) -> Result<(), Error> {
    if src.remaining() < n {
        return Err(Error::Incomplete);
    }

    src.advance(n);
    Ok(())
}

/// Read a new-line terminated decimal
fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u64, Error> {
    use atoi::atoi;

    let line = get_line(src)?;

    atoi::<u64>(line).ok_or_else(|| "protocol error; invalid Protocol format".into())
}

/// Find a line
fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);

            // Return the line
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(Error::Incomplete)
}

impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::Other(src.into())
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        src.to_string().into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        "protocol error; invalid Protocol format".into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid Protocol format".into()
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "stream ended early".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
        }
    }
}
