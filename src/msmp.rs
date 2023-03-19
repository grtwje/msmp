//! msmp

#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
//#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::all)]
#![allow(clippy::doc_markdown)]

use std::fmt;

pub use error::{Error, Kind};
pub use word_list::WordList;

mod error;
mod word_list;

pub struct HashClosure {
    pub cls: Box<dyn Fn(&str) -> i16>,
}

impl fmt::Debug for HashClosure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HashClosure")
    }
}

impl HashClosure {
    fn new(cls: impl Fn(&str) -> i16 + 'static) -> Self {
        Self { cls: Box::new(cls) }
    }
}

#[derive(Debug)]
pub struct HashData {
    pub as_string: String,
    pub as_closure: HashClosure,
}

/// # Errors
///
/// Will return `Err` if `word_list` fails to resolve to a hash function.
pub fn generate_hash(_word_list: &WordList) -> Result<HashData, Error> {
    //println!("{word_list:?}");
    Ok(HashData {
        as_string: String::from("test"),
        as_closure: HashClosure::new(|a| a.len().try_into().unwrap()),
    })
}
