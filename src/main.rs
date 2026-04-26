// SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
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

    /// Files to split and transfer
    files: Vec<PathBuf>,

    /// Destination to transfer split chunks to
    #[arg(last = true)]
    dest: String,
}

fn main() {
    let args = CliArgs::parse();
    println!("Arguments: {:?}", args);
}
