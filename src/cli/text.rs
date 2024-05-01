use crate::{process_generate, process_text_sign, process_text_verify, CmdExecutor};
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, fs, path::PathBuf, str::FromStr};

use super::{verify_file, verify_path};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(&self) -> Result<()> {
        let _ = process_text_sign(&self.input, &self.key, self.format);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(&self) -> Result<()> {
        let _ = process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(&self) -> Result<()> {
        let key = process_generate(self.format)?;
        match &self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0])?;
                fs::write(name.join("ed25519.pk"), &key[1])?;
            }
        };
        Ok(())
    }
}
