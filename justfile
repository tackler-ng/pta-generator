# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# pta-generator 2025
# SPDX-License-Identifier: Apache-2.0
#

# just list all targets
default:
    @just --list

alias c  := check
alias ut := test
# alias it := integration-test

alias db := debug-build
alias rb := release-build

# clean the workspace
clean:
    cargo clean

# run code style and linter checks
check: clippy
    cargo fmt --all --check -- --style-edition 2024

# run clippy the linter
clippy:
    cargo clippy --workspace --all-targets --no-deps -- -D warnings

# format code
fmt:
    cargo fmt --all -- --style-edition 2024

# run tests
test:
    cargo test


# build the debug target
debug-build:
    cargo build --bin pta-generator

# build the release target
release-build:
    cargo build --release --bin pta-generator

# run audit checks (advisories, bans, licenses, sources)
audit:
    cargo deny check advisories bans licenses sources

