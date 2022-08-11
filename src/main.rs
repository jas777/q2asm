mod config;
mod processor;
mod docs;
mod loader;

use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "jas777", version, about = "A customizable assembly compiler for Minecraft CPUs", long_about = None)]
struct Arguments {
    #[clap(short, long, value_parser, default_value = "instr_set.toml")]
    config: String,

    #[clap(short, long, value_parser)]
    input: String,

    #[clap(short, long, value_parser)]
    output: String,

    #[clap(short = 'G', long)]
    generate_docs: bool
}

fn main() {
    let args = Arguments::parse();
    let configuration = loader::load_and_parse(&args.config);

    let input_file = match fs::read_to_string(&args.input) {
        Ok(file) => file,
        Err(_) => {
            println!("Could not load the file \"{}\"!", &args.input);
            return
        }
    };
    fs::read_to_string(&args.output).expect_err("That file already exists!");

    if args.generate_docs {
        match docs::gen_and_write_docs(&configuration.0) {
            Ok(_) => println!("Documentation generated successfully!"),
            Err(err) => println!("Documentation generation failed: {}", err)
        }
    }

    match processor::interpreter::interpret_and_write(
        &input_file,
        &args.output,
        configuration.0.assembler,
        configuration.1,
    ) {
        Ok(()) => println!("Compiled successfully!"),
        Err(e) => println!("{}", e),
    }

    // println!("{:#?}", configuration.0);
}
