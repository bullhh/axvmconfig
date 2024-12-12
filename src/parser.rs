use std::fs;
use std::path::Path;

use clap::{Args, Parser, Subcommand};

use crate::AxVMCrateConfig;

#[derive(Parser)]
#[command(name = "axvmconfig")]
#[command(about = "A simple VM configuration tool for ArceOS-Hypervisor.", long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct CLI {
    #[command(subcommand)]
    pub subcmd: CLISubCmd,
}

#[derive(Subcommand)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub enum CLISubCmd {
    /// Parse the configuration file and check its validity.
    Check(CheckArgs),
}

#[derive(Debug, Args)]
pub struct CheckArgs {
    #[arg(short, long)]
    pub config_path: String,
}

pub fn run() {
    let cli = CLI::parse();
    match cli.subcmd {
        CLISubCmd::Check(args) => {
            let file_path = &args.config_path;
            // Check if the file exists.
            if !Path::new(file_path).exists() {
                eprintln!("Error: File '{}' does not exist.", file_path);
                std::process::exit(1);
            }
            let file_content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Error: Failed to read file '{}': {}", file_path, err);
                    std::process::exit(1);
                }
            };

            match AxVMCrateConfig::from_toml(&file_content) {
                Ok(config) => {
                    println!("Config file '{}' is valid.", file_path);
                    println!("Config: {:#x?}", config);
                }
                Err(err) => {
                    eprintln!("Error: Config file '{}' is invalid: {}", file_path, err);
                    std::process::exit(1);
                }
            }
        }
    }
}
