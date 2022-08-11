use serde_derive::Deserialize;

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
    pub arguments: Vec<String>
}
