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

    /// An error returned while creating the word list.
    WordListError(String),

    /// An error returned while creating the 2D array.
    TwoDArrayError(String),

    /// An error returned while creating the 1D packed array.
    OneDPackedArrayError(String),

    /// An error returned while creating the ELC algorithm.
    ElcAlgorithmError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::HashError(s) => write!(f, "Hash Error: {s}"),
            Kind::WordListError(s) => write!(f, "Word List Error: {s}"),
            Kind::TwoDArrayError(s) => write!(f, "2D Array Error: {s}"),
            Kind::OneDPackedArrayError(s) => write!(f, "1D Packed Array Error: {s}"),
            Kind::ElcAlgorithmError(s) => write!(f, "ELC Algorithm Error: {s}"),
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
        println!("{e}, {e:?}");

        let e: Error = Error::new(Kind::WordListError(String::from("idk")));
        match e.kind() {
            Kind::WordListError(s) => assert!(s == "idk"),
            _ => panic!("Unexpected Kind: {e}"),
        }
        println!("{e}");

        let e: Error = Error::new(Kind::TwoDArrayError(String::from("idk")));
        match e.kind() {
            Kind::TwoDArrayError(s) => assert!(s == "idk"),
            _ => panic!("Unexpected Kind: {e}"),
        }
        println!("{e}");

        let e: Error = Error::new(Kind::OneDPackedArrayError(String::from("idk")));
        match e.kind() {
            Kind::OneDPackedArrayError(s) => assert!(s == "idk"),
            _ => panic!("Unexpected Kind: {e}"),
        }
        println!("{e}");

        let e: Error = Error::new(Kind::ElcAlgorithmError(String::from("idk")));
        match e.kind() {
            Kind::ElcAlgorithmError(s) => assert!(s == "idk"),
            _ => panic!("Unexpected Kind: {e}"),
        }
        println!("{e}");
    }
}
