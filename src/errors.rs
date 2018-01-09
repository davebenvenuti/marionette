use std::error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidHostError;

impl fmt::Display for InvalidHostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid host string, should be of the form host:port")
    }
}

impl error::Error for InvalidHostError {
    fn description(&self) -> &str {
        "invalid host string, should be of the form host:port"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub struct AuthenticationError {
    pub host: String
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to authenticate with ssh-agent for {}", self.host)
    }
}

impl error::Error for AuthenticationError {
    fn description(&self) -> &str {
        "unable to authenticate with ssh-agent"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
