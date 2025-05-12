/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::setup::JournalSetup;
use crate::setup::{JournalFlavor, SetSize, ShardType};
use jiff::Zoned;
use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::{fs, io};

type RefWriter = Rc<RefCell<Box<dyn io::Write>>>;

pub struct JournalWriter {
    pub setup: JournalSetup,
    writers: Vec<Option<RefWriter>>,
}

impl JournalWriter {
    /// Fails if target path exists
    pub fn try_new(
        flavor: JournalFlavor,
        path: &Path,
        set: SetSize,
        shard_type: ShardType,
    ) -> Result<JournalWriter, Box<dyn Error>> {
        let setup = JournalSetup::try_new(flavor, path, set, shard_type.clone())?;
        let w = JournalWriter {
            setup,
            writers: vec![None; 12],
        };

        if shard_type != ShardType::Single && w.setup.txn_set_path().exists() {
            let msg = format!(
                "Target path already exists: '{}'",
                w.setup.txn_set_path().display()
            );
            return Err(msg.into());
        }
        Ok(w)
    }

    /// Makes writer for path and creates any missing directories
    /// Errors if path exists
    fn make_writer(dir: &Path, txn_path: &Path) -> Result<Box<dyn io::Write>, Box<dyn Error>> {
        fs::create_dir_all(dir)?;
        let f = match File::create_new(txn_path) {
            Ok(f) => f,
            Err(err) => {
                let msg = format!("{}: '{}'", err, txn_path.to_string_lossy());
                return Err(msg.into());
            }
        };
        let bw = BufWriter::new(f);
        Ok(Box::new(bw))
    }

    fn journal_writer(&mut self, ts: &Zoned, idx: u32) -> Result<RefWriter, Box<dyn Error>> {
        match self.setup.shard_type {
            ShardType::Single => {
                if let Some(w) = &self.writers[0] {
                    Ok(w.clone())
                } else {
                    let txn_path = self.setup.journal_path(ts, idx);
                    let w = Self::make_writer(&txn_path.0, &txn_path.1)?;
                    let rcw = Rc::new(RefCell::new(w));
                    self.writers[0] = Some(rcw.clone());

                    Ok(rcw)
                }
            }
            ShardType::Month => {
                let m = ts.month() - 1;
                if let Some(w) = &self.writers[m as usize] {
                    Ok(w.clone())
                } else {
                    let txn_path = self.setup.journal_path(ts, idx);
                    let w = Self::make_writer(&txn_path.0, &txn_path.1)?;

                    let rcw = Rc::new(RefCell::new(w));
                    self.writers[m as usize] = Some(rcw.clone());

                    Ok(rcw)
                }
            }
            ShardType::Txn => {
                let txn_path = self.setup.journal_path(ts, idx);
                let w = Self::make_writer(&txn_path.0, &txn_path.1)?;

                let rcw = Rc::new(RefCell::new(w));
                Ok(rcw)
            }
        }
    }
    pub fn write_txn(&mut self, ts: &Zoned, idx: u32, txn: &str) -> Result<(), Box<dyn Error>> {
        let w = self.journal_writer(ts, idx)?;
        Ok(write!(w.borrow_mut(), "{}", txn)?)
    }
    pub fn write_chart_of_accounts(&mut self, chart: &str) -> Result<(), Box<dyn Error>> {
        let (chart_dir, chart_path) = self.setup.chart_of_accounts_path();
        let mut w = Self::make_writer(&chart_dir, &chart_path)?;

        Ok(write!(w, "{}", chart)?)
    }

    pub fn write_chart_of_commodities(&mut self, chart: &str) -> Result<(), Box<dyn Error>> {
        let (chart_dir, chart_path) = self.setup.chart_of_commodities_path();
        let mut w = Self::make_writer(&chart_dir, &chart_path)?;

        Ok(write!(w, "{}", chart)?)
    }

    pub fn write_config(&mut self, config: &str) -> Result<(), Box<dyn Error>> {
        let (toml_dir, toml_path) = self.setup.config_path();
        let mut w = Self::make_writer(&toml_dir, &toml_path)?;

        Ok(write!(w, "{}", config)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_set_dir() {
        // UUID as part of path to make it unique, so JournalWriter::try_new won't fail
        let base_path = Path::new("target/cbc1a015-a250-4041-ba81-21a26a414a4b");

        // Test for ShardType::Single
        let writer_single = JournalWriter::try_new(JournalFlavor::Ledger, base_path, SetSize::Sz1e3, ShardType::Single).unwrap(/*:test:*/);
        assert_eq!(
            writer_single.setup.txn_set_path(),
            base_path.join("set-1e3-single")
        );

        // Test for ShardType::Month
        let writer_month = JournalWriter::try_new(JournalFlavor::Tackler, base_path, SetSize::Sz1e5, ShardType::Month).unwrap(/*:test:*/);
        assert_eq!(
            writer_month.setup.txn_set_path(),
            base_path.join("set-1e5-month")
        );

        // Test for ShardType::Txn
        let writer_txn = JournalWriter::try_new(JournalFlavor::Tackler, base_path, SetSize::Sz1e6, ShardType::Txn).unwrap(/*:test:*/);
        assert_eq!(
            writer_txn.setup.txn_set_path(),
            base_path.join("set-1e6-txn")
        );
    }

    #[test]
    fn test_journal_path() {
        // UUID as part of path to make it unique, so JournalWriter::try_new won't fail
        let base_path = Path::new("target/746c112a-b6ad-44f4-b8f2-f2bb92953630");
        let ts = ("2025-04-12T12:34:56+00:00[UTC]")
            .parse()
            .expect("Failed to parse timestamp");

        // Test for ShardType::Single
        let mut writer_single = JournalWriter::try_new(JournalFlavor::Ledger, base_path, SetSize::Sz1e3, ShardType::Single).unwrap(/*:test:*/);
        let (dir_single, path_single) = writer_single.setup.journal_path(&ts, 0);
        assert_eq!(dir_single, base_path.join("set-1e3-single/txns"));
        assert_eq!(
            path_single,
            base_path.join("set-1e3-single/txns/1e3.journal")
        );

        // Test for ShardType::Month
        let mut writer_month = JournalWriter::try_new(JournalFlavor::Tackler, base_path, SetSize::Sz1e5, ShardType::Month).unwrap(/*:test:*/);
        let (dir_month, path_month) = writer_month.setup.journal_path(&ts, 0);
        assert_eq!(dir_month, base_path.join("set-1e5-month/txns/2025/04"));
        assert_eq!(
            path_month,
            base_path.join("set-1e5-month/txns/2025/04/2025-04.txn")
        );

        // Test for ShardType::Txn
        let mut writer_txn = JournalWriter::try_new(JournalFlavor::Tackler, base_path, SetSize::Sz1e6, ShardType::Txn).unwrap(/*:test:*/);
        let (dir_txn, path_txn) = writer_txn.setup.journal_path(&ts, 333);
        assert_eq!(dir_txn, base_path.join("set-1e6-txn/txns/2025/04/12"));
        assert_eq!(
            path_txn,
            base_path.join("set-1e6-txn/txns/2025/04/12/20250412T123456-333.txn")
        );
    }
}
