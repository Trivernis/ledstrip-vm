use ledstrip_vm::asm_tokens;
use ledstrip_vm::asm_tokens::{
    AddToken, ClearToken, CmdToken, CopyToken, DivToken, ExitToken, GotoToken, JeToken, JgToken,
    JlToken, LabelToken, LoadToken, LshToken, ModToken, MulToken, PauseToken, RshToken, SetToken,
    SubToken, Token, WriteToken,
};
use ledstrip_vm::registers::get_register_by_name;
use std::fs::{read_to_string, File};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::num::ParseIntError;
use structopt::StructOpt;

macro_rules! some_box {
    ($expr:expr) => {
        Some(Box::new($expr))
    };
}

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(short = "i", name = "input")]
    input_file: String,

    #[structopt(short = "o", name = "output")]
    output_file: String,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::from_args();
    let input_file_name = opts.input_file;
    let contents = read_to_string(&input_file_name)?;
    let f = File::create(opts.output_file)?;
    let mut writer = BufWriter::new(f);
    let mut line_number = 0;
    contents
        .lines()
        .filter_map(|line| {
            line_number += 1;
            if let Some(token) = get_token(line) {
                Some(token.to_bytecode())
            } else {
                println!(
                    "Failed to parse instruction '{}' \n-> {}:{}",
                    line, &input_file_name, line_number
                );
                None
            }
        })
        .for_each(|code| {
            writer.write(&code).expect("Failed to write output.");
        });

    writer.flush()?;
    Ok(())
}

/// Parses the line into a token
fn get_token(line: &str) -> Option<Box<dyn Token>> {
    let mut instr_parts = line.split_whitespace();

    match instr_parts.next()? {
        "exit" => Some(Box::new(ExitToken {
            register: get_register_by_name(instr_parts.next()?)?,
        })),
        "set" => some_box!(SetToken {
            value: parse_value(instr_parts.next()?).expect(&format!(
                "Failed to parse the hex value into a u8: {}.",
                line
            )),
            register: get_register_by_name(instr_parts.next()?)?,
        }),
        "copy" => some_box!(CopyToken {
            register_1: get_register_by_name(instr_parts.next()?)?,
            register_2: get_register_by_name(instr_parts.next()?)?,
        }),
        "load" => some_box!(LoadToken),
        "clear" => some_box!(ClearToken {
            register: get_register_by_name(instr_parts.next()?)?,
        }),
        "write" => some_box!(WriteToken),
        "label" => some_box!(LabelToken),
        "goto" => some_box!(GotoToken),
        "add" => some_box!(AddToken),
        "sub" => some_box!(SubToken),
        "mul" => some_box!(MulToken),
        "div" => some_box!(DivToken),
        "mod" => some_box!(ModToken),
        "lsh" => some_box!(LshToken),
        "rsh" => some_box!(RshToken),
        "jg" => some_box!(JgToken),
        "jl" => some_box!(JlToken),
        "je" => some_box!(JeToken),
        "pause" => some_box!(PauseToken),
        "cmd" => some_box!(CmdToken),
        _ => None,
    }
}

/// Parses a value depending on if it starts with 0x (as a hex value)
/// or just is a plain base-10 number
fn parse_value(value: &str) -> Result<u8, ParseIntError> {
    if value.starts_with("0x") {
        let value = value.trim_start_matches("0x");
        Ok(i64::from_str_radix(value, 16)? as u8)
    } else {
        value.parse::<u8>()
    }
}
