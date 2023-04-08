#[derive(Debug)]
pub struct Rlt {
    table: Vec<isize>,

    /// This is not the number of entries in the table,
    /// but the number of words in the word list used to create the table.
    num_words: usize,
}

impl Rlt {
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![0; size],
            num_words: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<&isize> {
        self.table.get(index)
    }

    pub fn insert(&mut self, index: usize, value: isize) {
        self.table[index] = value;
    }

    pub fn get_num_entries(&self) -> usize {
        self.num_words
    }

    pub fn set_num_entries(&mut self, num_words: usize) {
        self.num_words = num_words;
    }

    pub fn get_as_text(&self) -> String {
        self.table
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rlt_unit_test() {
        let mut rlt = Rlt::new(5);
        rlt.insert(0, 1);
        rlt.insert(1, 2);
        rlt.insert(2, 3);
        rlt.insert(3, 4);
        rlt.insert(4, 5);

        assert_eq!(rlt.get(0), Some(&1));
        assert_eq!(rlt.get(1), Some(&2));
        assert_eq!(rlt.get(2), Some(&3));
        assert_eq!(rlt.get(3), Some(&4));
        assert_eq!(rlt.get(4), Some(&5));
        assert_eq!(rlt.get(5), None);
        assert_eq!(rlt.get(6), None);
        assert_eq!(rlt.get(7), None);
        assert_eq!(rlt.get(8), None);
        assert_eq!(rlt.get(9), None);

        assert_eq!(rlt.get_num_entries(), 0);
        rlt.set_num_entries(10);
        assert_eq!(rlt.get_num_entries(), 10);

        assert_eq!(rlt.get_as_text(), "1, 2, 3, 4, 5");
    }
}
