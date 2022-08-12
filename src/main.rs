mod config;
mod processor;
mod docs;
mod loader;

use std::fs;
use clap::Parser;

#[derive(Debug)]
pub enum OutputType {
    BIN,
    OCT,
    HEX,
    DEC,
}

#[derive(Parser, Debug)]
#[clap(author = "jas777", version, about = "A customizable assembly compiler for Minecraft CPUs", long_about = None)]
struct Arguments {
    #[clap(short, long, value_parser, default_value = "instr_set.toml")]
    config: String,

    #[clap(short, long, value_parser)]
    input: String,

    #[clap(short, long, value_parser)]
    output: String,

    #[clap(short = 'O', long = "output-format", value_parser, default_value = "bin")]
    output_format: String,

    #[clap(short = 'G', long)]
    generate_docs: bool,

    #[clap(short, long)]
    force: bool,
}

fn main() {
    let args = Arguments::parse();
    let configuration = loader::load_and_parse(&args.config);

    let input_file = match fs::read_to_string(&args.input) {
        Ok(file) => file,
        Err(_) => {
            println!("Could not load the file \"{}\"!", &args.input);
            return;
        }
    };

    if !args.force {
        fs::read_to_string(&args.output).expect_err("That file already exists!");
    }

    if args.generate_docs {
        match docs::gen_and_write_docs(&configuration.0) {
            Ok(_) => println!("Documentation generated successfully!"),
            Err(err) => println!("Documentation generation failed: {}", err)
        }
    }

    let output_type: OutputType = match args.output_format.as_str() {
        "bin" => OutputType::BIN,
        "oct" => OutputType::OCT,
        "hex" => OutputType::HEX,
        "dec" => OutputType::DEC,
        _ => {
            println!("Invalid output type! (bin, oct, hex, dec)");
            return;
        }
    };

    match processor::interpreter::interpret_and_write(
        &input_file,
        &args.output,
        configuration.0.assembler,
        configuration.1,
        output_type,
    ) {
        Ok(()) => println!("Compiled successfully!"),
        Err(e) => println!("{}", e),
    }
}
