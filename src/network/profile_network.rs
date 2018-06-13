use network::providers::{Machine};
use network::Network;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub(crate) struct ProfileNetwork {
    handler: Box<Network>,
}

/// Profile Network handler responsible to connect to a wireless network.
impl ProfileNetwork {
    pub(crate) fn new(name: &str) -> Result<Self, Error> {
        if !(cfg!(target_os = "linux") || cfg!(target_os = "windows")) {
            return Err(Error::new(
                ErrorKind::Other,
                "The Specified OS is not supported",
            ));
        }

        let handler = Machine::new(name)?;
        return Ok(ProfileNetwork {
            handler: Box::new(handler),
        });
    }

    pub(crate) fn connect(&self, password: &str) -> bool {
        self.handler.connect(password)
    }
}
