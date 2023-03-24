#!/bin/bash
echo "Running unit tests coverage. Output in coverage directoty."

cargo clean; rm -rf coverage

RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="coverage/cargo-test-%p-%m.profraw" cargo test --lib

rust-profdata merge -sparse coverage/cargo-test-*.profraw -o coverage/cargo-test.profdata

rust-cov report \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=coverage/cargo-test.profdata \
    --object target/debug/deps/msmp-*.exe \
    > coverage/unit_tests_summary.txt

rust-cov show \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=coverage/cargo-test.profdata \
    --object target/debug/deps/msmp-*.exe \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --format=html \
    > coverage/unit_tests_report.html
