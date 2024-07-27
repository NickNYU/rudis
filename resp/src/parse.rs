use crate::protocol::Protocol;
use bytes::Bytes;
use std::{fmt, str, vec};

/// Utility for parsing a command
///
/// Commands are represented as array Protocols. Each entry in the Protocol is a
/// "token". A `Parse` is initialized with the array Protocol and provides a
/// cursor-like API. Each command struct includes a `parse_Protocol` method that
/// uses a `Parse` to extract its fields.
#[derive(Debug)]
pub struct Parser {
    /// Array Protocol iterator.
    parts: vec::IntoIter<Protocol>,
}

/// Error encountered while parsing a Protocol.
///
/// Only `EndOfStream` errors are handled at runtime. All other errors result in
/// the connection being terminated.
#[derive(Debug)]
pub enum ParseError {
    /// Attempting to extract a value failed due to the Protocol being fully
    /// consumed.
    EndOfStream,

    /// All other errors
    Other(crate::Error),
}

impl Parser {
    /// Create a new `Parse` to parse the contents of `protocol`.
    ///
    /// Returns `Err` if `protocol` is not an array protocol.
    pub fn new(protocol: Protocol) -> Result<Parser, ParseError> {
        let array = match protocol {
            Protocol::Array(array) => array,
            protocol => return Err(format!("protocol error; expected array, got {:?}", protocol).into()),
        };

        Ok(Parser {
            parts: array.into_iter(),
        })
    }

    /// Return the next entry. Array Protocols are arrays of Protocols, so the next
    /// entry is a Protocol.
    fn next(&mut self) -> Result<Protocol, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfStream)
    }

    /// Return the next entry as a string.
    ///
    /// If the next entry cannot be represented as a String, then an error is returned.
    pub fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            // Both `Simple` and `Bulk` representation may be strings. Strings
            // are parsed to UTF-8.
            //
            // While errors are stored as strings, they are considered separate
            // types.
            Protocol::Simple(s) => Ok(s),
            Protocol::Bulk(data) => str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_| "protocol error; invalid string".into()),
            protocol => Err(format!(
                "protocol error; expected simple Protocol or bulk Protocol, got {:?}",
                protocol
            )
                .into()),
        }
    }

    /// Return the next entry as raw bytes.
    ///
    /// If the next entry cannot be represented as raw bytes, an error is
    /// returned.
    pub fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            // Both `Simple` and `Bulk` representation may be raw bytes.
            //
            // Although errors are stored as strings and could be represented as
            // raw bytes, they are considered separate types.
            Protocol::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Protocol::Bulk(data) => Ok(data),
            protocol => Err(format!(
                "protocol error; expected simple Protocol or bulk Protocol, got {:?}",
                protocol
            )
                .into()),
        }
    }

    /// Return the next entry as an integer.
    ///
    /// This includes `Simple`, `Bulk`, and `Integer` Protocol types. `Simple` and
    /// `Bulk` Protocol types are parsed.
    ///
    /// If the next entry cannot be represented as an integer, then an error is
    /// returned.
    // pub(crate) fn next_int(&mut self) -> Result<u64, ParseError> {
    //     use atoi::atoi;
    //
    //     const MSG: &str = "protocol error; invalid number";
    //
    //     match self.next()? {
    //         // An integer Protocol type is already stored as an integer.
    //         Protocol::Integer(v) => Ok(v),
    //         // Simple and bulk Protocols must be parsed as integers. If the parsing
    //         // fails, an error is returned.
    //         Protocol::Simple(data) => atoi::<u64>(data.as_bytes()).ok_or_else(|| MSG.into()),
    //         Protocol::Bulk(data) => atoi::<u64>(&data).ok_or_else(|| MSG.into()),
    //         protocol => Err(format!("protocol error; expected int Protocol but got {:?}", protocol).into()),
    //     }
    // }

    /// Ensure there are no more entries in the array
    pub fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err("protocol error; expected end of Protocol, but there was more".into())
        }
    }
}

impl From<String> for ParseError {
    fn from(src: String) -> ParseError {
        ParseError::Other(src.into())
    }
}

impl From<&str> for ParseError {
    fn from(src: &str) -> ParseError {
        src.to_string().into()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
            ParseError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}
