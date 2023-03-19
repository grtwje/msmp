use std::error;
use std::fmt;

/// An error that can occur in this library.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Error {
        Error { kind }
    }

    /// Convenience function for getting the kind of error.
    #[must_use]
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

/// The different kinds of errors that can occur.
#[derive(Debug)]
///#[non_exhaustive]
pub enum Kind {
    /// An error returned while creating the hash.
    HashError(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::HashError(_) => "Hash error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::HashError(s) => write!(f, "Hash Error: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_unit_test() {
        let e: Error = Error::new(Kind::HashError(String::from("idk")));
        match e.kind() {
            Kind::HashError(s) => assert!(s == "idk"),
        }
    }
}
