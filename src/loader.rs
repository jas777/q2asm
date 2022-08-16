use crate::config::Config;
use serde_derive::Deserialize;
use std::fs;
use serde_yaml;

#[derive(Deserialize, Debug)]
pub struct Instruction {
    pub name: String,
    pub opcode: usize,
    pub opcode_len: usize,
    pub arguments: Vec<InstructionArgument>,
}

#[derive(Deserialize, Debug)]
pub struct Flag {
    pub name: String,
    pub value: usize,
}

#[derive(Deserialize, Debug)]
pub struct InstructionArgument {
    pub name: String,
    pub length: usize,
    pub accepted_values: Vec<usize>,
}

pub fn load_and_parse(path: &str) -> Result<(Config, Vec<Instruction>), String> {
    let mut instructions = Vec::<Instruction>::new();

    let instr_set = fs::read_to_string(path)
        .expect("instr_set.toml is required to run the compiler!");

    let instr_set: Config = serde_yaml::from_str(&instr_set).expect("Invalid file format!");

    let instr_len = instr_set.assembler.instruction_len;

    for instruction in &instr_set.instructions {
        let constructed_args: Vec<InstructionArgument> = instruction
            .arguments
            .iter()
            .map(|x| {
                let length = x.length.parse::<usize>().expect(&format!(
                    "Invalid argument format! Instruction: {}",
                    instruction.name
                ));
                InstructionArgument {
                    name: x.name.to_owned(),
                    length,
                    accepted_values: match &x.accepted_values {
                        Some(values) => values.iter().map(|val| {
                            let parsed_val = parse_int::parse::<usize>(val)
                                .expect(&format!(
                                    "Invalid accepted value for argument \"{}\" ({})", x.name, val));
                            if format!("{:0b}", parsed_val).len() > length {
                                panic!("Accepted value for argument \"{}\" too long!", x.name)
                            }
                            parsed_val
                        }).collect(),
                        None => Vec::new()
                    },
                }
            }).collect();

        let mut final_len: usize = constructed_args.iter().map(|arg| arg.length).sum();
        final_len += if instruction.opcode_len != 0 {
            instruction.opcode_len
        } else {
            instr_set.assembler.opcode_len
        };

        if final_len != instr_len {
            return Err(format!(
                "Instruction {} has invalid length! ({} != {})",
                instruction.name, final_len, instr_len
            ));
        }

        instructions.push(Instruction {
            opcode: instruction.opcode,
            opcode_len: instruction.opcode_len,
            name: instruction.name.to_owned(),
            arguments: constructed_args,
        });
    }

    Ok((instr_set, instructions))
}
