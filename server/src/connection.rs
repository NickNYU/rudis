use resp::{self, Result, protocol::Protocol};

use bytes::{Buf, BytesMut};
use std::io::{self, Cursor, Read, Write};
use mio::net::TcpStream;

/// Send and receive `Protocol` values from a remote peer.
///
/// When implementing networking protocols, a message on that protocol is
/// often composed of several smaller messages known as Protocols. The purpose of
/// `Connection` is to read and write Protocols on the underlying `TcpStream`.
///
/// To read Protocols, the `Connection` uses an internal buffer, which is filled
/// up until there are enough bytes to create a full Protocol. Once this happens,
/// the `Connection` creates the Protocol and returns it to the caller.
///
/// When sending Protocols, the Protocol is first encoded into the write buffer.
/// The contents of the write buffer are then written to the socket.
#[derive(Debug)]
pub struct Connection {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    // stream: BufWriter<TcpStream>,

    tcp_stream: TcpStream,

    // The buffer for reading Protocols.
    buffer: BytesMut,
}



impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            // stream: BufWriter::new(socket),
            // Default to a 4KB read buffer. For the use case of mini redis,
            // this is fine. However, real applications will want to tune this
            // value to their specific use case. There is a high likelihood that
            // a larger read buffer will work better.
            buffer: BytesMut::with_capacity(4 * 1024),
            tcp_stream: socket
        }
    }

    /// Read a single `Protocol` value from the underlying stream.
    ///
    /// The function waits until it has retrieved enough data to parse a Protocol.
    /// Any data remaining in the read buffer after the Protocol has been parsed is
    /// kept there for the next call to `read_protocol`.
    ///
    /// # Returns
    ///
    /// On success, the received Protocol is returned. If the `TcpStream`
    /// is closed in a way that doesn't break a Protocol in half, it returns
    /// `None`. Otherwise, an error is returned.
    pub fn read_protocol(&mut self) -> Result<Option<Protocol>> {
        loop {
            // Attempt to parse a Protocol from the buffered data. If enough data
            // has been buffered, the Protocol is returned.
            if let Some(Protocol) = self.parse_protocol()? {
                return Ok(Some(Protocol));
            }

            // There is not enough buffered data to read a Protocol. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            if 0 == self.tcp_stream.read(&mut self.buffer).unwrap() {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a Protocol.
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("connection reset by peer".into())
                }
            }
        }
    }

    /// Tries to parse a Protocol from the buffer. If the buffer contains enough
    /// data, the Protocol is returned and the data removed from the buffer. If not
    /// enough data has been buffered yet, `Ok(None)` is returned. If the
    /// buffered data does not represent a valid Protocol, `Err` is returned.
    fn parse_protocol(&mut self) -> Result<Option<Protocol>> {
        // use Protocol::Error::Incomplete;

        // Cursor is used to track the "current" location in the
        // buffer. Cursor also implements `Buf` from the `bytes` crate
        // which provides a number of helpful utilities for working
        // with bytes.
        let mut buf = Cursor::new(&self.buffer[..]);

        // The first step is to check if enough data has been buffered to parse
        // a single Protocol. This step is usually much faster than doing a full
        // parse of the Protocol, and allows us to skip allocating data structures
        // to hold the Protocol data unless we know the full Protocol has been
        // received.
        match Protocol::check(&mut buf) {
            Ok(_) => {
                // The `check` function will have advanced the cursor until the
                // end of the protocol. Since the cursor had position set to zero
                // before `protocol::check` was called, we obtain the length of the
                // protocol by checking the cursor position.
                let len = buf.position() as usize;

                // Reset the position to zero before passing the cursor to
                // `protocol::parse`.
                buf.set_position(0);

                // Parse the protocol from the buffer. This allocates the necessary
                // structures to represent the protocol and returns the protocol
                // value.
                //
                // If the encoded protocol representation is invalid, an error is
                // returned. This should terminate the **current** connection
                // but should not impact any other connected client.
                let protocol = Protocol::parse(&mut buf)?;

                // Discard the parsed data from the read buffer.
                //
                // When `advance` is called on the read buffer, all the data
                // up to `len` is discarded. The details of how this works is
                // left to `BytesMut`. This is often done by moving an internal
                // cursor, but it may be done by reallocating and copying data.
                self.buffer.advance(len);

                // Return the parsed protocol to the caller.
                Ok(Some(protocol))
            }
            // There is not enough data present in the read buffer to parse a
            // single Protocol. We must wait for more data to be received from the
            // socket. Reading from the socket will be done in the statement
            // after this `match`.
            //
            // We do not want to return `Err` from here as this "error" is an
            // expected runtime condition.
            Err(_Incomplete) => Ok(None),
            // An error was encountered while parsing the Protocol. The connection
            // is now in an invalid state. Returning `Err` from here will result
            // in the connection being closed.
        }
    }

    /// Write a single `protocol` value to the underlying stream.
    ///
    /// The `protocol` value is written to the socket using the various `write_*`
    /// functions provided by `AsyncWrite`. Calling these functions directly on
    /// a `TcpStream` is **not** advised, as this will result in a large number of
    /// syscall. However, it is fine to call these functions on a *buffered*
    /// write stream. The data will be written to the buffer. Once the buffer is
    /// full, it is flushed to the underlying socket.
    pub fn write_protocol(&mut self, protocol: &Protocol) -> io::Result<()> {
        // Arrays are encoded by encoding each entry. All other protocol types are
        // considered literals. For now, rudis is not able to encode
        // recursive protocol structures. See below for more details.
        match protocol {
            Protocol::Array(val) => {
                // Encode the protocol type prefix. For an array, it is `*`.
                self.tcp_stream.write(&[b'*']).expect("TODO: panic message");

                // Encode the length of the array.
                self.write_decimal(val.len() as u64)?;

                // Iterate and encode each entry in the array.
                for entry in &**val {
                    self.write_value(entry)?;
                }
            }
            // The protocol type is a literal. Encode the value directly.
            _ => return self.write_value(protocol),
        }

        // Ensure the encoded protocol is written to the socket. The calls above
        // are to the buffered stream and writes. Calling `flush` writes the
        // remaining contents of the buffer to the socket.
        self.tcp_stream.flush()
    }

    /// Write a protocol literal to the stream
    fn write_value(&mut self, protocol: &Protocol) -> io::Result<()> {
        match protocol {
            Protocol::Simple(val) => {
                self.tcp_stream.write(&[b'+']).expect("TODO: panic message");
                self.tcp_stream.write_all(val.as_bytes())?;
                self.tcp_stream.write_all(b"\r\n")?;
            }
            Protocol::Error(val) => {
                self.tcp_stream.write(&[b'-']).expect("TODO: panic message");
                self.tcp_stream.write_all(val.as_bytes())?;
                self.tcp_stream.write_all(b"\r\n")?;
            }
            Protocol::Integer(val) => {
                self.tcp_stream.write(&[b':']).expect("TODO: panic message");
                self.write_decimal(*val)?;
            }
            Protocol::Null => {
                self.tcp_stream.write_all(b"$-1\r\n")?;
            }
            Protocol::Bulk(val) => {
                let len = val.len();

                self.tcp_stream.write(&[b'$']).expect("TODO: panic message");
                self.write_decimal(len as u64)?;
                self.tcp_stream.write_all(val)?;
                self.tcp_stream.write_all(b"\r\n")?;
            }
            // Encoding an `Array` from within a value cannot be done using a
            // recursive strategy. In general, async fns do not support
            // recursion. rudis has not needed to encode nested arrays yet,
            // so for now it is skipped.
            Protocol::Array(_val) => unreachable!(),
        }

        Ok(())
    }

    /// Write a decimal Protocol to the stream
    fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        // use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 20];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.tcp_stream.write_all(&buf.get_ref()[..pos])?;
        self.tcp_stream.write_all(b"\r\n")?;

        Ok(())
    }
}
