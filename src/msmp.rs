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
pub use one_d_packed_array::OneDPackedArray;
pub use two_d_array::{TwoDArray, TwoDArraySizeIterator};
pub use word_list::WordList;

mod error;
mod one_d_packed_array;
mod two_d_array;
mod word_list;

pub struct HashClosure {
    pub cls: Box<dyn Fn(&str) -> usize>,
}

impl fmt::Debug for HashClosure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HashClosure")
    }
}

impl HashClosure {
    fn new(cls: impl Fn(&str) -> usize + 'static) -> Self {
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
pub fn generate_hash(word_list: &WordList) -> Result<HashData, Error> {
    match word_list.is_valid() {
        Ok(_) => {
            let two_d_array: TwoDArray = TwoDArray::new(word_list)?;

            let one_d_packed_array: OneDPackedArray = OneDPackedArray::new(&two_d_array)?;

            println!("{two_d_array:?}");

            println!("{one_d_packed_array:?}");

            //let mut it = TwoDArraySizeIterator::new(&two_d_array);
            //while let Some((index, row)) = it.next_biggest() {
            //    println!("{index:?}: {row:?}");
            //}

            let n = word_list.len();
            Ok(HashData {
                as_string: String::from("test"),
                as_closure: HashClosure::new(move |a| a.len() * n),
            })
        }
        Err(e) => Err(e),
    }
}
