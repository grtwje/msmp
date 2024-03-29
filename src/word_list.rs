use crate::{Error, Kind};
use std::collections::BTreeSet;

/// A list of words that need to be accessed by a hash function.
#[derive(Debug)]
pub struct WordList {
    /// The list of words.
    pub list: Vec<String>,
}

impl WordList {
    /// Creates a new empty word list.
    #[must_use]
    pub fn new() -> Self {
        WordList { list: Vec::new() }
    }

    /// Tests whether all the words in the list are valid for the assumptions
    /// and limitations of the hashing implementation.
    /// * All words are ASCII uppercase alphabetic.
    /// * All words are unique.
    /// * The list is not empty.
    ///
    /// # Returns
    /// * Ok(()) if the list is valid.
    /// * Err(Error) if the list is invalid.
    ///
    /// # Errors
    /// Will return Err if words are not:
    /// * ASCII uppercase alphabetic
    /// * unique
    /// * the list is empty
    pub fn is_valid(&self) -> Result<(), Error> {
        // Check for empty list.
        if self.list.is_empty() {
            return Err(Error::new(Kind::WordListError(
                "Empty word list.".to_string(),
            )));
        }

        let mut duplicate_checker = BTreeSet::new();
        for (i, word) in self.list.iter().enumerate() {
            // Check for non ASCII upper case alphabetic words.
            if !word
                .chars()
                .all(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
            {
                return Err(Error::new(Kind::WordListError(format!(
                    "Non ASCII upper case alphabetic word detected at {}.",
                    i + 1
                ))));
            }

            // Check for duplicate words.
            if !duplicate_checker.insert(word) {
                return Err(Error::new(Kind::WordListError(format!(
                    "Duplicate word detected: {} at position {}",
                    word,
                    i + 1
                ))));
            }
        }

        Ok(())
    }

    /// Returns the number of words in the list.
    ///
    /// # Returns
    /// The number of words in the list.
    #[must_use]
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns true if the list is empty.
    ///
    /// # Returns
    /// True if the list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Adds a word to the list.
    ///
    /// # Parameters
    /// * `word` - A word to be added to the list.
    pub fn push(&mut self, word: &str) {
        self.list.push(word.to_string());
    }
}

impl Default for WordList {
    fn default() -> Self {
        WordList::new()
    }
}

impl FromIterator<String> for WordList {
    /// Creates a new word list from an iterator of strings.
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut wl = WordList::new();
        for word in iter {
            wl.push(&word);
        }
        wl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_list_unit_test() {
        let mut wl: WordList = WordList::default();
        assert!(wl.is_empty());
        if let Ok(()) = wl.is_valid() {
            panic!("Empty not detected.")
        }

        wl.push("HELLO");
        assert!(wl.len() == 1);
        assert!(!wl.is_empty());
        if let Err(e) = wl.is_valid() {
            panic!("Unexpected invalid. {e}");
        }

        wl.push("HELLO");
        assert!(wl.len() == 2);
        assert!(!wl.is_empty());
        if let Ok(()) = wl.is_valid() {
            panic!("Duplicate not detected.");
        }
        println!("{wl:?}");

        let mut wl2 = WordList::new();
        assert!(wl2.is_empty());

        wl2.push("hELLO");
        assert!(wl2.len() == 1);
        assert!(!wl2.is_empty());
        if let Ok(()) = wl2.is_valid() {
            panic!("lower case not detected");
        }

        let wl3: WordList = ["HELLO", "WORLD", "TEST"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        assert!(wl3.len() == 3);
    }
}
