use std::collections::BTreeSet;

#[derive(Debug)]
pub struct WordList {
    list: Vec<String>,
}

impl WordList {
    #[must_use]
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
    pub fn is_valid(&self) -> Result<(), String> {
        if self.list.is_empty() {
            return Err("Empty word list.".to_string());
        }

        let mut duplicate_checker = BTreeSet::new();
        for (i, word) in self.list.iter().enumerate() {
            if !word
                .chars()
                .all(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
            {
                return Err(format!(
                    "Non-ASCII upper case alphabetic word detected at {}.",
                    i + 1
                ));
            }

            if !duplicate_checker.insert(word) {
                return Err(format!(
                    "Duplicate word detected: {} at position {}",
                    word,
                    i + 1
                ));
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.list.len()
    }

    #[must_use]
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
        let mut wl = WordList::new();
        assert!(wl.is_empty());

        wl.push("HELLO");
        assert!(wl.len() == 1);
        assert!(!wl.is_empty());
        if let Err(e) = wl.is_valid() {
            panic!("test failed. invalid at {e}");
        }

        wl.push("HELLO");
        assert!(wl.len() == 2);
        assert!(!wl.is_empty());
        if let Ok(()) = wl.is_valid() {
            panic!("duplicate not detected");
        }

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
