use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use msmp::{generate_hash, ElcAlgorithm, WordList};
use rand::prelude::*;
use rand::seq::IteratorRandom;
use rand_chacha::ChaCha8Rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

fn open_file(input_file_name: &PathBuf) -> Option<File> {
    let fh = match File::open(input_file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("unable to read {:?}: {}.", input_file_name, err);
            return None;
        }
    };

    Some(fh)
}

fn load_random_word_list(
    input_file_name: &PathBuf,
    word_list_size: usize,
    seed: u64,
) -> Option<WordList> {
    let fh = match open_file(input_file_name) {
        Some(fh) => fh,
        None => return None,
    };

    let reader = BufReader::new(fh);
    let lines_iter = reader.lines();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let random_sample = lines_iter.choose_multiple(&mut rng, word_list_size);
    let mut random_word_list = WordList::new();

    for word in random_sample {
        match word {
            Ok(word) => random_word_list.push(&word.trim().to_string().to_ascii_uppercase()),
            Err(_) => return None,
        }
    }

    Some(random_word_list)
}

fn elc_benchmark(word_list: &WordList, elc: usize) {
    let hash_algorithm: ElcAlgorithm = ElcAlgorithm::new(elc, 26);

    if generate_hash(word_list, hash_algorithm).is_err() {
        panic!("generate_hash failed");
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input_file_name: PathBuf = ["tests", "data", "aspell_dump.txt"].iter().collect();

    let word_list = match load_random_word_list(&input_file_name, 8, 1) {
        Some(word_list) => word_list,
        None => panic!("Error processing {:?}.", input_file_name),
    };

    println!("word_list = {:?}", word_list);

    let mut group = c.benchmark_group("elc_benchmarks");
    for elc in 1..=4 {
        group.bench_with_input(BenchmarkId::new("elc", elc), &elc, |b, elc| {
            b.iter(|| elc_benchmark(&word_list, *elc))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
