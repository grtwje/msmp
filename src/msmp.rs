//! msmp

#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
//#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::all)]
#![allow(clippy::doc_markdown)]

use std::collections::BTreeSet;
use std::fmt;

pub use elc_algorithm::ElcAlgorithm;
pub use error::{Error, Kind};
pub use word_list::WordList;

use one_d_packed_array::OneDPackedArray;
use two_d_array::{Row, TwoDArray, TwoDArraySizeIterator};

mod elc_algorithm;
mod error;
mod one_d_packed_array;
mod two_d_array;
mod word_list;

pub trait HashAlgorithm {
    fn h1(&self, word: &str) -> Result<usize, Error>;
    fn h2(&self, word: &str) -> Result<usize, Error>;
}

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
pub fn generate_hash(
    word_list: &WordList,
    hash_algorithm: impl HashAlgorithm + 'static,
) -> Result<HashData, Error> {
    match word_list.is_valid() {
        Ok(_) => {
            let two_d_array: TwoDArray = TwoDArray::new(word_list, &hash_algorithm)?;

            let one_d_packed_array: OneDPackedArray = OneDPackedArray::new(&two_d_array)?;

            //println!("{two_d_array:?}");

            //println!("{one_d_packed_array:?}");

            if verify(word_list, &one_d_packed_array, &hash_algorithm).is_err() {
                return Err(Error::new(Kind::HashError(
                    "Could not verify packed array.".to_string(),
                )));
            }

            Ok(HashData {
                as_string: String::from("test"),
                as_closure: HashClosure::new(move |a| {
                    hash(a, &one_d_packed_array, &hash_algorithm)
                }),
            })
        }
        Err(e) => Err(e),
    }
}

fn hash(word: &str, packed_array: &OneDPackedArray, hash_algorithm: &dyn HashAlgorithm) -> usize {
    let row_index = hash_algorithm.h1(word).unwrap_or(0);
    let col_index = hash_algorithm.h2(word).unwrap_or(0);
    let rlt_val = packed_array.get_rlt(row_index).unwrap_or(&0);
    let tmp = usize::try_from(rlt_val + isize::try_from(col_index).unwrap_or(0)).unwrap_or(0);
    tmp % packed_array.len()
}

fn verify(
    word_list: &WordList,
    packed_array: &OneDPackedArray,
    hash_algorithm: &dyn HashAlgorithm,
) -> Result<(), Error> {
    let w_it = word_list.list.iter();
    let mut hash_results = BTreeSet::new();
    for word in w_it {
        let hash_result = hash(word, packed_array, hash_algorithm);
        if hash_results.contains(&hash_result) {
            return Err(Error::new(Kind::HashError(
                "Collision detected while verifying the hash.".to_string(),
            )));
        }
        hash_results.insert(hash_result);
    }

    let h_it = hash_results.iter();
    for (i, hash_result) in h_it.enumerate() {
        if *hash_result != i {
            return Err(Error::new(Kind::HashError(
                "Unexpected gap found in index list.".to_string(),
            )));
        }
    }
    Ok(())
}
