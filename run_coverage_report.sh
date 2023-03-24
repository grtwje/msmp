#!/bin/bash
############################################################
# Help                                                     #
############################################################
Help()
{
    # Display Help
    echo "Generate code coverage report for specified tests."
    echo
    echo "Syntax: run_coverage_report.sh [unit|integ]"
    echo "options:"
    echo "unit     Run code coverage on unit tests."
    echo "integ    Run code coverage on integration tests."
    echo
}

if [ $# != 1 ]; then
    Help
    exit 1
fi

if [[ "$1" == "unit" ]]; then
    coverage_type="unit"
    coverage_cmd="--lib"
    objects="--object target/debug/deps/msmp-*.exe"
elif [[ "$1" == "integ" ]]; then
    coverage_type="integration"
    coverage_cmd="--test *"
    objects="--object target/debug/deps/libmsmp-*.rlib \
             --object target/debug/deps/simple_tests-*.exe"
else
    Help
    exit 1
fi

dest="target/coverage"

echo "Running $coverage_type tests coverage. Output in $dest directory."

cargo clean

set -o noglob
RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="$dest/cargo-test-%p-%m.profraw" cargo test $coverage_cmd
set +o noglob

rust-profdata merge -sparse $dest/cargo-test-*.profraw -o $dest/cargo-test.profdata

rust-cov report \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=$dest/cargo-test.profdata \
    $objects \
    > $dest/${coverage_type}_coverage_summary.txt

rust-cov show \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=$dest/cargo-test.profdata \
    $objects \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --format=html \
    > $dest/${coverage_type}_coverage_report.html
