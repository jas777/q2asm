use crate::config::Config;
use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
pub struct Instruction {
    pub name: String,
    pub opcode: usize,
    pub opcode_len: usize,
    pub arguments: Vec<InstructionArgument>,
}

#[derive(Deserialize, Debug)]
pub struct InstructionArgument {
    pub name: String,
    pub length: usize,
}

pub fn load_and_parse(_path: &str) -> (Config, Vec<Instruction>) {
    let mut instructions = Vec::<Instruction>::new();

    let instr_set = fs::read_to_string("instr_set.toml")
        .expect("instr_set.toml is required to run the compiler!");

    let instr_set: Config = toml::from_str(&instr_set).expect("Invalid file format!");

    let instr_len = instr_set.assembler.instruction_len;

    for instruction in &instr_set.instructions {
        let constructed_args: Vec<InstructionArgument> = instruction
            .arguments
            .iter()
            .map(|x| {
                let split: Vec<String> = x.as_str().split("-").map(str::to_string).collect();
                InstructionArgument {
                    name: split[0].clone(),
                    length: split[1].parse::<usize>().expect(&format!(
                        "Invalid argument format! Instruction: {}",
                        instruction.name
                    )),
                }
            })
            .collect();

        let mut final_len: usize = constructed_args.iter().map(|arg| arg.length).sum();
        final_len += if instruction.opcode_len != 0 {
            instruction.opcode_len
        } else {
            instr_set.assembler.opcode_len
        };

        if final_len != instr_len {
            panic!(
                "Instruction {} has invalid length! ({} != {})",
                instruction.name, final_len, instr_len
            )
        }

        instructions.push(Instruction {
            opcode: instruction.opcode,
            opcode_len: instruction.opcode_len,
            name: instruction.name.to_owned(),
            arguments: constructed_args,
        });
    }

    (instr_set, instructions)
}
