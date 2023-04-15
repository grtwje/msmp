use crate::{Error, HashAlgorithm, Kind, WordList};
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::iter::zip;

/// A 2D array that is used to store the indices of the words in the word list.
/// The array is stored as a BTreeMap of rows. Each row is a BTreeMap of
/// column indices.
#[derive(Debug)]
pub struct TwoDArray {
    // The rows of the 2D array.
    // The key is the row index and the value is the row.
    rows: BTreeMap<usize, Row>, // row index, row

    // The row indices sorted by the number of entries in the row.
    rows_by_size: Vec<usize>,

    // The number of entries in the array.
    num_entries: usize,

    // The number of rows in the array.
    num_rows: usize,

    // The index of the last row in the array.
    last_row_index: usize,
}

/// A row in the 2D array.
#[derive(Debug)]
pub struct Row {
    // The columns of the row.
    // The key is the column index and the value is the index of the word in the
    // word list.
    cols: BTreeMap<usize, usize>, // col index, word index
}

impl<'a> TwoDArray {
    /// Create a new 2D array.
    /// The array is created by hashing each word in the word list and using the
    /// hash values as the row and column indices.
    /// The array is then sorted by the number of entries in each row.
    ///
    /// # Parameters
    /// * `word_list` - The word list to create the array from.
    /// * `hash_algorithm` - The hash algorithm to use.
    ///
    /// # Returns
    /// A new 2D array.
    ///
    /// # Errors
    /// Will return `Err` if there is a collision in the array.
    /// A collision occurs when two words hash to the same row and column.
    /// This will happen if the hash algorithm is not good enough. A new algorithm
    /// will need to be tried.
    pub fn new(word_list: &WordList, hash_algorithm: &dyn HashAlgorithm) -> Result<Self, Error> {
        let num_words = word_list.len();

        let mut self_ = TwoDArray {
            rows: BTreeMap::new(),
            rows_by_size: Vec::new(),
            num_entries: num_words,
            num_rows: 0,
            last_row_index: 0,
        };

        {
            // Calculate the indices that will be used in the 2D array.
            let mut row_indices = Vec::with_capacity(num_words);
            let mut col_indices = Vec::with_capacity(num_words);
            for word in &word_list.list {
                let row = hash_algorithm.h1(word)?;
                row_indices.push(row);

                let col = hash_algorithm.h2(word)?;
                col_indices.push(col);
            }

            // * Fill the 2-D array with values.
            let it = zip(row_indices, col_indices);
            for (i, (r, c)) in it.enumerate() {
                // Get the row to add to or create a new row if needed.
                let row = self_.rows.entry(r).or_insert_with(|| Row {
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
        }

        // * Sort the rows of the array.
        // Build secondary index, rows sorted by size.
        let mut rows_by_size: Vec<(usize, usize)> = Vec::new();
        for (i, r) in &self_.rows {
            rows_by_size.push((r.cols.len(), *i));
        }
        rows_by_size.sort_by_key(|k| (Reverse(k.0)));
        self_.rows_by_size = rows_by_size.iter().map(|a| a.1).collect::<Vec<usize>>();

        self_.num_rows = self_.rows.len();

        self_.last_row_index = *self_.rows.last_key_value().unwrap().0;

        Ok(self_)
    }

    // Get the number of entries in the array.
    //
    // # Returns
    // The number of entries in the array.
    pub fn get_num_entries(&self) -> usize {
        self.num_entries
    }

    // Get the index of the last row in the array.
    //
    // # Returns
    // The index of the last row in the array.
    pub fn get_last_row_index(&self) -> usize {
        self.last_row_index
    }

    // Get the number of rows in the array.
    // Only used for testing.
    //
    // # Returns
    // The number of rows in the array.
    #[cfg(test)]
    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    // Get a row of the array by its size.
    //
    // Index 0 is the row with the most entries.
    // Index 1 is the row with the second most entries.
    // etc.
    //
    // # Parameters
    // * `index` - The index of the row to get.
    //
    // # Returns
    // The row at the given index.
    fn get_row_by_size(&'a self, index: usize) -> Option<(usize, &'a Row)> {
        if let Some(size_index) = self.rows_by_size.get(index) {
            if let Some(row) = self.rows.get(size_index) {
                return Some((*size_index, row));
            }
        }
        None
    }
}

impl Row {
    // Get the used column indices in the row.
    //
    // # Returns
    // The used column indices in the row.
    pub fn get_col_indices(&self) -> Vec<usize> {
        let indices: Vec<_> = self.cols.keys().copied().collect();
        indices
    }

    // Get the column values in the row.
    //
    // # Returns
    // The column values in the row.
    pub fn get_col_values(&self) -> Vec<usize> {
        let values: Vec<_> = self.cols.values().copied().collect();
        values
    }
}

// Iterator for the rows of a 2D array sorted by size.
#[derive(Debug)]
pub struct RowSizeIterator<'a> {
    // The 2D array to iterate over.
    two_d_array: &'a TwoDArray,

    // The current index of the iterator.
    index: usize,
}

impl<'a> RowSizeIterator<'a> {
    // Create a new iterator.
    pub fn new(array: &'a TwoDArray) -> Self {
        RowSizeIterator {
            two_d_array: array,
            index: 0,
        }
    }

    // Get the next biggest row in the array.
    //
    // # Returns
    // The next biggest row in the array.
    pub fn next_biggest(&mut self) -> Option<(usize, &'a Row)> {
        if let Some((index, row)) = self.two_d_array.get_row_by_size(self.index) {
            self.index += 1;
            return Some((index, row));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ElcAlgorithm;

    #[test]
    fn two_d_array_unit_test() {
        let hash_algorithm: ElcAlgorithm = ElcAlgorithm::default();

        let mut word_list = WordList::new();
        word_list.push("WORD");

        match TwoDArray::new(&word_list, &hash_algorithm) {
            Ok(a) => {
                assert_eq!(a.get_num_entries(), 1);
                assert_eq!(a.get_num_rows(), 1);
                println!("{a:?}");
            }
            Err(e) => panic!("Unexpected 2D array creation failure. {e}"),
        }

        word_list.push("WIRE");
        word_list.push("ABLE");
        match TwoDArray::new(&word_list, &hash_algorithm) {
            Ok(a) => {
                assert_eq!(a.get_num_entries(), 3);
                assert_eq!(a.get_num_rows(), 2);
                let mut it = RowSizeIterator::new(&a);
                println!("{it:?}");
                if let Some((row_index, row)) = it.next_biggest() {
                    assert_eq!(row_index, 22);
                    assert_eq!(row.cols.len(), 2);
                    assert_eq!(row.get_col_indices(), vec![3, 4]);
                    assert_eq!(row.get_col_values(), vec![1, 2]);
                } else {
                    panic!("Unexpected iterator None");
                }
                if let Some((row_index, row)) = it.next_biggest() {
                    assert_eq!(row_index, 0);
                    assert_eq!(row.cols.len(), 1);
                } else {
                    panic!("Unexpected iterator None");
                }
                if let Some((_, _)) = it.next_biggest() {
                    panic!("Unexpected iterator Some");
                }
            }
            Err(e) => panic!("Unexpected 2D array creation failure. {e}"),
        }

        word_list.push("WILD");
        if let Ok(_a) = TwoDArray::new(&word_list, &hash_algorithm) {
            panic!("Undetected collision.");
        }
    }
}
