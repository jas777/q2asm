use crate::config::AssemblerConfig;
use crate::loader::{Instruction, InstructionArgument};
use crate::processor::preprocessor;
use parse_int;
use std::collections::HashMap;
use std::result::Result;
use std::{fmt, fs, string, str};
use crate::OutputType;

pub fn interpret_and_write(
    input: &str,
    output_path: &str,
    config: AssemblerConfig,
    instructions: Vec<Instruction>,
    output_type: OutputType,
) -> Result<(), String> {
    let mut line_num: u32 = 0; // line number in the code (excluding empty lines and labels)
    let mut actual_line_num: u32 = 0; // line number in the file
    let mut output_buff: String = String::new();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let line_count: usize = lines.len();
    let line_num_len = line_count.to_string().len();

    let labels = match preprocessor::find_labels(&lines) {
        Ok(labels) => labels,
        Err(err) => return Err(err)
    };

    let consts = match preprocessor::find_constants(&lines) {
        Ok(consts) => consts,
        Err(err) => return Err(err)
    };

    for line in input.split("\n") {
        line_num += 1;
        actual_line_num += 1;

        let line = line.trim();
        let content: Vec<String> = line.split("--").map(str::to_string).collect();
        let command: Vec<&str> = content[0].trim().split(" ").collect::<Vec<&str>>();

        if line.is_empty() || line.starts_with(".") || line.starts_with("#") || line.starts_with("--") {
            line_num -= 1;
            continue;
        }

        if command.len() > 2 {
            return Err(format!(
                "Syntax error on line {}! Expected format \"INSTR ARG1,ARG2...\" but found \"{}\"",
                actual_line_num, line
            ));
        }

        let instruction: &Instruction = match instructions
            .iter()
            .find(|instr| instr.name == command[0].to_uppercase())
        {
            Some(instr) => instr,
            None => {
                return Err(format!(
                    "Syntax error on line {}! Unknown instruction \"{}\"",
                    actual_line_num, command[0]
                ));
            }
        };

        let expected_arg_len = instruction
            .arguments
            .iter()
            .filter(|a| !a.name.starts_with("@fill"))
            .count();
        let args: Vec<String> = if command.len() > 1 {
            command[1].trim().split(",").map(str::to_string).collect()
        } else {
            vec![]
        };

        if args.len() != expected_arg_len {
            return Err(format!(
                "Syntax error on line {}! Instruction \"{}\" expected {} arguments, but got {}",
                actual_line_num,
                instruction.name,
                expected_arg_len,
                args.len()
            ));
        }

        let mut processed_args: Vec<String> = Vec::new();
        let mut arg_index: usize = 0;

        for arg_def in &instruction.arguments {
            if arg_def.name.eq("@fill") {
                processed_args.push("0".repeat(arg_def.length));
                continue;
            }

            let mut argument = args[arg_index].clone();

            if argument.starts_with(":") {
                let label_used = match argument.strip_prefix(":") {
                    Some(label) => label,
                    None => {
                        return Err(format!(
                            "Syntax error on line {}! Expected a label but got nothing!",
                            actual_line_num
                        ));
                    }
                };
                if labels.contains_key(label_used) {
                    let label_addr = labels.get(label_used).unwrap().to_string();
                    argument = label_addr;
                };
            }

            if argument.starts_with("#") {
                let const_used = match argument.strip_prefix("#") {
                    Some(const_name) => const_name,
                    None => {
                        return Err(format!(
                            "Syntax error on line {}! Expected a const but got nothing!",
                            actual_line_num
                        ));
                    }
                };
                if consts.contains_key(const_used) {
                    let const_val = consts.get(const_used).unwrap().to_string();
                    argument = const_val;
                };
            }

            if argument.starts_with("-") {
                argument = match preprocessor::handle_negative_val(argument) {
                    Ok(val) => val,
                    Err(_e) => return Err(format!(
                        "Syntax error on line {}! Invalid argument value for \"{}\"!",
                        actual_line_num,
                        arg_def.name
                    ))
                }
            }

            let parsed_arg = parse_int::parse::<usize>(&argument);

            let parsed_arg = match parsed_arg {
                Ok(num) => num,
                Err(_err) => {
                    return Err(format!(
                        "Syntax error on line {}! Invalid argument format \"{}\"!",
                        actual_line_num, argument
                    ));
                }
            };

            let mut bin_arg = format!("{:b}", parsed_arg);

            if bin_arg.len() < arg_def.length {
                bin_arg = format!("{}{}", &"0".repeat(arg_def.length - bin_arg.len()), bin_arg);
            }

            if bin_arg.len() > arg_def.length {
                return Err(
                    format!(
                        "Syntax error on line {}! Argument \"{}\" is too long, expected {} bits but got {}!",
                        actual_line_num, arg_def.name, arg_def.length, bin_arg.len())
                );
            }

            processed_args.push(bin_arg);
            arg_index += 1;
        }

        let i_opcode = instruction.opcode;
        let opcode_len = if instruction.opcode_len != 0 {
            instruction.opcode_len
        } else {
            config.opcode_len
        };

        let mut instr_buff = String::new();

        instr_buff.push_str(&format!(
            "{i_opcode:0opcode_len$b}",
            i_opcode = i_opcode,
            opcode_len = opcode_len
        ));

        processed_args.iter().for_each(|a| {
            instr_buff.push_str(a);
        });

        let instr_buff = match &output_type {
            OutputType::BIN => instr_buff,
            OutputType::OCT => {
                let parsed = usize::from_str_radix(&instr_buff, 2).unwrap();
                let instr_len = (config.instruction_len as f64 / 3_f64) as usize;
                format!("{:0instr_len$o}", parsed, instr_len = instr_len)
            }
            OutputType::HEX => {
                let parsed = usize::from_str_radix(&instr_buff, 2).unwrap();
                let instr_len = (config.instruction_len as f64 / 4_f64) as usize;
                format!("{:0instr_len$x}", parsed, instr_len = instr_len)
            }
            OutputType::DEC => {
                let parsed = usize::from_str_radix(&instr_buff, 2).unwrap();
                let max_instr_len = 2_usize
                    .pow(config.instruction_len as u32 - 1)
                    .to_string().len();
                format!("{:0max_instr_len$}", parsed, max_instr_len = max_instr_len)
            }
        };

        let instr = format!("{:0line_num_len$}: {}", line_num, instr_buff, line_num_len = line_num_len);
        output_buff.push_str(&*instr);
        output_buff.push_str("\n");
    }

    return match fs::write(output_path, &output_buff) {
        Ok(_success) => Ok(()),
        Err(_err) => Err("An error occurred while writing to file!".to_owned()),
    };
}
