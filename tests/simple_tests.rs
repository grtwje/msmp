use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use msmp::{generate_hash, ElcAlgorithm, WordList};

fn load_word_list(input_file_name: &PathBuf) -> Option<WordList> {
    let fh = match File::open(input_file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("unable to read {:?}: {}.", input_file_name, err);
            return None;
        }
    };

    let reader = BufReader::new(fh);
    let word_list: WordList = reader
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect();

    Some(word_list)
}

#[test]
fn pascal_keyword_subset_integ_test() {
    let input_file_name: PathBuf = [".", "tests", "data", "pascal_keyword_subset.txt"]
        .iter()
        .collect();
    let word_list = match load_word_list(&input_file_name) {
        Some(word_list) => word_list,
        None => panic!("Error processing {:?}.", input_file_name),
    };
    assert_eq!(word_list.len(), 8);

    let hash_algorithm: ElcAlgorithm = ElcAlgorithm::default();

    match generate_hash(&word_list, hash_algorithm) {
        Ok(hash) => {
            println!(":::\n{}:::", hash.as_string);
            assert!(hash.as_string.len() > 1);
            assert_eq!((hash.as_closure.cls)("AND"), 4);
            assert_eq!((hash.as_closure.cls)("BEGIN"), 7);
            assert_eq!((hash.as_closure.cls)("CHAR"), 3);
            assert_eq!((hash.as_closure.cls)("EOF"), 2);
        }
        Err(e) => panic!("generate_hash failed {e}"),
    }
}
