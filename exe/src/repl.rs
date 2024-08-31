use clap::{Parser, Subcommand};
use std::io::{self, Write};

/// Refer:
/// - https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
/// - https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
#[derive(Parser, Default)]
#[command(name = "a2600", about = "Hardware debugger for Atari 2600.", long_about = None)]
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
    Go {
        #[arg(index = 1)]
        count: Option<usize>,
    },

    #[command(name = "r")]
    Registers,

    #[command(name = "m")]
    MemoryDump {
        #[arg(index = 1)]
        start: Option<String>,
    },

    #[command(name = "d")]
    Disassemble {
        #[arg(index = 1)]
        start: Option<String>,
    },

    #[command(name = "l")]
    Load {
        #[arg(index = 1)]
        start: String,

        #[arg(index = 2)]
        path: String,
    },

    #[command(name = "s")]
    SetReg {
        #[arg(index = 1)]
        reg: String,

        #[arg(index = 2)]
        val: String,
    },

    #[command(name = "bp")]
    BreakPoint {
        #[arg(index = 1)]
        op: String,

        #[arg(index = 2)]
        addr: String,
    },

    #[command(name = "bpl")]
    BreakPointList,
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
