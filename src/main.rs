// rcli csv -i input.csv -o output.json
use anyhow::Result;
use clap::Parser;

use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?
        }
        SubCommand::GenPass(genpass_opts) => process_genpass(
            genpass_opts.length,
            genpass_opts.uppercase,
            genpass_opts.lowercase,
            genpass_opts.number,
            genpass_opts.symbol,
        )?,
        SubCommand::Base64(base64_opts) => match base64_opts {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
