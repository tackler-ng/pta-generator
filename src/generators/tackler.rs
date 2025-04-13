/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::generators::{
    AuditTxnGenerator, ChartOfAccGenerator, ChartOfCommGenerator, CommodityTxnGenerator,
    PlainTxnGenerator, TxnAccComm, commodity_name,
};
use crate::setup::{JournalSetup, SetSize};
use crate::txn_uuid::get_txn_uuid;
use jiff::Zoned;
use jiff::fmt::strtime;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::Write;

pub struct Tackler {}

impl ChartOfAccGenerator for Tackler {
    fn chart_of_accounts(accounts: &BTreeSet<String>) -> Result<String, Box<dyn Error>> {
        let mut chart = String::with_capacity(accounts.len() * 100);

        writeln!(chart, "accounts = [")?;
        for acc in accounts.iter() {
            writeln!(chart, "   \"{acc}\",")?;
        }
        writeln!(chart, "]")?;
        Ok(chart)
    }
}

impl ChartOfCommGenerator for Tackler {
    fn chart_of_commodities(commodities: &BTreeSet<String>) -> Result<String, Box<dyn Error>> {
        let mut chart = String::with_capacity(commodities.len() * 16);

        writeln!(chart, "permit-empty-commodity = true")?;
        writeln!(chart, "commodities = [")?;
        for comm in commodities.iter() {
            writeln!(chart, "   \"{comm}\",")?;
        }
        writeln!(chart, "]")?;
        Ok(chart)
    }
}

impl AuditTxnGenerator for Tackler {
    fn audit_txn(
        set: &SetSize,
        ts_tz: &Zoned,
        index: u32,
    ) -> Result<(String, TxnAccComm), Box<dyn Error>> {
        let ts = ts_tz.timestamp();
        let ts_tz_str = strtime::format("%Y-%m-%dT%H:%M:%S%.f%:z", ts_tz)?;

        let uuid = get_txn_uuid(set, index, ts, Some("audit"));

        let y = ts_tz.year();
        let m = ts_tz.month();
        let d = ts_tz.day();

        let assets_acc = format!("a:ay{:0>4}:am{:0>2}", y, m);
        let expenses_acc = format!("e:ey{:0>4}:em{:0>2}:ed{:0>2}", y, m, d);

        let mut txn = String::new();
        writeln!(
            txn,
            "{ts_tz_str} (#{:0>7}) '{} txn-{index}",
            index,
            set.str().to_uppercase()
        )?;
        writeln!(txn, "  # uuid: {uuid}")?;
        writeln!(txn, "  {expenses_acc}  {d}.0000001")?;
        writeln!(txn, "  {assets_acc}")?;
        writeln!(txn)?;

        Ok((
            txn,
            TxnAccComm {
                accounts: vec![assets_acc, expenses_acc],
                commodities: vec![],
            },
        ))
    }
}

impl PlainTxnGenerator for Tackler {
    fn plain_txn(set: &SetSize, ts_tz: &Zoned, index: u32) -> Result<String, Box<dyn Error>> {
        let y = ts_tz.year();
        let m = ts_tz.month();
        let d = ts_tz.day();

        let assets_acc = format!("a:ay{:0>4}:am{:0>2}", y, m);
        let expenses_acc = format!("e:ey{:0>4}:em{:0>2}:ed{:0>2}", y, m, d);

        let mut txn = String::new();
        writeln!(
            txn,
            "{:0>4}-{:0>2}-{:0>2} (#{:0>7}) '{} txn-{index}",
            y,
            m,
            d,
            index,
            set.str().to_uppercase()
        )?;
        writeln!(txn, "  {expenses_acc}  {d}.0000001")?;
        writeln!(txn, "  {assets_acc}")?;
        writeln!(txn)?;

        Ok(txn)
    }
}

impl CommodityTxnGenerator for Tackler {
    fn commodity_txn(
        set: &SetSize,
        ts_tz: &Zoned,
        index: u32,
    ) -> Result<(String, TxnAccComm), Box<dyn Error>> {
        let y = ts_tz.year();
        let m = ts_tz.month();
        let d = ts_tz.day();

        let assets_acc = format!("Assets:Ay{:0>4}:Am{:0>2}", y, m);
        let expenses_acc = format!("Expenses:Ey{:0>4}:Em{:0>2}:Ed{:0>2}", y, m, d);
        let commodity = commodity_name(ts_tz).to_string();

        let mut txn = String::new();
        writeln!(
            txn,
            "{:0>4}-{:0>2}-{:0>2} (#{:0>7}) '{} txn-{index}",
            y,
            m,
            d,
            index,
            set.str().to_uppercase()
        )?;
        writeln!(txn, "  {expenses_acc}  {d}.0000001 {commodity}",)?;
        writeln!(txn, "  {assets_acc}")?;
        writeln!(txn)?;

        Ok((
            txn,
            TxnAccComm {
                accounts: vec![assets_acc, expenses_acc],
                commodities: vec![commodity],
            },
        ))
    }
}

impl Tackler {
    pub fn config(
        strict_mode: bool,
        audit_mode: bool,
        setup: &JournalSetup,
    ) -> Result<String, Box<dyn Error>> {
        let accounts_toml = if strict_mode {
            format!("{}/conf/accounts.toml", setup.txn_set_dir())
        } else {
            "none".to_string()
        };
        let commodities_toml = if strict_mode {
            format!("{}/conf/commodities.toml", setup.txn_set_dir())
        } else {
            "none".to_string()
        };

        let toml = format!(
            r##"#
[kernel]
strict = {strict_mode}
audit = {{ mode = {audit_mode}, hash = "SHA-256" }}
timestamp = {{ default-time = 00:00:00, timezone = {{ name = "UTC" }} }}

[kernel.input]
storage = "fs"
fs  = {{ path = "{txn_set_dir}",      dir = "txns", suffix = "txn" }}
git = {{ repo = "{txn_set_dir}/.git", dir = "txns", suffix = "txn", ref = "main" }}

[transaction]
accounts    = {{ path = "{accounts_toml}" }}
commodities = {{ path = "{commodities_toml}" }}
tags        = {{ path = "none" }}

[report]
report-timezone = "UTC"
scale = {{ min = 2, max = 7 }}
accounts = [ ]
targets = [ "balance" ]

balance       = {{ title = "Tackler: {txn_set} Balance Report", type = "flat" }}
balance-group = {{ title = "Tackler: {txn_set} Balance Group Report", type = "flat", group-by = "month" }}
register      = {{ title = "Tackler: {txn_set} Register Report" }}

[export]
targets = [ ]
equity = {{ accounts = [ "Assets(:.*)?", ], equity-account = "Equity:Balance" }}
"##,
            txn_set_dir = setup.txn_set_dir(),
            txn_set = setup.txn_set.to_string(),
        );
        Ok(toml)
    }
}

#[cfg(test)]
mod tests {
    use crate::generators::tackler::Tackler;
    use crate::generators::{AuditTxnGenerator, CommodityTxnGenerator, PlainTxnGenerator};
    use crate::setup::SetSize;
    use jiff::Timestamp;
    use jiff::tz::TimeZone;

    #[test]
    fn test_audit() {
        let ts: Timestamp = "2024-12-31T23:58:00Z".parse().unwrap(/*:test:*/);
        let txn = Tackler::audit_txn(&SetSize::Sz1e3, &ts.to_zoned(TimeZone::UTC), 999).unwrap(/*:test:*/);
        let txn_str = "2024-12-31T23:58:00+00:00 (#0000999) '1E3 txn-999
  # uuid: 8e43c795-8fb1-552e-9dde-eae36f233676
  e:ey2024:em12:ed31  31.0000001
  a:ay2024:am12

";
        assert_eq!(txn.0, txn_str);
    }

    #[test]
    fn test_plain() {
        let ts: Timestamp = "2024-12-31T23:58:00Z".parse().unwrap(/*:test:*/);
        let txn = Tackler::plain_txn(&SetSize::Sz1e3, &ts.to_zoned(TimeZone::UTC), 999).unwrap(/*:test:*/);
        let txn_str = "2024-12-31 (#0000999) '1E3 txn-999
  e:ey2024:em12:ed31  31.0000001
  a:ay2024:am12

";
        assert_eq!(txn, txn_str);
    }

    #[test]
    fn test_commodity() {
        let ts: Timestamp = "2024-12-31T23:58:00Z".parse().unwrap(/*:test:*/);
        let txn = Tackler::commodity_txn(&SetSize::Sz1e3, &ts.to_zoned(TimeZone::UTC), 999).unwrap(/*:test:*/);
        let txn_str = "2024-12-31 (#0000999) '1E3 txn-999
  Expenses:Ey2024:Em12:Ed31  31.0000001 EUR
  Assets:Ay2024:Am12

";
        assert_eq!(txn.0, txn_str);
        assert_eq!(
            txn.1.accounts,
            vec![
                "Assets:Ay2024:Am12".to_string(),
                "Expenses:Ey2024:Em12:Ed31".to_string()
            ]
        );
        assert_eq!(txn.1.commodities, vec!["EUR".to_string()]);
    }
}
