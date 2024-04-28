use std::fs;

// rcli csv -i input.csv -o output.json
use anyhow::Result;
use clap::Parser;

use rcli::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_text_sign, process_text_verify, Base64SubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};

use zxcvbn::zxcvbn;

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
        SubCommand::GenPass(genpass_opts) => {
            let password = process_genpass(
                genpass_opts.length,
                genpass_opts.uppercase,
                genpass_opts.lowercase,
                genpass_opts.number,
                genpass_opts.symbol,
            )?;
            let estimate = zxcvbn(&password, &[])?;
            eprintln!("password strength: {}", estimate.score());
        }
        SubCommand::Base64(base64_opts) => match base64_opts {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(text_opts) => match text_opts {
            TextSubCommand::Sign(text_opts) => {
                let sig = process_text_sign(&text_opts.input, &text_opts.key, text_opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(text_opts) => {
                let verified = process_text_verify(
                    &text_opts.input,
                    &text_opts.key,
                    text_opts.format,
                    &text_opts.sig,
                )?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(text_opts) => {
                let key = process_generate(text_opts.format)?;
                match &text_opts.format {
                    TextSignFormat::Blake3 => {
                        let name = text_opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &text_opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
