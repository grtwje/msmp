use crate::{Error, Kind, WordList};
use std::collections::BTreeMap;
use std::iter::zip;

#[derive(Debug)]
pub struct TwoDArray {
    rows: BTreeMap<usize, Row>,
}

#[derive(Debug)]
pub struct Row {
    cols: BTreeMap<usize, usize>,
}

impl Row {
    fn get_num_cols(&self) -> usize {
        self.cols.len()
    }
}

impl TwoDArray {
    pub fn new(word_list: &WordList) -> Result<Self, Error> {
        let mut _self = TwoDArray {
            rows: BTreeMap::new(),
        };

        // Calculate the indices that will be used in the 2D array.
        let mut row_indices = Vec::new();
        let mut col_indices = Vec::new();
        for word in word_list.list.iter() {
            let row = h1(word)?;
            row_indices.push(row);

            let col = h2(word)?;
            col_indices.push(col);
        }

        // Build the 2D array.
        let it = zip(row_indices, col_indices);
        for (i, (r, c)) in it.enumerate() {
            // Get the row to add to or create a new row if needed.
            let row = _self.rows.entry(r).or_insert_with(|| Row {
                cols: BTreeMap::new(),
            });

            let current_idx = i + 1; // word list is 1 based
            if let Some(prior) = row.cols.insert(c, current_idx) {
                return Err(Error::new(Kind::TwoDArrayError(format!(
                    "Collision: {} === {}",
                    word_list.list[prior - 1],
                    word_list.list[current_idx - 1]
                ))));
            }
        }

        Ok(_self)
    }

    pub fn get_sorted_row_list(&self) -> BTreeMap<usize, Row> {
        let mut list = BTreeMap::new();

        for row in self.rows.iter() {
            println!("{row:?}")
            //list.insert(row.get_num_cols(), );
        }

        list
    }
}

fn h1(word: &str) -> Result<usize, Error> {
    if let Some(first_char) = word.chars().next() {
        char_to_index(first_char)
    } else {
        Err(Error::new(Kind::TwoDArrayError(format!(
            "Could not find first letter of ({word})."
        ))))
    }
}

fn h2(word: &str) -> Result<usize, Error> {
    if let Some(last_char) = word.chars().last() {
        char_to_index(last_char)
    } else {
        Err(Error::new(Kind::TwoDArrayError(format!(
            "Could not find last letter of ({word})."
        ))))
    }
}

fn char_to_index(c: char) -> Result<usize, Error> {
    if c.is_ascii_uppercase() {
        Ok((c as usize) - ('A' as usize))
    } else {
        Err(Error::new(Kind::TwoDArrayError(format!(
            "Unexpected character encountered ({c})"
        ))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_d_array_unit_test() {
        let mut word_list = WordList::new();
        word_list.push("WORD");

        if let Err(e) = TwoDArray::new(&word_list) {
            panic!("Unexpected 2D array creation failure. {e}")
        }

        word_list.push("WIRED");
        if let Ok(_a) = TwoDArray::new(&word_list) {
            panic!("Undetected collision.");
        }
    }
}
