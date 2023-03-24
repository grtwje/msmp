#!/bin/bash
echo "Running integration tests coverage. Output in coverage directoty."

cargo clean; rm -rf coverage

RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="coverage/cargo-test-%p-%m.profraw" cargo test --test simple_tests

rust-profdata merge -sparse coverage/cargo-test-*.profraw -o coverage/cargo-test.profdata

rust-cov report \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=coverage/cargo-test.profdata \
    --object target/debug/deps/libmsmp-*.rlib \
    --object target/debug/deps/simple_tests-*.exe \
    > coverage/integration_tests_summary.txt

rust-cov show \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=coverage/cargo-test.profdata \
    --object target/debug/deps/libmsmp-*.rlib \
    --object target/debug/deps/simple_tests-*.exe \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --format=html \
    > coverage/integration_tests_report.html
