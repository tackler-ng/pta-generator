/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::cli_args::Commands;
use crate::commands::{audit, comm, plain};
use clap::Parser;

mod cli_args;
mod commands;
mod generators;
mod setup;
mod txn_uuid;
mod writers;

fn main() {
    let cli = cli_args::Cli::parse();

    #[rustfmt::skip]
    let res = match cli.cmd() {
        Commands::Audit {
            global_args,
            flavor: _,
        } => {
            audit::exec(global_args)
        },
        Commands::Plain {
            global_args,
            flavor,
        } => {
            plain::exec(global_args, flavor)
        },
        Commands::Comm {
            global_args,
            flavor,
        } => {
            comm::exec(global_args, flavor)
        },
    };

    match res {
        Ok(msg) => {
            if let Some(msg) = msg {
                println!("{}", msg);
            }
            std::process::exit(0)
        }
        Err(err) => {
            let msg = format!("PTA-Generator error: {err}");
            eprintln!("{msg}");
            std::process::exit(1)
        }
    }
}
