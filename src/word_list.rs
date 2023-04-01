use crate::{Error, Kind};
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct WordList {
    pub list: Vec<String>,
}

impl WordList {
    pub fn new() -> Self {
        WordList { list: Vec::new() }
    }

    /// Tests whether all the words in the list are valid for the assumptions
    /// and limitations of the hashing implementation.
    ///
    /// # Errors
    ///
    /// Will return Err if words are not:
    /// * ASCII uppercase alphabetic
    /// * unique
    pub fn is_valid(&self) -> Result<(), Error> {
        if self.list.is_empty() {
            return Err(Error::new(Kind::WordListError(
                "Empty word list.".to_string(),
            )));
        }

        let mut duplicate_checker = BTreeSet::new();
        for (i, word) in self.list.iter().enumerate() {
            if !word
                .chars()
                .all(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
            {
                return Err(Error::new(Kind::WordListError(format!(
                    "Non ASCII upper case alphabetic word detected at {}.",
                    i + 1
                ))));
            }

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

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn push(&mut self, word: &str) {
        self.list.push(word.to_string());
    }
}

impl Default for WordList {
    fn default() -> Self {
        WordList::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_list_unit_test() {
        let mut wl: WordList = Default::default();
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
        println!("{:?}", wl);

        let mut wl2 = WordList::new();
        assert!(wl2.is_empty());

        wl2.push("hELLO");
        assert!(wl2.len() == 1);
        assert!(!wl2.is_empty());
        if let Ok(()) = wl2.is_valid() {
            panic!("lower case not detected");
        }
    }
}
