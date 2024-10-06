use clap::{Parser, ValueEnum};
use clap_repl::ClapEditor;
use reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use std::path::PathBuf;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Register {
    A,
    X,
    Y,
    PC,
    S,
    PSR,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BreakPointOp {
    #[value(aliases = ["a", "add"])]
    Add,
    #[value(aliases = ["r", "rem", "d", "del"])]
    Remove,
}

/// Refer:
/// - https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
/// - https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
#[derive(Debug, Parser)]
#[command(name = "", about = "Hardware debugger for Atari 2600.", long_about = None)]
pub enum Commands {
    #[command(
        visible_aliases = [ "q", "exit" ],
        about = "Exit the debugger.",
        long_about = None)]
    Quit,

    #[command(
        visible_aliases = [ "g" ],
        about = "Run number of instructions and/or till break point is hit.",
        long_about = None)]
    Go {
        #[arg(
            index = 1,
            default_value_t = u64::MAX,
            value_parser = clap::value_parser!(u64).range(1..),
            help = "Number of instructions to execute.")]
        count: u64,
    },

    #[command(
        visible_aliases = [ "r", "reg" ],
        about = "Display registers & next instruction to execute.",
        long_about = None)]
    Registers,

    #[command(
        visible_aliases = [ "s", "sr", "sreg" ],
        about = "Set one of the registers.",
        long_about = None)]
    SetRegisters {
        #[arg(index = 1, value_enum, help = "Register to set")]
        reg: Register,

        #[arg(
            index = 2,
            value_parser = parse_u16_hex,
            help = "Value to set into register.")]
        val: u16,
    },

    #[command(
        visible_aliases = [ "m", "mem" ],
        about = "Dump the 128 bytes of memory starting location.",
        long_about = None)]
    Memory {
        #[arg(
            index = 1,
            default_value_t = 0,
            value_parser = parse_u16_hex,
            help = "Starting address.")]
        start: u16,
    },

    #[command(
        visible_aliases = [ "d", "dis" ],
        about = "Disassemble the next 16 instructions starting location.",
        long_about = None)]
    Disassemble {
        #[arg(
            index = 1,
            default_value_t = 0,
            value_parser = parse_u16_hex,
            help = "Starting address.")]
        start: u16,

        #[arg(
            index = 2,
            default_value_t = 16,
            value_parser = clap::value_parser!(u64),
            help = "Starting address.")]
        count: u64,
    },

    #[command(
        visible_aliases = [ "l", "ld" ],
        about = "Load contents of binary file into memory starting address.",
        long_about = None)]
    Load {
        #[arg(
            index = 1,
            value_parser = parse_u16_hex,
            help = "Starting address.")]
        start: u16,

        #[arg(
            index = 2,
            value_parser = clap::value_parser!(PathBuf),
            help = "Path of the binary file.")]
        path: PathBuf,
    },

    #[command(
        visible_aliases = [ "lbp", "bpl" ],
        about = "List of the active break points.",
        long_about = None)]
    BreakPoints,

    #[command(
        visible_aliases = [ "b", "bp" ],
        about = "Add or remove break points.",
        long_about = None)]
    BreakPointChange {
        #[arg(index = 1, value_enum, help = "Add or remove break point.")]
        op: BreakPointOp,

        #[arg(
            index = 2,
            value_parser = parse_u16_hex,
            help = "Address of the break point.")]
        address: u16,
    },
}

pub fn cmd_line() -> ClapEditor<Commands> {
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("? ".to_owned()),
        ..DefaultPrompt::default()
    };
    ClapEditor::<Commands>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            // Do custom things with `Reedline` instance here
            reed.with_history(Box::new(
                FileBackedHistory::with_file(
                    10000,
                    homedir::my_home().unwrap().unwrap().join(".rustella"),
                )
                .unwrap(),
            ))
        })
        .build()
}

fn parse_u16_hex(s: &str) -> Result<u16, String> {
    let val = u16::from_str_radix(s, 16).map_err(|_| format!("`{s}` is not in hex u16 format."))?;
    Ok(val)
}
