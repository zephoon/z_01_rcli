// rcli csv -i input.csv -o output.json
use anyhow::Result;
use clap::Parser;

use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => process_csv(&csv_opts.input, &csv_opts.output)?,
    }
    Ok(())
}
