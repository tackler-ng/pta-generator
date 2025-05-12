/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::setup::{JournalFlavor, ShardType};
use clap::builder::PossibleValue;
use clap::{Parser, Subcommand};
use jiff::Zoned;
use jiff::civil::date;
use jiff::tz::TimeZone;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version=env!("VERSION"), about, long_about = None)]
#[command(propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn cmd(&self) -> Commands {
        self.command.clone()
    }
}

#[derive(Clone, Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    /// Generate journal for audit testing
    ///
    /// This will use stable predictable UUID for
    /// each transaction, so that Txn Set Checksum will be stay
    /// same for the same generated txn sets.
    ///
    /// Supported flavors: tackler
    Audit {
        #[clap(flatten)]
        global_args: GlobalArgs,

        /// Flavor of Journal
        #[arg(long,
            value_parser([
                PossibleValue::new(JournalFlavor::TACKLER),
            ]),
        )]
        flavor: Option<String>,
    },

    /// Generate journal in the simplest form
    ///
    /// The simplest journal, could be used as baseline for `audit` and `comm`
    /// Beancount IS NOT supported, as it needs always Chart of Accounts.
    ///
    /// Supported flavors: tackler, (h)ledger
    Plain {
        #[clap(flatten)]
        global_args: GlobalArgs,

        /// Flavor of Journal
        #[arg(long,
            value_parser([
                PossibleValue::new(JournalFlavor::TACKLER),
                PossibleValue::new(JournalFlavor::LEDGER),
            ]),
        )]
        flavor: Option<String>,
    },

    /// Generate journal with commodities
    ///
    /// Journal with commodities
    ///
    /// Supported flavors: tackler, (h)ledger, beancount
    Comm {
        #[clap(flatten)]
        global_args: GlobalArgs,

        /// Flavor of Journal
        #[arg(long,
            value_parser([
                PossibleValue::new(JournalFlavor::TACKLER),
                PossibleValue::new(JournalFlavor::LEDGER),
                PossibleValue::new(JournalFlavor::BEANCOUNT),
            ]),
        )]
        flavor: Option<String>,
    },
}

#[allow(clippy::doc_overindented_list_items)]
#[derive(Debug, Clone, clap::Args)]
pub(crate) struct GlobalArgs {
    /// Path to output directory
    ///
    /// Under this path there will be a folder named as
    /// `set-<SIZE>-<SHARD_TYPE>`
    #[arg(long = "path", value_name = "path/to/output_directory")]
    pub data_path: PathBuf,

    /// How to store (shard) the transaction journal
    ///
    /// - single: Use single journal file
    ///           path(tackler):   'PATH/set-SET-single/txns/SET-journal.txn
    ///           path(h/ledger):  'PATH/set-SET-single/txns/SET.journal
    ///           path(beancount): 'PATH/set-SET-single/txns/SET.beancount
    /// - month:  Use 12 sub journals (shards) based on month of txn
    ///           path: 'PATH/set-SET-month/txns/YYYY/MM/YYYY-MM.txn
    /// - txn:    Use own file for each transaction, shard is based on txn timestamp
    ///           path: 'PATH/set-SET-txn/txns/YYYY/MM/DD/YYYYMMDDTHHMMSS-IDX.txn'
    #[arg(long, verbatim_doc_comment,
        value_parser([
                PossibleValue::new(ShardType::SINGLE),
                PossibleValue::new(ShardType::MONTH),
                PossibleValue::new(ShardType::TXN),
            ]),
    )]
    pub shard_type: String,

    /// How many transactions to generate (1e1, 1e2, ...)
    ///
    /// Supported set sizes are:
    ///   1e1 = 10
    ///   1e2 = 100
    ///   1e3 = 1_000
    ///   1e4 = 10_000
    ///   1e5 = 100_000
    ///   1e6 = 1_000_000
    #[arg(long, verbatim_doc_comment)]
    pub set_size: String,

    /// Optional start time in RFC-9557 format
    ///
    /// Default is: 2024-01-01T00:00:00+00:00[UTC]
    ///
    /// If used, must be with `stop`
    #[arg(long, value_name = "START_TIMESTAMP", requires = "stop")]
    start: Option<String>,

    /// Optional stop time in RFC-9557 format
    ///
    /// Default is: 2025-01-01T00:00:00+00:00[UTC]
    ///
    /// If used, must be with `start`
    #[arg(long, value_name = "STOP_TIMESTAMP", requires = "start")]
    stop: Option<String>,
}

impl GlobalArgs {
    pub fn start_ts(&self) -> Result<Zoned, jiff::Error> {
        self.start.as_ref().map_or(
            date(2024, 1, 1).at(0, 0, 0, 0).to_zoned(TimeZone::UTC),
            |d| d.parse(),
        )
    }
    pub fn stop_ts(&self) -> Result<Zoned, jiff::Error> {
        self.stop.as_ref().map_or(
            date(2025, 1, 1).at(0, 0, 0, 0).to_zoned(TimeZone::UTC),
            |d| d.parse(),
        )
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
