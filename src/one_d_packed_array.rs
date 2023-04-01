use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;
use std::iter::zip;

use crate::{Error, Kind, Row, TwoDArray, TwoDArraySizeIterator};

#[derive(Debug)]
pub struct OneDPackedArray {
    array: Vec<usize>,
    rlt: BTreeMap<usize, isize>,
}

impl OneDPackedArray {
    pub fn new(two_d_array: &TwoDArray) -> Result<Self, Error> {
        let mut _self = OneDPackedArray {
            array: vec![0; two_d_array.get_num_entries()],
            rlt: BTreeMap::new(),
        };

        let mut unused_array_indices: BTreeSet<usize> =
            (0..two_d_array.get_num_entries()).collect();

        // * Loop through all rows containing one or more values.
        let mut it = TwoDArraySizeIterator::new(two_d_array);
        while let Some((row_index, row)) = it.next_biggest() {
            let col_indices = row.get_col_indices();
            if let Some(fist_col_index) = col_indices.first() {
                if let Ok(rlt_seed) = isize::try_from(*fist_col_index) {
                    let first_array_unused_index =
                        *(unused_array_indices.first().unwrap()) as isize;
                    let mut rlt_value = -rlt_seed + first_array_unused_index;

                    while _self.not_inserted(&mut unused_array_indices, row, rlt_value) {
                        rlt_value += 1; // todo step to next unused index
                        if rlt_value + rlt_seed >= two_d_array.get_num_entries() as isize {
                            return Err(Error::new(Kind::OneDPackedArrayError(
                                "unable to minimally pack array".to_string(),
                            )));
                        }
                    }
                    _self.rlt.insert(row_index, rlt_value);
                } else {
                    return Err(Error::new(Kind::OneDPackedArrayError(
                        "Unexpected empty row found".to_string(),
                    )));
                }
            }
        }

        Ok(_self)
    }

    pub fn get_rlt(&self, row_index: usize) -> Option<&isize> {
        self.rlt.get(&row_index)
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    fn not_inserted(
        &mut self,
        unused_array_indices: &mut BTreeSet<usize>,
        row: &Row,
        rlt_value: isize,
    ) -> bool {
        let col_indices = row.get_col_indices();

        let adj_col_indices: Vec<usize> = col_indices
            .iter()
            .map(|x| OneDPackedArray::adjust_index(*x, rlt_value, self.array.len()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        if adj_col_indices.len() != col_indices.len() {
            return true;
        }

        {
            let it = adj_col_indices.iter();
            for i in it {
                if !unused_array_indices.contains(i) {
                    return true;
                }
            }
        }

        let it = adj_col_indices.iter();
        for i in it {
            unused_array_indices.remove(i);
        }

        let col_values = row.get_col_values();
        let it = zip(col_values.iter(), adj_col_indices.iter());
        for (v, adj_i) in it {
            self.array[*adj_i] = *v;
        }

        false
    }

    fn adjust_index(index: usize, adj: isize, num_entries: usize) -> usize {
        let adj_index = ((index as isize) + adj) % num_entries as isize;
        assert!(adj_index >= 0);
        adj_index as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WordList;

    #[test]
    fn one_d_packed_array_unit_test() {
        let mut word_list = WordList::new();
        word_list.push("AXXA");
        word_list.push("AXXC");
        word_list.push("AXXD");
        word_list.push("BXXA");
        word_list.push("BXXC");

        if let Ok(tda) = TwoDArray::new(&word_list) {
            if let Ok(odpa) = OneDPackedArray::new(&tda) {
                println!("{:?}", odpa);
                assert_eq!(odpa.array, vec![1, 4, 2, 3, 5]);
                assert_eq!(odpa.rlt, [(0, 0), (1, 4)].iter().cloned().collect());
                assert_eq!(odpa.get_rlt(0), Some(&0));
                assert_eq!(odpa.len(), 5);
                assert!(!odpa.is_empty());
            } else {
                panic!("Unable to create OneDPackedArray");
            }
        } else {
            panic!("Unable to create TwoDArray");
        }

        word_list.push("BXXZ");
        if let Ok(tda) = TwoDArray::new(&word_list) {
            match OneDPackedArray::new(&tda) {
                Ok(_) => panic!("Should not be able to create OneDPackedArray"),
                Err(e) => match e.kind() {
                    Kind::OneDPackedArrayError(s) => {
                        assert_eq!(s, "unable to minimally pack array")
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

        if let Ok(tda) = TwoDArray::new(&word_list2) {
            match OneDPackedArray::new(&tda) {
                Ok(_) => panic!("Should not be able to create OneDPackedArray"),
                Err(e) => match e.kind() {
                    Kind::OneDPackedArrayError(s) => {
                        assert_eq!(s, "unable to minimally pack array")
                    }
                    _ => panic!("Unexpected error type"),
                },
            }
        } else {
            panic!("Unable to create TwoDArray");
        }
    }
}
