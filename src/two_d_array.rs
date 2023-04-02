use crate::{Error, HashAlgorithm, Kind, WordList};
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::iter::zip;

#[derive(Debug)]
pub struct TwoDArray {
    rows: BTreeMap<usize, Row>,
    rows_by_size: Vec<usize>,
    num_entries: usize,
    num_rows: usize,
}

#[derive(Debug)]
pub struct Row {
    cols: BTreeMap<usize, usize>,
}

impl<'a> TwoDArray {
    pub fn new(word_list: &WordList, hash_algorithm: &dyn HashAlgorithm) -> Result<Self, Error> {
        let num_words = word_list.len();

        let mut _self = TwoDArray {
            rows: BTreeMap::new(),
            rows_by_size: Vec::new(),
            num_entries: num_words,
            num_rows: 0,
        };

        {
            // Calculate the indices that will be used in the 2D array.
            let mut row_indices = Vec::with_capacity(num_words);
            let mut col_indices = Vec::with_capacity(num_words);
            for word in word_list.list.iter() {
                let row = hash_algorithm.h1(word)?;
                row_indices.push(row);

                let col = hash_algorithm.h2(word)?;
                col_indices.push(col);
            }

            // * Fill the 2-D array with values.
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
        }

        // * Sort the rows of the array.
        // Build secondary index, rows sorted by size.
        let mut rows_by_size: Vec<(usize, usize)> = Vec::new();
        for (i, r) in _self.rows.iter() {
            rows_by_size.push((r.cols.len(), *i));
        }
        rows_by_size.sort_by_key(|k| (Reverse(k.0)));
        _self.rows_by_size = rows_by_size.iter().map(|a| a.1).collect::<Vec<usize>>();

        _self.num_rows = _self.rows.len();

        Ok(_self)
    }

    pub fn get_num_entries(&self) -> usize {
        self.num_entries
    }

    #[cfg(test)]
    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

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
    pub fn get_col_indices(&self) -> Vec<usize> {
        let indices: Vec<_> = self.cols.keys().cloned().collect();
        indices
    }

    pub fn get_col_values(&self) -> Vec<usize> {
        let values: Vec<_> = self.cols.values().cloned().collect();
        values
    }
}

#[derive(Debug)]
pub struct TwoDArraySizeIterator<'a> {
    two_d_array: &'a TwoDArray,
    index: usize,
}

impl<'a> TwoDArraySizeIterator<'a> {
    pub fn new(array: &'a TwoDArray) -> Self {
        TwoDArraySizeIterator {
            two_d_array: array,
            index: 0,
        }
    }

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
        let hash_algorithm: ElcAlgorithm = Default::default();

        let mut word_list = WordList::new();
        word_list.push("WORD");

        match TwoDArray::new(&word_list, &hash_algorithm) {
            Ok(a) => {
                assert_eq!(a.get_num_entries(), 1);
                assert_eq!(a.get_num_rows(), 1);
                println!("{:?}", a);
            }
            Err(e) => panic!("Unexpected 2D array creation failure. {e}"),
        }

        word_list.push("WIRE");
        word_list.push("ABLE");
        match TwoDArray::new(&word_list, &hash_algorithm) {
            Ok(a) => {
                assert_eq!(a.get_num_entries(), 3);
                assert_eq!(a.get_num_rows(), 2);
                let mut it = TwoDArraySizeIterator::new(&a);
                println!("{:?}", it);
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
