mod base64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::genpass::GenPassOpts;
pub use self::{base64::*, csv::*, http::*, text::*};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

// impl CmdExecutor for SubCommand {
//     async fn execute(&self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::Csv(opt) => opt.execute().await,
//             SubCommand::Base64(opt) => opt.execute().await,
//             SubCommand::GenPass(opt) => opt.execute().await,
//             SubCommand::Text(opt) => opt.execute().await,
//             SubCommand::Http(opt) => opt.execute().await,
//         }
//     }
// }

pub fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

pub fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or it is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(
            verify_input_file("*"),
            Err("File does not exist".to_string())
        );
        assert_eq!(
            verify_input_file("not-exist"),
            Err("File does not exist".to_string())
        );
    }
}
