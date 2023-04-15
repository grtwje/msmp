//! msmp
//! ====
//! A library for generating a hash function from a word list.

#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::all)]
#![allow(clippy::doc_markdown)]

use std::collections::BTreeSet;
use std::fmt;

pub use elc_algorithm::ElcAlgorithm;
pub use error::{Error, Kind};
pub use word_list::WordList;

use one_d_packed_array::OneDPackedArray;
use rlt::Rlt;
use two_d_array::{Row, RowSizeIterator, TwoDArray};

mod elc_algorithm;
mod error;
mod one_d_packed_array;
mod rlt;
mod two_d_array;
mod word_list;

/// A trait for a hash algorithm.
pub trait HashAlgorithm {
    /// Hash function h1 that is used internally to generate row indices.
    ///
    /// # Parameters
    /// * `word` - A word to be hashed.
    ///
    /// # Returns
    /// A hash value.
    ///
    /// # Errors
    /// Will return `Err` if `word` is not a valid word.
    fn h1(&self, word: &str) -> Result<usize, Error>;

    /// Hash function h2 that is used internally to generate column indices.
    ///
    /// # Parameters
    /// * `word` - A word to be hashed.
    ///
    /// # Returns
    /// A hash value.
    ///
    /// # Errors
    /// Will return `Err` if `word` is not a valid word.
    fn h2(&self, word: &str) -> Result<usize, Error>;

    /// A representation of the hash function h1 as a string of pseudo code.
    ///
    /// # Returns
    /// A string representation of the h1 hash function.
    fn h1_as_text(&self) -> String;

    /// A representation of the hash function h2 as a string of pseudo code.
    ///
    /// # Returns
    /// A string representation of the h2 hash function.
    fn h2_as_text(&self) -> String;
}

///  A closure that takes a word and returns a hash value.
pub struct HashClosure {
    /// A closure that takes a word and returns a hash value.
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

/// A struct containing a string representation of the hash function and a
/// closure that takes a word and returns a hash value.
#[derive(Debug)]
pub struct HashData {
    /// A string representation of the hash function.
    /// This is a pseudo code representation of the msmp hash function. It can be
    /// used to help implement the msmp hash function in other languages.
    pub as_string: String,

    /// A closure that takes a word and returns a hash value.
    pub as_closure: HashClosure,
}

/// Generates a msmp hash function from a word list.
///
/// # Parameters
/// * `word_list` - A word list.
/// * `hash_algorithm` - A hash algorithm.
///
/// # Returns
/// A struct containing a string representation of the hash function and a
/// closure that takes a word and returns a hash value.
///
/// # Errors
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

            verify(word_list, one_d_packed_array.get_rlt(), &hash_algorithm)?;

            Ok(HashData {
                as_string: text(one_d_packed_array.get_rlt(), &hash_algorithm),
                as_closure: HashClosure::new(move |a| {
                    hash(a, one_d_packed_array.get_rlt(), &hash_algorithm)
                }),
            })
        }
        Err(e) => Err(e),
    }
}

/// Implements the closure returned to the generate_hash caller.
///
/// # Parameters
/// * `word` - A word to be hashed.
/// * `rlt` - A row lookup table.
/// * `hash_algorithm` - A hash algorithm.
///
/// # Returns
/// A hash value.
fn hash(word: &str, rlt: &Rlt, hash_algorithm: &dyn HashAlgorithm) -> usize {
    let row_index = hash_algorithm.h1(word).unwrap_or(0);
    let col_index = hash_algorithm.h2(word).unwrap_or(0);
    let rlt_val = rlt.get(row_index).unwrap_or(&0);
    let tmp = usize::try_from(rlt_val + isize::try_from(col_index).unwrap_or(0)).unwrap_or(0);
    tmp % rlt.get_num_entries()
}

/// Generates a string representation of the hash function.
///
/// The string representation is a pseudo code representation of the msmp hash
/// function. It can be used to help implement the msmp hash function in other
/// languages.
///
/// # Parameters
/// * `rlt` - A row lookup table.
/// * `hash_algorithm` - A hash algorithm.
///
/// # Returns
/// A string representation of the hash function.
fn text(rlt: &Rlt, hash_algorithm: &dyn HashAlgorithm) -> String {
    let rv = format!(
        "row_lookup_table = [{rlt}]\n\
         row_index = {h1}\n\
         col_index = {h2}\n\
         hash_value = (row_lookup_table[row_index] + col_index) % {len}\n",
        rlt = rlt.get_as_text(),
        h1 = hash_algorithm.h1_as_text(),
        h2 = hash_algorithm.h2_as_text(),
        len = rlt.get_num_entries()
    );
    rv
}

/// Verifies that the hash function is valid for the given word list.
///
/// This is done by generating a hash value for each word in the word list
/// and checking that there are no collisions and that the hash values are
/// in the range [0, len(word_list)-1).
/// This function is used to verify that the hash function generated by
/// generate_hash is valid.
/// This function is not intended to be called directly by the caller.
/// It is called by generate_hash.
///
/// # Parameters
/// * `word_list` - A word list.
/// * `rlt` - A row lookup table.
/// * `hash_algorithm` - A hash algorithm.
///
/// # Returns
/// `Ok(())` if the hash function is valid.
///
/// # Errors
/// Will return `Err` if a collision is detected or if the hash values are
/// not in the range [0, len(word_list)).
fn verify(
    word_list: &WordList,
    rlt: &Rlt,
    hash_algorithm: &dyn HashAlgorithm,
) -> Result<(), Error> {
    let w_it = word_list.list.iter();
    let mut hash_results = BTreeSet::new();
    for word in w_it {
        let hash_result = hash(word, rlt, hash_algorithm);
        println!("{word} -> {hash_result}");
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
        if *hash_result >= word_list.list.len() {
            return Err(Error::new(Kind::HashError(
                "Hash value is out of range.".to_string(),
            )));
        }
    }
    Ok(())
}
