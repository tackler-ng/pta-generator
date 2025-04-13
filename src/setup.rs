/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use jiff::Zoned;
use jiff::fmt::strtime;
use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ShardType {
    #[default]
    Single,
    Month,
    Txn,
}

impl ShardType {
    pub const SINGLE: &'static str = "single";
    pub const MONTH: &'static str = "month";
    pub const TXN: &'static str = "txn";
}

impl TryFrom<&str> for ShardType {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            Self::SINGLE => Ok(ShardType::Single),
            Self::MONTH => Ok(ShardType::Month),
            Self::TXN => Ok(ShardType::Txn),
            _ => Err(format!(
                "Unknown shard type: {}, supported types are: {}, {}, {}",
                value,
                Self::SINGLE,
                Self::MONTH,
                Self::TXN
            )
            .into()),
        }
    }
}

impl Display for ShardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShardType::Single => write!(f, "{}", Self::SINGLE),
            ShardType::Month => write!(f, "{}", Self::MONTH),
            ShardType::Txn => write!(f, "{}", Self::TXN),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum SetSize {
    Sz1e1,
    Sz1e2,
    #[default]
    Sz1e3,
    Sz1e4,
    Sz1e5,
    Sz1e6,
}

impl SetSize {
    pub const SZ1E1: &'static str = "1e1";
    pub const SZ1E2: &'static str = "1e2";
    pub const SZ1E3: &'static str = "1e3";
    pub const SZ1E4: &'static str = "1e4";
    pub const SZ1E5: &'static str = "1e5";
    pub const SZ1E6: &'static str = "1e6";

    pub fn str(&self) -> &'static str {
        match self {
            SetSize::Sz1e1 => Self::SZ1E1,
            SetSize::Sz1e2 => Self::SZ1E2,
            SetSize::Sz1e3 => Self::SZ1E3,
            SetSize::Sz1e4 => Self::SZ1E4,
            SetSize::Sz1e5 => Self::SZ1E5,
            SetSize::Sz1e6 => Self::SZ1E6,
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            SetSize::Sz1e1 => 10,
            SetSize::Sz1e2 => 100,
            SetSize::Sz1e3 => 1_000,
            SetSize::Sz1e4 => 10_000,
            SetSize::Sz1e5 => 100_000,
            SetSize::Sz1e6 => 1_000_000,
        }
    }
}

impl TryFrom<&str> for SetSize {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ucase = value.to_lowercase();
        match ucase.as_str() {
            Self::SZ1E1 => Ok(SetSize::Sz1e1),
            Self::SZ1E2 => Ok(SetSize::Sz1e2),
            Self::SZ1E3 => Ok(SetSize::Sz1e3),
            Self::SZ1E4 => Ok(SetSize::Sz1e4),
            Self::SZ1E5 => Ok(SetSize::Sz1e5),
            Self::SZ1E6 => Ok(SetSize::Sz1e6),
            _ => Err(format!(
                "Unknown set size: {}, supported sizes are (1E1, 1E2, ...):\n{}\n{}\n{}\n{}\n{}\n{}",
                value,
                SetSize::Sz1e1, SetSize::Sz1e2, SetSize::Sz1e3, SetSize::Sz1e4, SetSize::Sz1e5, SetSize::Sz1e6,
            ).into()),
        }
    }
}

impl Display for SetSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetSize::Sz1e1 => write!(f, "1e1 (10)"),
            SetSize::Sz1e2 => write!(f, "1e2 (100)"),
            SetSize::Sz1e3 => write!(f, "1e3 (1_000)"),
            SetSize::Sz1e4 => write!(f, "1e4 (10_000)"),
            SetSize::Sz1e5 => write!(f, "1e5 (100_000)"),
            SetSize::Sz1e6 => write!(f, "1e6 (1_000_000)"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum JournalFlavor {
    #[default]
    Tackler,
    Ledger,
    Beancount,
}

impl JournalFlavor {
    pub const TACKLER: &'static str = "tackler";
    pub const LEDGER: &'static str = "ledger";
    pub const BEANCOUNT: &'static str = "beancount";
}

impl TryFrom<&str> for JournalFlavor {
    type Error = Box<dyn Error>;

    fn try_from(flavor: &str) -> Result<JournalFlavor, Self::Error> {
        match flavor {
            JournalFlavor::TACKLER => Ok(JournalFlavor::Tackler),
            JournalFlavor::LEDGER => Ok(JournalFlavor::Ledger),
            JournalFlavor::BEANCOUNT => Ok(JournalFlavor::Beancount),
            _ => Err(format!(
                "Unknown journal flavor: {}, supported flavors are: {}, {}, {}",
                flavor,
                JournalFlavor::TACKLER,
                JournalFlavor::LEDGER,
                JournalFlavor::BEANCOUNT
            )
            .into()),
        }
    }
}

impl Display for JournalFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tackler => write!(f, "{}", JournalFlavor::TACKLER),
            Self::Ledger => write!(f, "{}", JournalFlavor::LEDGER),
            Self::Beancount => write!(f, "{}", JournalFlavor::BEANCOUNT),
        }
    }
}

pub struct JournalSetup {
    pub flavor: JournalFlavor,
    pub path: PathBuf,
    pub txn_set: SetSize,
    pub shard_type: ShardType,
}

impl JournalSetup {
    pub fn try_new(
        flavor: JournalFlavor,
        path: &Path,
        set: SetSize,
        shard_type: ShardType,
    ) -> Result<Self, Box<dyn Error>> {
        if flavor != JournalFlavor::Tackler && shard_type != ShardType::Single {
            let msg = format!(
                "Shard type '{}' is not supported for journal flavor '{}'. Use shard type '{}' instead.",
                shard_type,
                flavor,
                ShardType::Single
            );
            return Err(From::from(msg));
        }
        Ok(JournalSetup {
            flavor,
            txn_set: set,
            path: path.to_path_buf(),
            shard_type: shard_type.clone(),
        })
    }
    pub fn txn_set_dir(&self) -> String {
        match self.shard_type {
            ShardType::Single => {
                // set-1e3-single
                format!("set-{}-single", self.txn_set.str())
            }
            ShardType::Month => {
                // set-1e3-month
                format!("set-{}-month", self.txn_set.str())
            }
            ShardType::Txn => {
                // set-1e3-txn
                format!("set-{}-txn", self.txn_set.str())
            }
        }
    }

    pub fn chart_of_accounts_path(&self) -> (PathBuf, PathBuf) {
        let chart_dir = self.txn_set_path().join("conf");
        let chart_path = match self.flavor {
            JournalFlavor::Tackler => chart_dir.join("accounts.toml"),
            JournalFlavor::Ledger => chart_dir.join("accounts.ledger"),
            JournalFlavor::Beancount => chart_dir.join("accounts.beancount"),
        };
        (chart_dir, chart_path)
    }
    pub fn chart_of_commodities_path(&self) -> (PathBuf, PathBuf) {
        let chart_dir = self.txn_set_path().join("conf");
        let chart_path = match self.flavor {
            JournalFlavor::Tackler => chart_dir.join("commodities.toml"),
            JournalFlavor::Ledger => chart_dir.join("commodities.ledger"),
            JournalFlavor::Beancount => chart_dir.join("commodities.beancount"),
        };
        (chart_dir, chart_path)
    }

    pub fn config_path(&self) -> (PathBuf, PathBuf) {
        let set_dir = self.txn_set_path();
        let toml_dir = set_dir.parent().expect("IE: missing parent for set_dir");
        let mut toml_path = self.txn_set_path();
        toml_path.set_extension("toml");
        (toml_dir.to_path_buf(), toml_path)
    }
    pub fn txn_set_path(&self) -> PathBuf {
        let set_dir = self.txn_set_dir();
        self.path.join(set_dir.as_str())
    }

    /// Journal path based on set, shard and flavor
    ///
    /// Set: 1e3
    /// `ShardType::Single`
    /// - tackler:   path/set-1e3-single/txns/1e3.txn
    /// - hledger:   path/set-1e3-single/txns/1e3.journal
    /// - beancount: path/set-1e3-single/txns/1e3.beancount
    ///
    /// `ShardType::Month`
    /// - tackler: path/set-1e3-month/txns/YYYY-MM.txn
    /// - ledger: Not supported
    /// - beancount: Not supported
    ///
    /// `ShardType::Txn`
    /// - tackler:  path/set-1e3-txn/YYYY/MM/DD/YYYYMMDDTHHMMSS-IDX.txn
    /// - ledger: Not supported
    /// - beancount: Not supported
    pub fn journal_path(&mut self, ts: &Zoned, idx: u32) -> (PathBuf, PathBuf) {
        match self.shard_type {
            ShardType::Single => {
                let journal = match self.flavor {
                    JournalFlavor::Tackler => format!("{}.txn", self.txn_set.str()),
                    JournalFlavor::Ledger => format!("{}.journal", self.txn_set.str()),
                    JournalFlavor::Beancount => format!("{}.beancount", self.txn_set.str()),
                };

                let txn_dir = self.txn_set_path().join("txns");
                let txn_path = txn_dir.join(journal.as_str());

                (txn_dir, txn_path)
            }
            ShardType::Month => {
                let y = ts.year();
                let m = ts.month();

                let journal = format!("{}-{:0>2}.txn", y, m);

                let txn_dir = self.txn_set_path().join("txns");
                let txn_path = txn_dir.join(journal.as_str());

                (txn_dir, txn_path)
            }
            ShardType::Txn => {
                let y = ts.year();
                let m = ts.month();
                let d = ts.day();

                let txn_shard = format!("{}/{:0>2}/{:0>2}", y, m, d);
                let ts_str = strtime::format("%Y%m%dT%H%M%S", ts).expect("IE: ts format failed");
                let journal = format!("{}-{}.txn", ts_str, idx);

                let txn_dir = self.txn_set_path().join("txns").join(txn_shard);
                let txn_path = txn_dir.join(journal.as_str());

                (txn_dir, txn_path)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::setup::SetSize;

    #[test]
    fn set_size_str() {
        assert_eq!(SetSize::Sz1e1.str(), "1e1");
        assert_eq!(SetSize::Sz1e2.str(), "1e2");
        assert_eq!(SetSize::Sz1e3.str(), "1e3");
        assert_eq!(SetSize::Sz1e4.str(), "1e4");
        assert_eq!(SetSize::Sz1e5.str(), "1e5");
        assert_eq!(SetSize::Sz1e6.str(), "1e6");
    }

    #[test]
    fn set_size_size() {
        assert_eq!(SetSize::Sz1e1.size(), 10);
        assert_eq!(SetSize::Sz1e2.size(), 100);
        assert_eq!(SetSize::Sz1e3.size(), 1_000);
        assert_eq!(SetSize::Sz1e4.size(), 10_000);
        assert_eq!(SetSize::Sz1e5.size(), 100_000);
        assert_eq!(SetSize::Sz1e6.size(), 1_000_000);
    }
}
