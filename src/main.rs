// SPDX-License-Identifier: BSD-3-Clause

use clap::{Args, Parser};
use human_units::Size;
use std::path::PathBuf;

/// Split a set of files into fixed size chunks and optionally transfer them to a remote host.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// OpenSSH private key file to authenticate remote SSH connections
    #[arg(short)]
    identity_file: Option<PathBuf>,

    /// size of the split chunks
    #[arg(short, default_value = "3M")]
    size: Size,

    /// number of parallel SSH connections to transfer with
    #[arg(short, default_value = "5")]
    parallel: u16,

    #[command(flatten)]
    output: CliArgsOutput,

    /// Files to split and transfer
    files: Vec<PathBuf>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct CliArgsOutput {
    /// Output split chunks to local directory
    #[arg(short)]
    output: Option<PathBuf>,

    /// Transfer split chunks to remote location
    #[arg(short)]
    remote: Option<String>,
}

fn main() {
    let args = CliArgs::parse();
    println!("Arguments: {:?}", args);
}
