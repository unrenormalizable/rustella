use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser, Default)]
#[command(name = "")]
#[command(about = "Hardware debugger for Atari 2600.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn command(&self) -> &Option<Commands> {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "q")]
    Quit,

    #[command(name = "g")]
    Go,

    #[command(name = "r")]
    Registers,

    #[command(name = "d")]
    DumpMem {
        #[arg(index = 1)]
        start: Option<String>,
    },
}

pub fn get_cmdline() -> Cli {
    print!("? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut args = input
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    if args.is_empty() {
        return Cli::default();
    }

    let cmd = args[0].clone();
    args.insert(0, "".to_string());
    Cli::try_parse_from(args).unwrap_or_else(|e| {
        if cmd == "help" {
            eprintln!("{e}");
        } else {
            eprintln!("Unknown command {cmd}.");
        }
        Cli::default()
    })
}
