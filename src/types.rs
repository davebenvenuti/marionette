use std::num;

use errors;
use errors::InvalidHostError;

use ssh2::{Session, Agent};

#[derive(Debug)]
pub struct Server {
    pub host: String,
    pub port: u32
}

fn parse_port(port: &str) -> Result<u32, num::ParseIntError> {
    port.parse::<u32>()
}

impl Server {
    pub fn from_str(host_string: &str) -> Result<Server, InvalidHostError> {
        let host_string_parts: Vec<&str> = host_string.split(":").collect::<Vec<&str>>();

        let host = host_string_parts.get(0);
        let port = host_string_parts.get(1);

        match (host, port) {
            (Some(ref h), Some(ref p)) => {
                match parse_port(p) {
                    Ok(parsed_p) => {
                        Ok(Server { host: h.to_string(), port: parsed_p })
                    },
                    Err(_e) => {
                        Err(InvalidHostError)
                    }
                }
            }
            _ => Err(InvalidHostError)
        }
    }
}
