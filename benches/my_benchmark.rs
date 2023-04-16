use criterion::{criterion_group, criterion_main, Criterion};

use msmp::{generate_hash, ElcAlgorithm, WordList};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

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

fn elc2_benchmark(word_list: &WordList) {
    let hash_algorithm: ElcAlgorithm = ElcAlgorithm::default();

    if generate_hash(word_list, hash_algorithm).is_err() {
        panic!("generate_hash failed");
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input_file_name: PathBuf = ["tests", "data", "pascal_keyword_subset.txt"]
        .iter()
        .collect();

    let word_list = match load_word_list(&input_file_name) {
        Some(word_list) => word_list,
        None => panic!("Error processing {:?}.", input_file_name),
    };

    c.bench_function("my_benchmark", |b| b.iter(|| elc2_benchmark(&word_list)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
