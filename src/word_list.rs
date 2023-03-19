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
                    "Duplicate word detected: {} at line {}",
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
