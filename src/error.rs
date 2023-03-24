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
pub enum Kind {
    /// An error returned while creating the hash.
    HashError(String),
    WordListError(String),
    TwoDArrayError(String),
    OneDPackedArrayError(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::HashError(_) => "Hash error",
            Kind::WordListError(_) => "Word list error",
            Kind::TwoDArrayError(_) => "2D Array error",
            Kind::OneDPackedArrayError(_) => "1D Packed Array error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::HashError(s) => write!(f, "Hash Error: {s}"),
            Kind::WordListError(s) => write!(f, "Word List Error: {s}"),
            Kind::TwoDArrayError(s) => write!(f, "2D Array Error: {s}"),
            Kind::OneDPackedArrayError(s) => write!(f, "1D Packed Array Error: {s}"),
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
            _ => panic!("Unexpected Kind: {e}"),
        }

        let e: Error = Error::new(Kind::WordListError(String::from("idk")));
        match e.kind() {
            Kind::WordListError(s) => assert!(s == "idk"),
            _ => panic!("Unexpected Kind: {e}"),
        }

        let e: Error = Error::new(Kind::TwoDArrayError(String::from("idk")));
        match e.kind() {
            Kind::TwoDArrayError(s) => assert!(s == "idk"),
            _ => panic!("Unexpected Kind: {e}"),
        }

        let e: Error = Error::new(Kind::OneDPackedArrayError(String::from("idk")));
        match e.kind() {
            Kind::OneDPackedArrayError(s) => assert!(s == "idk"),
            _ => panic!("Unexpected Kind: {e}"),
        }
    }
}
