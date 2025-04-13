/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::setup::SetSize;
use jiff::Zoned;
use std::collections::BTreeSet;
use std::error::Error;

pub mod beancount;
pub mod ledger;
pub mod tackler;

pub struct TxnAccComm {
    pub accounts: Vec<String>,
    pub commodities: Vec<String>,
}

/// Get commodity name based on day
pub fn commodity_name(ts: &Zoned) -> &'static str {
    #[rustfmt::skip]
    let comm_names = vec![
        "CAA", "CAB", "CAC", "CAD", "CAE",
        "CBA", "CBB", "CBC", "CBD", "CBE",
        "CCA", "CCB", "CCC", "CCD", "CCE",
        "CDA", "CDB", "CDC", "CDD", "CDE",
        "CEA", "CEB", "CEC", "CED", "CEE",
        "CFA", "CFB", "CFC", "CFD", "CFE",
        "EUR"
    ];
    comm_names[(ts.day() - 1) as usize]
}

/// Chart of Accounts
pub trait ChartOfAccGenerator {
    fn chart_of_accounts(accounts: &BTreeSet<String>) -> Result<String, Box<dyn Error>>;
}

/// Chart of Commodities
pub trait ChartOfCommGenerator {
    fn chart_of_commodities(commodities: &BTreeSet<String>) -> Result<String, Box<dyn Error>>;
}

/// Audit Txn Generator
pub trait AuditTxnGenerator: ChartOfAccGenerator {
    fn audit_txn(
        set: &SetSize,
        ts: &Zoned,
        index: u32,
    ) -> Result<(String, TxnAccComm), Box<dyn Error>>;
}

/// Plain txn, no extra
pub trait PlainTxnGenerator {
    fn plain_txn(set: &SetSize, ts: &Zoned, index: u32) -> Result<String, Box<dyn Error>>;
}

/// Transactions with commodities, Charts of accounts and commodities
pub trait CommodityTxnGenerator: ChartOfAccGenerator + ChartOfCommGenerator {
    fn commodity_txn(
        set: &SetSize,
        ts: &Zoned,
        index: u32,
    ) -> Result<(String, TxnAccComm), Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use crate::generators::commodity_name;
    use jiff::Timestamp;
    use jiff::tz::TimeZone;

    #[test]
    fn test_commodity() {
        let ts: Timestamp = "2024-12-01T23:58:00Z".parse().unwrap();
        assert_eq!(commodity_name(&ts.to_zoned(TimeZone::UTC)), "CAA");

        let ts: Timestamp = "2024-12-15T23:58:00Z".parse().unwrap();
        assert_eq!(commodity_name(&ts.to_zoned(TimeZone::UTC)), "CCE");

        let ts: Timestamp = "2024-12-31T23:58:00Z".parse().unwrap();
        assert_eq!(commodity_name(&ts.to_zoned(TimeZone::UTC)), "EUR");
    }
}
