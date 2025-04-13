/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::cli_args::GlobalArgs;
use crate::generators::tackler::Tackler;
use crate::generators::{AuditTxnGenerator, ChartOfAccGenerator, ChartOfCommGenerator};
use crate::setup::{JournalFlavor, SetSize, ShardType};
use crate::writers::JournalWriter;
use jiff::{Span, Unit};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::Write;
use std::ops::Add;

pub fn exec(args: GlobalArgs) -> Result<Option<String>, Box<dyn Error>> {
    let flavor = JournalFlavor::default();
    let set = SetSize::try_from(args.set_size.as_str())?;
    let shard_type = ShardType::try_from(args.shard_type.as_str())?;
    let ts_start = args.start_ts()?;
    let ts_end = args.stop_ts()?;

    let span_secs = (&ts_end - &ts_start).total(Unit::Second)? as u32;
    let step = Span::new().seconds(span_secs / set.size());

    let mut writer = JournalWriter::try_new(
        flavor,
        args.data_path.join("audit").as_path(),
        set.clone(),
        shard_type,
    )?;
    let mut accounts = BTreeSet::new();
    let mut commodities = BTreeSet::new();

    let mut ts = ts_start;
    for idx in 1..=set.size() {
        let txn = Tackler::audit_txn(&set, &ts, idx)?;

        writer.write_txn(&ts, idx, txn.0.as_str())?;

        for a in txn.1.accounts {
            accounts.insert(a);
        }
        for a in txn.1.commodities {
            commodities.insert(a);
        }

        ts = ts.add(step);
    }

    writer.write_chart_of_accounts(Tackler::chart_of_accounts(&accounts)?.as_str())?;
    writer.write_chart_of_commodities(Tackler::chart_of_commodities(&commodities)?.as_str())?;

    writer.write_config(Tackler::config(true, true, &writer.setup)?.as_str())?;

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
