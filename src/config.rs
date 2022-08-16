use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub assembler: AssemblerConfig,
    pub instructions: Vec<InstructionConfig>
}

#[derive(Deserialize, Debug)]
pub struct AssemblerConfig {
    pub opcode_len: usize,
    pub instruction_len: usize
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstructionConfig {
    pub name: String,
    pub opcode: usize,
    pub opcode_len: usize,
    pub arguments: Vec<InstructionArgumentConfig>
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstructionArgumentConfig {
    pub name: String,
    pub length: String,
    pub accepted_values: Option<Vec<String>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct FlagConfig {
    pub name: String,
    pub value: String
}
