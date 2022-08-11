pub mod generator;
mod components;

use crate::config::Config;
use std::fs;

pub fn gen_and_write_docs(config: &Config) -> Result<(), String> {
    let generated_docs = generator::generate_docs_html(config);
    
    match fs::write("docs.html", generated_docs) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Writing documentation failed!".to_string())
    }

}