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
                        if c == 'c' && event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                            shutdown();
                            return;
                        }
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

pub fn shutdown() {
    disable_raw_mode().unwrap();
    execute!(std::io::stdout(),
             DisableBracketedPaste,
    ).unwrap();
}

fn main() {

    let args = Args::parse();

    if args.manual {
        println!("{}", MANUAL);
        return;
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

    //disable_raw_mode().unwrap();

    let commands = turing_definitions::parser::parse(&code);
    
    //println!("{:#?}", commands);

    //enable_raw_mode().unwrap();
    
    let mut interpreter = Interpreter::new();
    interpreter.interpret_commands(&commands);

    shutdown();
}


static MANUAL: &str = r#"
Turing Machine Simulator
Numbers:
  - Numbers move the tape head over by one and set that cell to the number
  - Example: 5
Strings:
  - Strings move the tape head over by each character and set that cell to the UTF32 value of the character
  - Example: "Hello"
Writing in Place:
  - Writing in place rather than moving the tape head is done by wrapping the value in brackets.
  - List will still move the tape head
  - Example: [5]
Commands:
  - <: Move the tape head to the left
  - >: Move the tape head to the right
  - <|: Move the tape head to the left by the value of the current cell
  - |>: Move the tape head to the right by the value of the current cell
  - <#): Move the tape head to the left by #
  - (#>): Move the tape head to the right by #
  - +: Increment the value of the current cell
  - -: Decrement the value of the current cell
Arithmetic:
Uses +, -, *, /, %
  - <_: A _ B = A <_ B
  - _>: B _ A = A _> B
  - A number can be included between the bracket and the operator to do the operation that many cells away
Control Flow:
  - if: If the current cell is not 0, execute the code until else or end, otherwise skip to else or end
  - while: While the current cell is not 0, execute the code until end
  - loop: Execute the code until end, then check if the current cell is 0, if not, repeat
  - end: End a block of code
Functions:
  - fun name: Define a function with the name name
  - end: End the function
  - getfun name: Get the index of the function with the name name
  - call: Call the function at the index of the current cell
Printing:
  - .: (Period) Print the value of the current cell as a number
  - ,: (Comma) Print the value of the current cell as a character
User Input:
  - ?: (Question Mark) Get a keypress from the user and set the current cell to the UTF32 value of the key
Quitting:
  - c^c: (Control-C) Quit the program
See this website for more information: https://killerrabbit.xyz/pl/turing/"#;
