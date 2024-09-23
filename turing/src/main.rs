use std::io::Write;
use crossterm::event::{read, DisableBracketedPaste, EnableBracketedPaste, Event};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
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



fn read_keys(buffer: &mut String) {
    loop {
        match read().unwrap() {
            Event::Key(event) => {
                match event.code {
                    crossterm::event::KeyCode::Enter => {
                        println!("\r");
                        break;
                    }
                    crossterm::event::KeyCode::Backspace => {
                        print!("\x08 \x08");
                        std::io::stdout().flush().unwrap();
                        buffer.pop();
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        print!("{}", c);
                        std::io::stdout().flush().unwrap();
                        buffer.push(c);
                    }
                    _ => {}
                }
            }
            Event::Paste(text) => {
                println!("Paste: {}\r", text);
                buffer.push_str(&text);
                break;
            }
            _ => {}
        }

    }
}


fn main() {

    let args = Args::parse();

    if args.manual {
        todo!("Add manual");
    }

    execute!(std::io::stdout(),
             EnableBracketedPaste,
    ).unwrap();
    enable_raw_mode().unwrap();

    std::panic::set_hook(Box::new(|panic_info| {
        disable_raw_mode().unwrap();
        execute!(std::io::stdout(),
                 DisableBracketedPaste,
        ).unwrap();
        eprintln!("{}", panic_info);
    }));
    
    
    let code = if let Some(file) = args.file {
        std::fs::read_to_string(file).unwrap()
    } else {
        let mut line = String::new();
        println!("\r\nEnter your code: \r");
        read_keys(&mut line);
        //std::io::stdin().read_line(&mut line).unwrap();
        line
    };

    let commands = turing_definitions::parser::parse(&code);

    let mut interpreter = Interpreter::new();
    interpreter.interpret_commands(&commands);

    execute!(std::io::stdout(),
             DisableBracketedPaste,
    ).unwrap();
    disable_raw_mode().unwrap();
}
