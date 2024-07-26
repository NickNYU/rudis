use resp::{self, Result, protocol::Protocol, parse::Parser};
use crate::client::Client;
use crate::command::{ping::Ping, unknown::Unknown};

pub(crate) mod ping;
pub(crate) mod set;
pub(crate) mod unknown;

#[derive(Debug)]
pub(crate) enum Command {
    Ping(Ping),
    // Get,
    // Set
    Unknown(Unknown),
}

impl Command {
    /// Parse a command from a received protocol.
    ///
    /// The `Protocol` must represent a Redis command supported by `rudis` and
    /// be the array variant.
    ///
    /// # Returns
    ///
    /// On success, the command value is returned, otherwise, `Err` is returned.
    pub fn from_frame(protocol: Protocol) -> Result<Command> {
        // The protocol value is decorated with `Parse`. `Parse` provides a
        // "cursor" like API which makes parsing the command easier.
        //
        // The protocol value must be an array variant. Any other protocol variants
        // result in an error being returned.
        let mut parse = Parser::new(protocol)?;

        // All redis commands begin with the command name as a string. The name
        // is read and converted to lower cases in order to do case sensitive
        // matching.
        let command_name = parse.next_string()?.to_lowercase();

        // Match the command name, delegating the rest of the parsing to the
        // specific command.
        let command = match &command_name[..] {
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            _ => {
                // The command is not recognized and an Unknown command is
                // returned.
                //
                // `return` is called here to skip the `finish()` call below. As
                // the command is not recognized, there is most likely
                // unconsumed fields remaining in the `Parse` instance.
                return Ok(Command::Unknown(Unknown::new(command_name)));
            }
        };

        // Check if there is any remaining unconsumed fields in the `Parse`
        // value. If fields remain, this indicates an unexpected protocol format
        // and an error is returned.
        parse.finish()?;

        // The command has been successfully parsed
        Ok(command)
    }

    /// Apply the command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    pub(crate) fn apply(self, client: &mut Client) -> Result<()> {
        use Command::*;

        match self {
            Ping(cmd) => cmd.apply(client.connection),
            Unknown(cmd) => cmd.apply(client.connection),
        }
    }

    /// Returns the command name
    pub(crate) fn get_name(&self) -> &str {
        match self {
            Command::Ping(_) => "ping",
            Command::Unknown(cmd) => cmd.get_name(),
        }
    }
}