[![Build Status](https://github.com/tackler-ng/pta-generator/actions/workflows/ci.yml/badge.svg)](https://github.com/tackler-ng/pta-generator/actions)
[![Github Releases](https://img.shields.io/github/v/release/tackler-ng/pta-generator?include_prereleases&color=%230868da)](https://github.com/tackler-ng/pta-generator/releases)
[![Crates.io Version](https://img.shields.io/crates/v/pta-generator)](https://crates.io/crates/pta-generator)
[![GitHub Discussions](https://img.shields.io/github/discussions/tackler-ng/pta-generator)](https://github.com/tackler-ng/pta-generator/discussions)
[![Matrix](https://img.shields.io/matrix/tackler%3Amatrix.org)](https://matrix.to/#/#tackler:matrix.org)

# Test Data Generator for Plain Text Accounting

PTA-Generator generates [plain text accounting](https://plaintextaccounting.org/) data sets 
("journals") for test and demo usage.

It creates journals with or without commodities and journals with audit data for [tackler](https://tackler.e257.fi/). 
PTA-Generator can also crete journals of different sizes, and it also supports different journal storing strategies.

Currently it has following features:

* Support for following PTA tools
    * [tackler](https://tackler.e257.fi/)
    * [ledger](https://ledger-cli.org/)
    * [hledger](https://hledger.org/)
    * [beancount](https://beancount.github.io/)
    * [rustledger](https://github.com/rustledger/rustledger)
* Three major modes:
    * `comm`: Journal with commodities
        * Tools: tackler, (h)ledger, beancount, rustledger
    * `plain`: The simplest journal
        * Tools: tackler, (h)ledger
    * `audit`: Journal with transaction audit data
        * Tools: tackler
* Three journal storage strategies:
    * `single`: Single journal
        * Tools: tackler, (h)ledger, beancount
    * `month`: Shard by transaction date
        * Tools: tackler
    * `txn`: Shard by transaction (each txn is in own file)
        * Tools: tackler
* Journal sizes from 10 (1e1) to 1_000_000 (1e6) transactions


## Usage

For full command line help, see help for each subcommands:

````bash
pta-generator help audit
pta-generator help plain
pta-generator help comm
````

### Simple Journal

````bash
# Generate 10_000 (1e4) txns single file journals under `data` directory
pta-generator plain --path data --set-size 1e4 --shard-type single --flavor tackler
pta-generator plain --path data --set-size 1e4 --shard-type single --flavor ledger


# Run balance report with this test data
tackler    --config data/plain/set-1e4-single.toml
ledger -no-pager -f data/plain/set-1e4-single/txns/1e4.journal
hledger          -f data/plain/set-1e4-single/txns/1e4.journal
````

### Journal with Commodities

````bash
# Generate 10_000 (1e4) txns single file journals under `data` directory
pta-generator comm --path data --set-size 1e4 --shard-type single --flavor tackler
pta-generator comm --path data --set-size 1e4 --shard-type single --flavor ledger
pta-generator comm --path data --set-size 1e4 --shard-type single --flavor beancount

# Run balance report with this test data
tackler    --config data/comm/set-1e4-single.toml
ledger -no-pager -f data/comm/set-1e4-single/txns/1e4.journal bal
hledger          -f data/comm/set-1e4-single/txns/1e4.journal bal
rledger    report   data/comm/set-1e4-single/txns/1e4.beancount balances
bean-query          data/comm/set-1e4-single/txns/1e4.beancount 'balances from year = 2024'
````

### Journal with Tackler Audit Test Data 

Tackler can produce [cryptographic proofs of used accounting data](https://tackler.e257.fi/docs/auditing/) 
with reports. Audit mode produce test data for this. 

````bash
# Generate 10_000 (1e4) audit txns set ("journal") under `data` directory
pta-generator audit --path data --set-size 1e4 --shard-type month

# Run balance report with this test data
tackler --config data/audit/set-1e4-month.toml
````

To run full account auditing report, do following extra steps:

````bash
cd data/audit/set-1e4-month
git init .
git add .
git commit -m "Journal for year 2024"
cd ../../..

tackler --config data/audit/set-1e4-month.toml --input.storage git
````

This will produce following report:

````text
Git Storage
         commit : bd3baa9204607d8ef556dcd480ff10703ccfc168
      reference : main
      directory : txns
         suffix : .txn
        message : Journal for year 2024

Txn Set Checksum
        SHA-256 : cd00114244ec332d3aac3c301c9c9d04bb2b4e64e7b0d9e336d59cde04e7693f
       Set size : 10000

**********************************************************************************
Account Selector Checksum
           None : select all


Tackler: 1e4 (10_000) Balance Report
------------------------------------
         -13568.0000848   a:ay2024:am01
...
````

The commit id will change but Txn Set Checksum will be the same. This identifies uniquely all transactions used 
to produce this report, and git commit id will verify the content of journal. The report is calculated directly 
from repository data, the working copy is not used for this and the repository could be even bare.


## Installation

````bash
# Latest released version
cargo install --locked pta-generator

# Latest development version
cargo install --locked --git https://github.com/tackler-ng/pta-generator
````


### How to install Tackler, Ledger, HLedger and Beancount

You can install [tackler](https://tackler.e257.fi/) directly with `cargo`:

````bash
# Latest released version
cargo install --locked tackler

# Latest development version
cargo install --locked --git https://github.com/tackler-ng/tackler tackler
````

Full documentation how to install the tools can be found here:

* Tackler: [Tour of Tackler](https://tackler.e257.fi/docs/)
* Ledger: [Download](https://ledger-cli.org/download.html)
* HLedger: [Install](https://hledger.org/install.html)
* Beancount: [Installing Beancount](https://beancount.github.io/docs/installing_beancount.html)
* RustLedger: [Install](https://github.com/rustledger/rustledger/blob/main/README.md#install)


## Simple Performance Test Setup

See [Just file](./benchmark/justfile) under results for simple performance test setup
(time, memory, CPU) usage of various PTA tools.


## Design

See [Design Documentation](./docs/design.adoc) for high level description of PTA-Generator
and possible extension points.


## Contributing

See [Contributing Guidelines](./CONTRIBUTING.md) and [Developer Documention](./docs/readme.adoc)
how to work with the PTA-Generator project.


## Security

See [Security Policy](./SECURITY.md) how to report security findings.


## License

PTA-Generator is licensed under the [Apache License, version 2.0](./LICENSE).
