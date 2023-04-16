use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;
use std::iter::zip;

use crate::{Error, Kind, Rlt, Row, RowSizeIterator, TwoDArray};

/// A one dimensional packed array.
#[derive(Debug)]
pub struct OneDPackedArray {
    /// The packed array.
    array: Vec<usize>,

    /// The row lookup table. The row lookup table is used to find the index of the first element in
    /// the array for a given row.
    rlt: Rlt,
}

impl OneDPackedArray {
    /// Create a new one dimensional packed array.
    /// The array is created by packing the values in the 2D array into a one dimensional array.
    /// The array is packed by placing the values in the row into the array in order.
    ///
    /// # Parameters
    /// * `two_d_array` - The 2D array to pack.
    ///
    /// # Returns
    /// A new one dimensional packed array.
    ///
    /// # Errors
    /// Will return `Err` if the array cannot be packed.
    pub fn new(two_d_array: &TwoDArray) -> Result<Self, Error> {
        let mut self_ = OneDPackedArray {
            array: vec![0; two_d_array.get_num_entries()],
            rlt: Rlt::new(two_d_array.get_last_row_index() + 1),
        };
        let mut rlt_wrk: BTreeMap<usize, isize> = BTreeMap::new();

        let mut unused_array_indices: BTreeSet<usize> =
            (0..two_d_array.get_num_entries()).collect();

        // * Loop through all rows containing one or more values.
        let mut it = RowSizeIterator::new(two_d_array);
        while let Some((row_index, row)) = it.next_biggest() {
            let col_indices = row.get_col_indices();
            if let Some(fist_col_index) = col_indices.first() {
                if let Ok(rlt_seed) = isize::try_from(*fist_col_index) {
                    if let Some(first_array_unused_index) = unused_array_indices.first() {
                        if let Ok(first_array_unused_index) =
                            isize::try_from(*first_array_unused_index)
                        {
                            let mut rlt_value = -rlt_seed + first_array_unused_index;

                            // Try to insert the row into the packed array. Keep shifting the row index
                            // until it can be inserted.
                            while self_.not_inserted(&mut unused_array_indices, row, rlt_value) {
                                rlt_value += 1; // todo step to next unused index
                                if let Ok(num_entries) =
                                    isize::try_from(two_d_array.get_num_entries())
                                {
                                    if rlt_value + rlt_seed >= num_entries {
                                        return Err(Error::new(Kind::OneDPackedArrayError(
                                            "unable to minimally pack array".to_string(),
                                        )));
                                    }
                                } else {
                                    return Err(Error::new(Kind::OneDPackedArrayError(
                                        "Unexpected num entries overflow".to_string(),
                                    )));
                                }
                            }

                            // Record how much the row was shifted in a row lookup map.
                            rlt_wrk.insert(row_index, rlt_value);
                        } else {
                            return Err(Error::new(Kind::OneDPackedArrayError(
                                "Unexpected index overflow".to_string(),
                            )));
                        }
                    } else {
                        return Err(Error::new(Kind::OneDPackedArrayError(
                            "Unexpected no unused index found".to_string(),
                        )));
                    }
                } else {
                    return Err(Error::new(Kind::OneDPackedArrayError(
                        "Unexpected empty row found".to_string(),
                    )));
                }
            }
        }

        // Convert the row lookup map into a row lookup table.
        let it = rlt_wrk.iter();
        for (row_index, rlt_value) in it {
            self_.rlt.insert(*row_index, *rlt_value);
        }
        self_.rlt.set_num_entries(self_.array.len());

        Ok(self_)
    }

    /// Get the row lookup table for the packed array.
    /// The row lookup table is used to find the index of the first element in the array for a given
    /// row.
    ///
    /// # Returns
    /// The row lookup table.
    pub fn get_rlt(&self) -> &Rlt {
        &self.rlt
    }

    /// Is the packed array empty.
    /// Only used for testing.
    ///
    /// # Returns
    /// True if the packed array is empty.
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    /// Attempt to insert a row into the packed array.
    ///
    /// The row is inserted into the packed array if it can be inserted without overlapping any
    /// existing values.
    /// The row is inserted into the packed array by placing the values in the row into the array in
    /// order.
    /// The row is shifted by the given amount before being inserted.
    ///
    /// # Parameters
    /// * `unused_array_indices` - The set of unused indices in the packed array.
    /// * `row` - The row to insert.
    /// * `rlt_value` - The amount to shift the row by before inserting.
    ///
    /// # Returns
    /// True if the row cannot be inserted without overlapping an existing value.
    /// False if the row can be inserted without overlapping an existing value.
    fn not_inserted(
        &mut self,
        unused_array_indices: &mut BTreeSet<usize>,
        row: &Row,
        rlt_value: isize,
    ) -> bool {
        let col_indices = row.get_col_indices();

        // Adjust the rows's column indices by the row lookup table value.
        let adj_col_indices: Vec<usize> = col_indices
            .iter()
            .map(|x| OneDPackedArray::adjust_index(*x, rlt_value, self.array.len()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        // Sanity check the adjusted column indices.
        if adj_col_indices.len() != col_indices.len() {
            return true;
        }

        // Check if any of the adjusted column indices are already in use.
        {
            let it = adj_col_indices.iter();
            for i in it {
                if !unused_array_indices.contains(i) {
                    return true;
                }
            }
        }

        // Remove the adjusted column indices from the set of unused indices.
        let it = adj_col_indices.iter();
        for i in it {
            unused_array_indices.remove(i);
        }

        // Insert the row's values into the packed array at the adjusted column indices.
        let col_values = row.get_col_values();
        let it = zip(col_values.iter(), adj_col_indices.iter());
        for (v, adj_i) in it {
            self.array[*adj_i] = *v;
        }

        false
    }

    /// Adjust an index by the given amount.
    /// The index is adjusted by adding the given amount to the index.
    /// The index is then wrapped around the given number of entries.
    /// The adjusted index is returned.
    ///
    /// # Parameters
    /// * `index` - The index to adjust.
    /// * `adj` - The amount to adjust the index by.
    /// * `num_entries` - The number of entries in the array.
    ///
    /// # Panics
    /// Panics if the adjusted index is negative.
    /// Panics if the index or number of entries overflows.
    ///
    /// # Returns
    /// The adjusted index.
    fn adjust_index(index: usize, adj: isize, num_entries: usize) -> usize {
        let index_ = isize::try_from(index);
        let num_entries_ = isize::try_from(num_entries);
        if let (Ok(index_), Ok(num_entries_)) = (index_, num_entries_) {
            let adj_index = (index_ + adj) % num_entries_;
            if let Ok(rv) = usize::try_from(adj_index) {
                return rv;
            }
            panic!("Unexpected negative adjusted index: {adj_index}");
        }
        panic!("Unexpected parameter overflow: {index}, {num_entries}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ElcAlgorithm, WordList};

    #[test]
    fn one_d_packed_array_unit_test() {
        let hash_algorithm: ElcAlgorithm = ElcAlgorithm::default();

        let mut word_list = WordList::new();
        word_list.push("AXXA");
        word_list.push("AXXC");
        word_list.push("AXXD");
        word_list.push("BXXA");
        word_list.push("BXXC");

        if let Ok(tda) = TwoDArray::new(&word_list, &hash_algorithm) {
            if let Ok(odpa) = OneDPackedArray::new(&tda) {
                println!("{odpa:?}");
                assert_eq!(odpa.array, vec![1, 4, 2, 3, 5]);
                assert_eq!(odpa.rlt.get(0), Some(&0));
                assert_eq!(odpa.rlt.get(1), Some(&4));
                assert_eq!(odpa.array.len(), 5);
                assert!(!odpa.is_empty());
            } else {
                panic!("Unable to create OneDPackedArray");
            }
        } else {
            panic!("Unable to create TwoDArray");
        }

        word_list.push("BXXZ");
        if let Ok(tda) = TwoDArray::new(&word_list, &hash_algorithm) {
            match OneDPackedArray::new(&tda) {
                Ok(_) => panic!("Should not be able to create OneDPackedArray"),
                Err(e) => match e.kind() {
                    Kind::OneDPackedArrayError(s) => {
                        assert_eq!(s, "unable to minimally pack array");
                    }
                    _ => panic!("Unexpected error type"),
                },
            }
        } else {
            panic!("Unable to create TwoDArray");
        }

        let mut word_list2 = WordList::new();
        word_list2.push("WORD");
        word_list2.push("WORF");

        if let Ok(tda) = TwoDArray::new(&word_list2, &hash_algorithm) {
            match OneDPackedArray::new(&tda) {
                Ok(_) => panic!("Should not be able to create OneDPackedArray"),
                Err(e) => match e.kind() {
                    Kind::OneDPackedArrayError(s) => {
                        assert_eq!(s, "unable to minimally pack array");
                    }
                    _ => panic!("Unexpected error type"),
                },
            }
        } else {
            panic!("Unable to create TwoDArray");
        }
    }
}
