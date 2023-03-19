//! msmp

#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
//#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::doc_markdown)]

pub use error::{Error, Kind};
pub use word_list::WordList;

mod error;
mod word_list;

/// # Errors
///
/// Will return `Err` if `word_list` fails to resolve to a hash function.
pub fn generate_hash(word_list: &WordList) -> Result<String, Error> {
    println!("{word_list:?}");
    Ok(String::from("test"))
}
