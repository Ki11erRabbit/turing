use interpreter::Interpreter;
use clap::Parser;

use crate::interpreter::InterpreterUtils;

pub mod interpreter;

#[derive(Parser)]
#[command(name = "turing", version = "0.1.0", about = "A simple turing machine interpreter")]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long)]
    manual: bool,
}



fn main() {

    let args = Args::parse();
    
    let code = if let Some(file) = args.file {
        std::fs::read_to_string(file).unwrap()
    } else {
        let mut line = String::new();
        println!("Enter your code: ");
        std::io::stdin().read_line(&mut line).unwrap();
        line
    };

    let commands = turing_definitions::parser::parse(&code);

    let mut interpreter = Interpreter::new();

    interpreter.interpret_commands(&commands);
}
