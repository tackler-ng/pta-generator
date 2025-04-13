/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::cli_args::GlobalArgs;
use crate::generators::beancount::Beancount;
use crate::generators::ledger::Ledger;
use crate::generators::tackler::Tackler;
use crate::generators::{ChartOfAccGenerator, ChartOfCommGenerator, CommodityTxnGenerator};
use crate::setup::{JournalFlavor, SetSize, ShardType};
use crate::writers::JournalWriter;
use jiff::{Span, Unit};
use std::collections::BTreeSet;
use std::fmt::Write;
use std::ops::Add;

pub fn exec(
    args: GlobalArgs,
    flavor: Option<String>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let flavor = flavor.map_or(Ok(JournalFlavor::default()), |f| {
        JournalFlavor::try_from(f.as_str())
    })?;

    let set = SetSize::try_from(args.set_size.as_str())?;
    let shard_type = ShardType::try_from(args.shard_type.as_str())?;
    let ts_start = args.start_ts()?;
    let ts_end = args.stop_ts()?;

    let span_secs = (&ts_end - &ts_start).total(Unit::Second)? as u32;
    let step = Span::new().seconds(span_secs / set.size());

    let mut writer = JournalWriter::try_new(
        flavor.clone(),
        args.data_path.join("comm").as_path(),
        set.clone(),
        shard_type,
    )?;

    let txn_generator = match flavor {
        JournalFlavor::Tackler => Tackler::commodity_txn,
        JournalFlavor::Ledger => Ledger::commodity_txn,
        JournalFlavor::Beancount => Beancount::commodity_txn,
    };

    let mut ts = ts_start;
    match flavor {
        JournalFlavor::Tackler => {}
        JournalFlavor::Ledger => {
            // Only single file shard mode is supported for ledger
            let journal_path = writer.setup.chart_of_accounts_path();
            let accs = format!(
                "include ../conf/{}\n\n",
                journal_path.1.file_name().unwrap(/*:ok:*/).to_str().unwrap(/*:ok:*/)
            );
            writer.write_txn(&ts, 0, accs.as_str())?;
        }
        JournalFlavor::Beancount => {
            // Only single file shard mode is supported for beancount
            let journal_path = writer.setup.chart_of_accounts_path();
            let accs = format!(
                "include \"../conf/{}\"\n\n",
                journal_path.1.file_name().unwrap(/*:ok:*/).to_str().unwrap(/*:ok:*/)
            );
            writer.write_txn(&ts, 0, accs.as_str())?;
        }
    }
    let mut accounts = BTreeSet::new();
    let mut commodities = BTreeSet::new();
    for idx in 1..=set.size() {
        let txn = txn_generator(&set, &ts, idx)?;

        for a in txn.1.accounts {
            accounts.insert(a);
        }
        for a in txn.1.commodities {
            commodities.insert(a);
        }

        writer.write_txn(&ts, idx, txn.0.as_str())?;

        ts = ts.add(step);
    }

    match flavor {
        JournalFlavor::Tackler => {
            writer.write_config(Tackler::config(true, false, &writer.setup)?.as_str())?;
            writer.write_chart_of_accounts(Tackler::chart_of_accounts(&accounts)?.as_str())?;
            writer.write_chart_of_commodities(
                Tackler::chart_of_commodities(&commodities)?.as_str(),
            )?;

            let mut msg = String::new();
            writeln!(msg, "Created {} test set", writer.setup.txn_set)?;
            writeln!(
                msg,
                "Test set is located at: {}",
                writer.setup.txn_set_path().display()
            )?;
            writeln!(msg, "You can test it with command:\n")?;
            writeln!(
                msg,
                "   tackler --config {}",
                writer.setup.config_path().1.display()
            )?;
            Ok(Some(msg))
        }
        JournalFlavor::Ledger => {
            writer.write_chart_of_accounts(Ledger::chart_of_accounts(&accounts)?.as_str())?;

            let mut msg = String::new();
            writeln!(msg, "Created {} test set", writer.setup.txn_set)?;
            writeln!(
                msg,
                "Test journal is located at: {}/txns",
                writer.setup.txn_set_path().display()
            )?;
            Ok(Some(msg))
        }
        JournalFlavor::Beancount => {
            writer.write_chart_of_accounts(Beancount::chart_of_accounts(&accounts)?.as_str())?;

            let mut msg = String::new();
            writeln!(msg, "Created {} test set", writer.setup.txn_set)?;
            writeln!(
                msg,
                "Test journal is located at: {}/txns",
                writer.setup.txn_set_path().display()
            )?;
            Ok(Some(msg))
        }
    }
}
