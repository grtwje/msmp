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
            println!("{row_index:?}: {row:?}");

            let col_indices = row.get_col_indices();
            if let Some(fist_col_index) = col_indices.first() {
                if let Ok(rlt_seed) = isize::try_from(*fist_col_index) {
                    let first_array_unused_index =
                        *(unused_array_indices.first().unwrap()) as isize;
                    let mut rlt_value = -rlt_seed + first_array_unused_index;
                    println!("rlt_value: {rlt_value}");

                    while _self.not_inserted(&mut unused_array_indices, row, rlt_value) {
                        rlt_value += 1; // todo step to next unused index
                        if rlt_value + rlt_seed >= two_d_array.get_num_entries() as isize {
                            return Err(Error::new(Kind::OneDPackedArrayError(
                                "packed array overflow".to_string(),
                            )));
                        }
                    }
                    _self.rlt.insert(row_index, rlt_value);

                    println!("new rlt_value: {rlt_value}");
                    break;
                } else {
                    return Err(Error::new(Kind::OneDPackedArrayError(
                        "Unexpected empty row found".to_string(),
                    )));
                }
            }
        }

        Ok(_self)
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
            .map(|x| OneDPackedArray::adjust_index(*x, rlt_value))
            .collect();

        {
            let it = adj_col_indices.iter();
            println!("..{adj_col_indices:?}");
            for i in it {
                if !unused_array_indices.contains(i) {
                    println!("{i} not in unused");
                    println!("   {unused_array_indices:?}");
                    return true;
                }
            }
        }

        let _ = adj_col_indices
            .iter()
            .map(|i| unused_array_indices.remove(i));

        let col_values = row.get_col_values();
        let it = zip(col_values.iter(), adj_col_indices.iter());
        for (v, adj_i) in it {
            self.array.insert(*adj_i, *v);
        }

        false
    }

    fn adjust_index(index: usize, adj: isize) -> usize {
        let adj_index = (index as isize) + adj;
        assert!(adj_index >= 0);
        adj_index as usize
    }
}
