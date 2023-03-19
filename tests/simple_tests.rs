use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use msmp::{generate_hash, WordList};

fn load_word_list(input_file_name: &PathBuf) -> Option<WordList> {
    let fh = match File::open(input_file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("unable to read {:?}: {}.", input_file_name, err);
            return None;
        }
    };
    let reader = BufReader::new(fh);
    let mut word_list = WordList::new();

    for (line_num, line) in reader.lines().enumerate() {
        let raw_word = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!(
                    "error to reading {:?}:{} {}.",
                    input_file_name, line_num, err
                );
                return None;
            }
        };

        let word = raw_word.trim().to_string();

        word_list.push(&word);
    }

    Some(word_list)
}

#[test]
fn pascal_keyword_subset_test() {
    let input_file_name: PathBuf = [".", "tests", "data", "pascal_keyword_subset.txt"]
        .iter()
        .collect();
    let word_list = match load_word_list(&input_file_name) {
        Some(word_list) => word_list,
        None => panic!("Error processing {:?}.", input_file_name),
    };
    assert_eq!(word_list.len(), 8);

    match generate_hash(&word_list) {
        Ok(hash) => assert!(hash == "test"),
        Err(e) => panic!("generate_hash failed {e}"),
    }
}
