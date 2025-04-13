/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::cli_args::GlobalArgs;
use crate::generators::PlainTxnGenerator;
use crate::generators::ledger::Ledger;
use crate::generators::tackler::Tackler;
use crate::setup::{JournalFlavor, SetSize, ShardType};
use crate::writers::JournalWriter;
use jiff::{Span, Unit};
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
        args.data_path.join("plain").as_path(),
        set.clone(),
        shard_type,
    )?;

    let txn_generator = match flavor {
        JournalFlavor::Tackler => Tackler::plain_txn,
        JournalFlavor::Ledger => Ledger::plain_txn,
        JournalFlavor::Beancount => {
            let msg =
                "Plain mode for Beancount is not implemented, use 'comm' mode instead".to_string();
            return Err(msg.into());
        }
    };

    let mut ts = ts_start;
    for idx in 1..=set.size() {
        let txn = txn_generator(&set, &ts, idx)?;

        writer.write_txn(&ts, idx, txn.as_str())?;

        ts = ts.add(step);
    }

    match flavor {
        JournalFlavor::Tackler => {
            writer.write_config(Tackler::config(false, false, &writer.setup)?.as_str())?;

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
            let msg =
                "Plain mode for Beancount is not implemented, use 'comm' mode instead".to_string();
            Err(msg.into())
        }
    }
}
