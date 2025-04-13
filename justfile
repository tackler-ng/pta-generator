# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# pta-generator 2025
# SPDX-License-Identifier: Apache-2.0
#

set positional-arguments

time := 'time'
#time := 'gtime'


# List all targets
default:
    @just --list --unsorted

alias c  := check
alias ut := unit-test
alias it := integration-test

alias db := debug-build
alias rb := release-build

# Clean the workspace (cargo)
clean:
    cargo clean

# Clean all data under data/{audit,plain,comm}
clean-data:
    rm -rf data/audit
    rm -rf data/plain
    rm -rf data/comm

# Run audit checks (advisories, bans, licenses, sources)
audit:
    cargo deny check advisories bans licenses sources

# Format code
fmt:
    cargo fmt --all -- --style-edition 2024

# Run Clippy the linter
clippy:
    cargo clippy --workspace --all-targets --no-deps -- -D warnings

# Check code style and clippy lints
check: clippy
    cargo fmt --all --check -- --style-edition 2024

# Run all tests (this needs tackler, ledger, hledger and bean-query installed)
test: unit-test integration-test

# Run unit tests
unit-test:
    cargo test

# Data path under target, this is cleaned by cargo clean and by integration-test target
data_path := "target/data"
it_bin := "target/release/pta-generator"

# Run integration tests with tackler, ledger, hledger and bean-query (beancount)
integration-test: release-build _it-audit _it-plain _it-comm

_it-audit:
    rm -rf "{{data_path}}/audit"
    {{it_bin}} audit --path {{data_path}} --set-size 1e2 --shard-type txn --flavor tackler
    {{it_bin}} audit --path {{data_path}} --set-size 1e2 --shard-type month
    {{it_bin}} audit --path {{data_path}} --set-size 1e2 --shard-type single

    tackler --config {{data_path}}/audit/set-1e2-single.toml > /dev/null
    tackler --config {{data_path}}/audit/set-1e2-month.toml > /dev/null
    tackler --config {{data_path}}/audit/set-1e2-txn.toml > /dev/null

_it-plain:
    rm -rf "{{data_path}}/plain"
    {{it_bin}} plain --path {{data_path}} --set-size 1e1 --shard-type txn
    {{it_bin}} plain --path {{data_path}} --set-size 1e1 --shard-type month --flavor tackler
    {{it_bin}} plain --path {{data_path}} --set-size 1e1 --shard-type single
    {{it_bin}} plain --path {{data_path}} --set-size 1e1 --shard-type single --flavor ledger
    
    tackler --config {{data_path}}/plain/set-1e1-single.toml > /dev/null
    tackler --config {{data_path}}/plain/set-1e1-month.toml > /dev/null
    tackler --config {{data_path}}/plain/set-1e1-txn.toml > /dev/null
    ledger        -f {{data_path}}/plain/set-1e1-single/txns/1e1.journal bal >/dev/null
    hledger       -f {{data_path}}/plain/set-1e1-single/txns/1e1.journal bal >/dev/null

_it-comm:
    rm -rf "{{data_path}}/comm"
    {{it_bin}} comm --path {{data_path}} --set-size 1e1 --shard-type txn
    {{it_bin}} comm --path {{data_path}} --set-size 1e1 --shard-type month

    {{it_bin}} comm --path {{data_path}} --set-size 1e4 --shard-type single --flavor tackler
    {{it_bin}} comm --path {{data_path}} --set-size 1e4 --shard-type single --flavor ledger
    {{it_bin}} comm --path {{data_path}} --set-size 1e4 --shard-type single --flavor beancount

    tackler --config {{data_path}}/comm/set-1e1-txn.toml > /dev/null
    tackler --config {{data_path}}/comm/set-1e1-month.toml > /dev/null
    
    @echo "###"
    @echo "### Journal with Commodities, 10_000 (1e4) txns"
    @echo "###"

    {{time}} tackler --config {{data_path}}/comm/set-1e4-single.toml > /dev/null
    {{time}} ledger        -f {{data_path}}/comm/set-1e4-single/txns/1e4.journal bal >/dev/null
    {{time}} hledger       -f {{data_path}}/comm/set-1e4-single/txns/1e4.journal bal >/dev/null
    {{time}} bean-query       {{data_path}}/comm/set-1e4-single/txns/1e4.beancount  'balances from year = 2024' >/dev/null

# Build the debug target
debug-build:
    cargo build --bin pta-generator

# Build the release target
release-build:
    cargo build --release --bin pta-generator


# Run the pta-generator in release mode by cargo
run *ARGS:
    cargo run --release -- {{ ARGS }}

