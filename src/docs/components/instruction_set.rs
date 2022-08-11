use sailfish::TemplateOnce;
use crate::config::*;

#[derive(TemplateOnce)]
#[template(path = "instruction_set.stpl")]
pub struct InstructionSet <'a> {
    pub config: &'a AssemblerConfig,
    pub instructions: Vec<InstructionConfig>
}
