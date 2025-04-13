/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::generators::{
    ChartOfAccGenerator, ChartOfCommGenerator, CommodityTxnGenerator, PlainTxnGenerator,
    TxnAccComm, commodity_name,
};
use crate::setup::SetSize;
use jiff::Zoned;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::Write;

pub struct Ledger;

impl PlainTxnGenerator for Ledger {
    fn plain_txn(set: &SetSize, ts_tz: &Zoned, index: u32) -> Result<String, Box<dyn Error>> {
        let y = ts_tz.year();
        let m = ts_tz.month();
        let d = ts_tz.day();

        let assets_acc = format!("a:ay{:0>4}:am{:0>2}", y, m);
        let expenses_acc = format!("e:ey{:0>4}:em{:0>2}:ed{:0>2}", y, m, d);

        let mut txn = String::new();
        writeln!(
            txn,
            "{:0>4}/{:0>2}/{:0>2} (#{:0>7}) {} txn-{index}",
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

impl ChartOfAccGenerator for Ledger {
    fn chart_of_accounts(accounts: &BTreeSet<String>) -> Result<String, Box<dyn Error>> {
        let mut chart = String::with_capacity(accounts.len() * 100);

        for acc in accounts.iter() {
            writeln!(chart, "account {acc}")?;
        }
        Ok(chart)
    }
}

impl ChartOfCommGenerator for Ledger {
    fn chart_of_commodities(_commodities: &BTreeSet<String>) -> Result<String, Box<dyn Error>> {
        // This is not needed at the moment
        unimplemented!()
    }
}

impl CommodityTxnGenerator for Ledger {
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
            "{:0>4}/{:0>2}/{:0>2} (#{:0>7}) {} txn-{index}",
            y,
            m,
            d,
            index,
            set.str().to_uppercase()
        )?;
        writeln!(txn, "  {expenses_acc}  {d}.0000001 {commodity}")?;
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

#[cfg(test)]
mod tests {
    use crate::generators::ledger::Ledger;
    use crate::generators::{CommodityTxnGenerator, PlainTxnGenerator};
    use crate::setup::SetSize;
    use jiff::Timestamp;
    use jiff::tz::TimeZone;

    #[test]
    fn test_plain() {
        let ts: Timestamp = "2024-12-31T23:58:00Z".parse().unwrap(/*:test:*/);
        let txn =
            Ledger::plain_txn(&SetSize::Sz1e3, &ts.to_zoned(TimeZone::UTC), 999).unwrap(/*:test:*/);
        let txn_str = "2024/12/31 (#0000999) 1E3 txn-999
  e:ey2024:em12:ed31  31.0000001
  a:ay2024:am12

";
        assert_eq!(txn, txn_str);
    }

    #[test]
    fn test_commodity() {
        let ts: Timestamp = "2024-12-31T23:58:00Z".parse().unwrap(/*:test:*/);
        let txn = Ledger::commodity_txn(&SetSize::Sz1e3, &ts.to_zoned(TimeZone::UTC), 999).unwrap(/*:test:*/);
        let txn_str = "2024/12/31 (#0000999) 1E3 txn-999
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
