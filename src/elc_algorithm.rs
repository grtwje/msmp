use crate::{Error, HashAlgorithm, Kind};

/// The ElcAlgorithm.
#[derive(Debug)]
pub struct ElcAlgorithm {
    elc: usize,
    num_vals: usize,
}

impl ElcAlgorithm {
    /// Creates a new ElcAlgorithm.
    #[must_use]
    pub fn new(elc: usize, num_vals: usize) -> Self {
        Self { elc, num_vals }
    }

    fn char_to_index(c: char) -> usize {
        assert!(c.is_ascii_uppercase());
        (c as usize) - ('A' as usize)
    }

    fn step(&self, acc: usize, x: usize) -> usize {
        (acc * self.num_vals) + x
    }
}

impl HashAlgorithm for ElcAlgorithm {
    fn h1(&self, word: &str) -> Result<usize, Error> {
        if word.len() < self.elc {
            return Err(Error::new(Kind::ElcAlgorithmError(format!(
                "Expected word length ({word_len}) to be greater than or equal to elc ({elc}).",
                word_len = word.len(),
                elc = self.elc
            ))));
        }

        if word.chars().take(self.elc).all(|c| c.is_ascii_uppercase()) {
            let x: usize = word
                .chars()
                .take(self.elc)
                .map(ElcAlgorithm::char_to_index)
                .fold(0, |acc, x| self.step(acc, x));
            Ok(x)
        } else {
            Err(Error::new(Kind::ElcAlgorithmError(format!(
                "Unexpected character(s) encountered ({word}) in indices (0..{elc}).",
                word = word,
                elc = self.elc
            ))))
        }
    }

    fn h2(&self, word: &str) -> Result<usize, Error> {
        if word.len() < self.elc {
            return Err(Error::new(Kind::ElcAlgorithmError(format!(
                "Expected word length ({word_len}) to be greater than or equal to elc ({elc}).",
                word_len = word.len(),
                elc = self.elc
            ))));
        }

        if word
            .chars()
            .rev()
            .take(self.elc)
            .all(|c| c.is_ascii_uppercase())
        {
            let x: usize = word
                .chars()
                .rev()
                .take(self.elc)
                .map(ElcAlgorithm::char_to_index)
                .fold(0, |acc, x| self.step(acc, x));
            Ok(x)
        } else {
            Err(Error::new(Kind::ElcAlgorithmError(format!(
                "Unexpected character(s) encountered ({word}) in indices (0..{elc}).",
                word = word,
                elc = self.elc
            ))))
        }
    }
}

impl Default for ElcAlgorithm {
    fn default() -> Self {
        Self {
            elc: 1,
            num_vals: 26,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elc_algorithm_unit_test() {
        assert_eq!(ElcAlgorithm::char_to_index('A'), 0);
        assert_eq!(ElcAlgorithm::char_to_index('B'), 1);
        assert_eq!(ElcAlgorithm::char_to_index('Z'), 25);

        // ----
        let hash_algorithm: ElcAlgorithm = ElcAlgorithm::default();

        assert_eq!(hash_algorithm.h1("A").unwrap(), 0);
        assert_eq!(hash_algorithm.h1("B").unwrap(), 1);
        assert_eq!(hash_algorithm.h1("Z").unwrap(), 25);
        assert_eq!(hash_algorithm.h1("AA").unwrap(), 0);

        assert_eq!(hash_algorithm.h2("A").unwrap(), 0);
        assert_eq!(hash_algorithm.h2("B").unwrap(), 1);
        assert_eq!(hash_algorithm.h2("Z").unwrap(), 25);
        assert_eq!(hash_algorithm.h2("AA").unwrap(), 0);
        assert_eq!(hash_algorithm.h2("AB").unwrap(), 1);
        assert_eq!(hash_algorithm.h2("AZ").unwrap(), 25);
        assert_eq!(hash_algorithm.h2("BA").unwrap(), 0);

        assert_eq!(hash_algorithm.step(0, 0), 0);
        assert_eq!(hash_algorithm.step(0, 1), 1);
        assert_eq!(hash_algorithm.step(1, 0), 26);
        assert_eq!(hash_algorithm.step(1, 1), 27);

        // ----
        let hash_algorithm: ElcAlgorithm = ElcAlgorithm::new(2, 26);

        assert_eq!(hash_algorithm.h1("AA").unwrap(), 0);
        assert_eq!(hash_algorithm.h1("AB").unwrap(), 1);
        assert_eq!(hash_algorithm.h1("AZ").unwrap(), 25);
        assert_eq!(hash_algorithm.h1("BA").unwrap(), 26);
        assert_eq!(hash_algorithm.h1("BB").unwrap(), 27);

        assert_eq!(hash_algorithm.h2("AA").unwrap(), 0);
        assert_eq!(hash_algorithm.h2("AB").unwrap(), 26);
        assert_eq!(hash_algorithm.h2("AZ").unwrap(), 650);
        assert_eq!(hash_algorithm.h2("BA").unwrap(), 1);
        assert_eq!(hash_algorithm.h2("BB").unwrap(), 27);
        assert_eq!(hash_algorithm.h2("BZ").unwrap(), 651);
        assert_eq!(hash_algorithm.h2("CA").unwrap(), 2);
        assert_eq!(hash_algorithm.h2("ZZ").unwrap(), 675);

        assert_eq!(hash_algorithm.step(0, 0), 0);
        assert_eq!(hash_algorithm.step(0, 1), 1);
        assert_eq!(hash_algorithm.step(1, 0), 26);
        assert_eq!(hash_algorithm.step(1, 1), 27);

        match hash_algorithm.h1("A").unwrap_err().kind() {
            Kind::ElcAlgorithmError(msg) => {
                assert_eq!(
                    msg,
                    "Expected word length (1) to be greater than or equal to elc (2)."
                );
            }
            _ => panic!("Unexpected error type."),
        };

        match hash_algorithm.h1("aA").unwrap_err().kind() {
            Kind::ElcAlgorithmError(msg) => {
                assert_eq!(
                    msg,
                    "Unexpected character(s) encountered (aA) in indices (0..2)."
                );
            }
            _ => panic!("Unexpected error type."),
        };

        match hash_algorithm.h2("A").unwrap_err().kind() {
            Kind::ElcAlgorithmError(msg) => {
                assert_eq!(
                    msg,
                    "Expected word length (1) to be greater than or equal to elc (2)."
                );
            }
            _ => panic!("Unexpected error type."),
        };

        match hash_algorithm.h2("aA").unwrap_err().kind() {
            Kind::ElcAlgorithmError(msg) => {
                assert_eq!(
                    msg,
                    "Unexpected character(s) encountered (aA) in indices (0..2)."
                );
            }
            _ => panic!("Unexpected error type."),
        };

        println!("{hash_algorithm:?}");
    }
}
