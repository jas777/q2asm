use crate::config::Config;
use sailfish::TemplateOnce;
use super::components::*;

pub fn generate_docs_html(config: &Config) -> String {
    let ctx = instruction_set::InstructionSet {
        instructions: config.instructions.clone(),
        config: &config.assembler
    };

    ctx.render_once().unwrap()
    // "".to_string()
}
