use resp::{self, Result, protocol::Protocol, parse::Parser, parse::ParseError};
use bytes::Bytes;
use crate::connection::Connection;

/// Returns PONG if no argument is provided, otherwise
/// return a copy of the argument as a bulk.
///
/// This command is often used to test if a connection
/// is still alive, or to measure latency.
#[derive(Debug, Default)]
pub struct Ping {
    /// optional message to be returned
    msg: Option<Bytes>,
}

impl Ping {
    /// Create a new `Ping` command with optional `msg`.
    pub fn new(msg: Option<Bytes>) -> Ping {
        Ping { msg }
    }

    /// Parse a `Ping` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Protocol`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `PING` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `Ping` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing `PING` and an optional message.
    ///
    /// ```text
    /// PING [message]
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parser) -> Result<Ping> {
        match parse.next_bytes() {
            Ok(msg) => Ok(Ping::new(Some(msg))),
            Err(ParseError::EndOfStream) => Ok(Ping::default()),
            Err(e) => Err(e.into()),
        }
    }

    /// Apply the `Ping` command and return the message.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    // #[instrument(skip(self, dst))]
    pub(crate) fn apply(self, dst: &mut Connection) -> Result<()> {
        let response = match self.msg {
            None => Protocol::Simple("PONG".to_string()),
            Some(msg) => Protocol::Bulk(msg),
        };

        // debug!(?response);

        // Write the response back to the client
        dst.write_protocol(&response)?;

        Ok(())
    }

    /// Converts the command into an equivalent `Protocol`.
    ///
    /// This is called by the client when encoding a `Ping` command to send
    /// to the server.
    pub(crate) fn into_frame(self) -> Protocol {
        let mut frame = Protocol::array();
        frame.push_bulk(Bytes::from("ping".as_bytes()));
        if let Some(msg) = self.msg {
            frame.push_bulk(msg);
        }
        frame
    }
}
