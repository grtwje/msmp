use crate::{Error, Kind, TwoDArray, TwoDArraySizeIterator};

#[derive(Debug)]
pub struct OneDPackedArray {
    array: Vec<usize>,
    rlt: Vec<usize>,
}

impl OneDPackedArray {
    pub fn new(two_d_array: &TwoDArray) -> Result<Self, Error> {
        let mut _self = OneDPackedArray {
            array: Vec::with_capacity(two_d_array.get_num_entries()),
            rlt: Vec::with_capacity(two_d_array.get_num_rows()),
        };

        let mut it = TwoDArraySizeIterator::new(two_d_array);
        while let Some((row_index, row)) = it.next_biggest() {
            println!("{row_index:?}: {row:?}");
        }

        Ok(_self)
        //Err(Error::new(Kind::OneDPackedArrayError("tmp".to_string())))
    }
}
