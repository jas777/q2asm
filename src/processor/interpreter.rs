use crate::config::AssemblerConfig;
use crate::loader::{Instruction, InstructionArgument};
use crate::processor::preprocessor;
use parse_int;
use std::collections::HashMap;
use std::result::Result;
use std::{fmt, fs};

pub fn interpret_and_write(
    input: &str,
    output_path: &str,
    config: AssemblerConfig,
    instructions: Vec<Instruction>,
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

    let consts = match preprocessor::find_consts(&lines) {
        Ok(consts) => consts,
        Err(err) => return Err(err)
    };

    for line in input.split("\n") {
        line_num += 1;
        actual_line_num += 1;
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
                ))
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

        println!("{:?}", args);

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

        // let args_to_iter = &instruction
        //     .arguments
        //     .iter()
        //     .filter(|a| !a.name.starts_with("@"))
        //     .collect::<Vec<&InstructionArgument>>();

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
                        ))
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
                        ))
                    }
                };
                if consts.contains_key(const_used) {
                    let const_val = consts.get(const_used).unwrap().to_string();
                    argument = const_val;
                };
            }

            let parsed_arg = parse_int::parse::<usize>(&argument);

            let parsed_arg = match parsed_arg {
                Ok(num) => num,
                Err(_err) => {
                    return Err(format!(
                        "Syntax error on line {}! Invalid argument format \"{}\"!",
                        actual_line_num, argument
                    ))
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

        // println!("{}", opcode_len);
        output_buff.push_str(&format!(
            "{:0line_num_len$}: {i_opcode:0opcode_len$b}",
            line_num,
            opcode_len = opcode_len
        ));
        processed_args.iter().for_each(|a| {
            output_buff.push_str(a);
            // output_buff.push_str("gówno ")
        });
        output_buff.push_str("\n");
    }

    match fs::write(output_path, &output_buff) {
        Ok(_success) => return Ok(()),
        Err(_err) => return Err("An error occured while writing to file!".to_owned()),
    };
}
