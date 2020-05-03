use ledstrip_vm::registers::get_register_code_by_name;
use ledstrip_vm::tokens::{
    AddToken, ClearToken, CmdToken, CopyToken, DebugToken, DivToken, ExitToken, GotoToken, JeToken,
    JgToken, JlToken, LabelToken, LoadToken, LshToken, ModToken, MulToken, PauseToken, RshToken,
    SendToken, SetToken, SubToken, Token, WriteToken,
};
use std::fs::{read_to_string, File};
use std::io;
use std::io::{BufWriter, Write};
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
            } else if line.replace("\\s", "").len() > 0 && !line.starts_with("#") {
                println!(
                    "Failed to parse instruction '{}' \n-> {}:{}",
                    line, &input_file_name, line_number
                );
                None
            } else {
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
        "exit" => some_box!(ExitToken {
            register: get_register_code_by_name(instr_parts.next()?)?,
        }),
        "set" => some_box!(SetToken {
            value: parse_value(instr_parts.next()?).expect(&format!(
                "Failed to parse the hex value into a u8: {}.",
                line
            )) as u8,
            register: get_register_code_by_name(instr_parts.next()?)?,
        }),
        "copy" => some_box!(CopyToken {
            register_1: get_register_code_by_name(instr_parts.next()?)?,
            register_2: get_register_code_by_name(instr_parts.next()?)?,
        }),
        "load" => some_box!(LoadToken),
        "clear" => some_box!(ClearToken {
            register: get_register_code_by_name(instr_parts.next()?)?,
        }),
        "write" => some_box!(WriteToken),
        "label" => some_box!(LabelToken {
            value: parse_value(instr_parts.next()?).expect("Failed to parse label name")
        }),
        "goto" => some_box!(GotoToken),
        "debug" => some_box!(DebugToken),
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
        "send" => some_box!(SendToken),
        _ => None,
    }
}

/// Parses a value depending on if it starts with 0x (as a hex value)
/// or just is a plain base-10 number
fn parse_value(value: &str) -> Result<u32, ParseIntError> {
    if value.starts_with("0x") {
        let value = value.trim_start_matches("0x");
        Ok(i64::from_str_radix(value, 16)? as u32)
    } else {
        value.parse::<u32>()
    }
}
