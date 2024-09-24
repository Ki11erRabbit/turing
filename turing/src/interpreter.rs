use std::time::Duration;
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use turing_definitions::{ast::{Command, SpannedCommand}, Direction, Number, Tape};
use std::io::Write;

fn check_for_ctrl_c() {
    if poll(Duration::from_millis(1)).unwrap() {
        match read().unwrap() {
            Event::Key(event) => {
                if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                    super::shutdown();
                    std::process::exit(0);
                }
            }
            _ => {}
        }
    }
}

pub trait InterpreterUtils<'a, T:Number> {
    fn interpret_commands(&'a mut self, command_tape: &'a Vec<SpannedCommand<'a>>);
    fn add_string(&mut self, s: &str);
    fn write_string(&mut self, s: &str);
    fn output_char(&mut self);
    fn write_char(&mut self, c: char);
}

struct InterpreterExt<'a, T: Number> {
    add_string: fn(&mut Interpreter<'a, T>, &str),
    write_string: fn(&mut Interpreter<'a, T>, &str),
    output_char: fn(&mut Interpreter<'a, T>),
    write_char: fn(&mut Interpreter<'a, T>, char),
}

pub struct Interpreter<'a, T: Number> {
    tape: Rc<RefCell<dyn Tape<T>>>,
    tape_index: usize,
    command_index: usize,
    functions: HashMap<&'a str, usize>,
    functions_list: Vec<&'a Vec<SpannedCommand<'a>>>,
}

impl<'a, T: Number> Interpreter<'a, T> {
    fn interpret_command(&mut self, command: &'a SpannedCommand, interpreter_ext: &InterpreterExt<'a, T>) {
        match &command.command {
            Command::AddInteger(i) => {
                let mut tape = self.tape.borrow_mut();
                self.tape_index += 1;
                while !tape.in_bounds(self.tape_index) {
                    tape.grow();
                }
                tape.set(self.tape_index, T::from(*i));
            }
            Command::AddString(s) => {
                (interpreter_ext.add_string)(self, s);
            }
            Command::WriteInteger(i) => {
                let mut tape = self.tape.borrow_mut();
                tape.set(self.tape_index, T::from(*i));
            }
            Command::WriteString(s) => {
                (interpreter_ext.write_string)(self, s);
            }
            Command::MoveLeft => {
                self.tape_index = self.tape_index.saturating_sub(1);
            }
            Command::MoveRight => {
                self.tape_index += 1;
                while !self.tape.borrow().in_bounds(self.tape_index) {
                    self.tape.borrow_mut().grow();
                }
            }
            Command::ReadMoveLeft => {
                let tape = self.tape.borrow();
                let offset = tape.get(self.tape_index);
                self.tape_index = self.tape_index.saturating_sub(offset.to_u64() as usize);
            }
            Command::ReadMoveRight => {
                let tape = self.tape.borrow();
                let offset = tape.get(self.tape_index);
                self.tape_index += offset.to_u64() as usize;
                while !self.tape.borrow().in_bounds(self.tape_index) {
                    self.tape.borrow_mut().grow();
                }
            }
            Command::MoveNLeft(n) => {
                self.tape_index = self.tape_index.saturating_sub(*n as usize);
            }
            Command::MoveNRight(n) => {
                self.tape_index += *n as usize;
                while !self.tape.borrow().in_bounds(self.tape_index) {
                    self.tape.borrow_mut().grow();
                }
            }
            Command::Increment => {
                let mut tape = self.tape.borrow_mut();
                tape.increment(self.tape_index);
            }
            Command::Decrement => {
                let mut tape = self.tape.borrow_mut();
                tape.decrement(self.tape_index);
            }
            Command::LeftAdd(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.add(self.tape_index, Direction::Left, *offset);
            }
            Command::RightAdd(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.add(self.tape_index, Direction::Right, *offset);
            }
            Command::LeftSubtract(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.sub(self.tape_index, Direction::Left, *offset);
            }
            Command::RightSubtract(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.sub(self.tape_index, Direction::Right, *offset);
            }
            Command::LeftMultiply(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.mul(self.tape_index, Direction::Left, *offset);
            }
            Command::RightMultiply(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.mul(self.tape_index, Direction::Right, *offset);
            }
            Command::LeftDivide(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.div(self.tape_index, Direction::Left, *offset);
            }
            Command::RightDivide(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.div(self.tape_index, Direction::Right, *offset);
            }
            Command::LeftModulo(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.modulo(self.tape_index, Direction::Left, *offset);
            }
            Command::RightModulo(offset) => {
                let mut tape = self.tape.borrow_mut();
                tape.modulo(self.tape_index, Direction::Right, *offset);
            }
            Command::If(if_commands, else_commands) => {
                let tape = self.tape.borrow();
                if tape.get(self.tape_index).is_nonzero() {
                    let command_index = self.command_index;
                    self.command_index = 0;
                    drop(tape);
                    while self.command_index < if_commands.len() {
                        check_for_ctrl_c();
                        self.interpret_command(&if_commands[self.command_index], interpreter_ext);
                    }
                    self.command_index = command_index;
                } else if let Some(else_commands) = else_commands {
                    let command_index = self.command_index;
                    self.command_index = 0;
                    drop(tape);
                    while self.command_index < else_commands.len() {
                        check_for_ctrl_c();
                        self.interpret_command(&else_commands[self.command_index], interpreter_ext);
                    }
                    self.command_index = command_index;
                }
            }
            Command::While(while_commands) => {
                let tape = self.tape.clone();
                while tape.borrow().get(self.tape_index).is_nonzero() {
                    let command_index = self.command_index;
                    self.command_index = 0;
                    while self.command_index < while_commands.len() {
                        check_for_ctrl_c();
                        self.interpret_command(&while_commands[self.command_index], interpreter_ext);
                    }
                    self.command_index = command_index;
                }
            }
            Command::Loop(loop_commands) => {
                loop {
                    let command_index = self.command_index;
                    self.command_index = 0;
                    while self.command_index < loop_commands.len() {
                        check_for_ctrl_c();
                        self.interpret_command(&loop_commands[self.command_index], interpreter_ext);
                    }
                    self.command_index = command_index;
                    if self.tape.borrow().get(self.tape_index).is_zero() {
                        break;
                    }
                }
            }
            Command::FunctionDefinition(name, commands) => {
                let index = self.functions_list.len();
                self.functions.insert(name, index);
                self.functions_list.push(commands);
            }
            Command::FunctionCall(name) => {
                if let Some(commands) = self.functions.get(name) {
                    let commands = *commands;
                    let command_index = self.command_index;
                    self.command_index = 0;
                    while self.command_index < self.functions_list[commands].len() {
                        check_for_ctrl_c();
                        self.interpret_command(&self.functions_list[commands][self.command_index], interpreter_ext);
                        self.command_index += 1;
                    }
                    self.command_index = command_index;
                }
            }
            Command::GetFunction(name) => {
                let index = self.functions.get(name);
                if let Some(index) = index {
                    let index = *index;
                    let mut tape = self.tape.borrow_mut();
                    tape.set(self.tape_index, T::from(index as i64));
                }
            }
            Command::CallFunction => {
                let command_index = self.command_index;
                self.command_index = 0;
                while self.command_index < self.functions_list.len() {
                    check_for_ctrl_c();
                    self.interpret_command(&self.functions_list[self.command_index][self.command_index], interpreter_ext);
                }
                self.command_index = command_index;
            }
            Command::OutputNumber => {
                let tape = self.tape.borrow();
                print!("{}", tape.get(self.tape_index));
                std::io::stdout().flush().unwrap();
            }
            Command::OutputChar => {
                (interpreter_ext.output_char)(self);
                std::io::stdout().flush().unwrap();
            }
            Command::ReadKey => {
                loop {
                    match read().unwrap() {
                        Event::Key(event) => {
                            match event.code {
                                KeyCode::Enter => {
                                    (interpreter_ext.write_char)(self, '\n');
                                    break;
                                }
                                KeyCode::Char(c) => {
                                    if event.modifiers == KeyModifiers::SHIFT {
                                        (interpreter_ext.write_char)(self, c.to_uppercase().next().unwrap());
                                    } else if event.modifiers == KeyModifiers::CONTROL && c == 'c' {
                                        super::shutdown();
                                    } else {
                                        (interpreter_ext.write_char)(self, c);
                                    }
                                    break;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            Command::Comment => {}
        }
        self.command_index += 1;
    }
}


impl<'a> Interpreter<'a, i64> {
    pub fn new() -> Self {
        Self {
            tape: Rc::new(RefCell::new(vec![0])),
            tape_index: 0,
            command_index: 0,
            functions: HashMap::new(),
            functions_list: Vec::new(),
        }
    }
}

impl<'a> InterpreterUtils<'a, i64> for Interpreter<'a, i64> {
    fn interpret_commands(&'a mut self, command_tape: &'a Vec<SpannedCommand<'a>>) {
        while self.command_index < command_tape.len() {
            check_for_ctrl_c();
            let command = &command_tape[self.command_index];
            self.interpret_command(command, &InterpreterExt {
                add_string: Interpreter::add_string,
                write_string: Interpreter::write_string,
                output_char: Interpreter::output_char,
                write_char: Interpreter::write_char,
            });
        }
    }

    fn add_string(&mut self, s: &str) {
        let mut tape = self.tape.borrow_mut();
        for c in s.chars() {
            self.tape_index += 1;
            if !tape.in_bounds(self.tape_index) {
                tape.grow();
            }
            tape.set(self.tape_index, c as i64);
        }
    }

    fn write_string(&mut self, s: &str) {
        let mut tape = self.tape.borrow_mut();
        for c in s.chars() {
            if !tape.in_bounds(self.tape_index) {
                tape.grow();
            }
            tape.set(self.tape_index, c as i64);
            self.tape_index += 1;
        }
    }

    fn output_char(&mut self) {
        let tape = self.tape.borrow();
        match char::from_u32(tape.get(self.tape_index) as u32) {
            Some('\n') => print!("\r\n"),
            Some(c) => print!("{}", c),
            None => print!(" "),
        }
        std::io::stdout().flush().unwrap();
    }

    fn write_char(&mut self, c: char) {
        let mut tape = self.tape.borrow_mut();
        if !tape.in_bounds(self.tape_index) {
            tape.grow();
        }
        tape.set(self.tape_index, c as i64);
    }
}
