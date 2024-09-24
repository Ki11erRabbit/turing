use std::str::CharIndices;

use crate::ast::{Command, SpannedCommand};


struct Lexer<'a> {
    input: &'a str,
    char_indices: std::iter::Peekable<CharIndices<'a>>,
    end_stack: Vec<()>,
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input,
            char_indices: input.char_indices().peekable(),
            end_stack: Vec::new(),
        }
    }

    fn parse_if(&mut self) -> (Vec<SpannedCommand<'a>>, Option<Vec<SpannedCommand<'a>>>, usize) {
        self.end_stack.push(());
        let stack_len = self.end_stack.len();
        let mut commands = Vec::new();
        let mut else_commands = None;
        let mut end = 0;
        while let Some(command) = self.next() {
            end = command.end;
            match command.command {
                Command::FunctionCall(name) => {
                    if name == "else" {
                        let mut else_commands_vec = Vec::new();
                        while let Some(command) = self.next() {
                            end = command.end;
                            match command.command {
                                Command::FunctionCall(name) => {
                                    if name == "end" && self.end_stack.len() == stack_len {
                                        self.end_stack.pop();
                                        break;
                                    } else if name != "end" {
                                        else_commands_vec.push(command);
                                    }
                                }
                                _ => else_commands_vec.push(command),
                            }
                        }
                        else_commands = Some(else_commands_vec);
                        break;
                    } else if name == "end" && self.end_stack.len() == stack_len {
                        self.end_stack.pop();
                        break;
                    } else if name != "end" {
                        commands.push(command);
                    }
                }
                _ => commands.push(command),
            }
        }
        (commands, else_commands, end)
    }

    fn parse_while(&mut self) -> (Vec<SpannedCommand<'a>>, usize) {
        self.end_stack.push(());
        let stack_len = self.end_stack.len();
        let mut commands = Vec::new();
        let mut end = 0;
        while let Some(command) = self.next() {
            end = command.end;
            match command.command {
                Command::FunctionCall(name) => {
                    if name == "end" && self.end_stack.len() == stack_len {
                        self.end_stack.pop();
                        break;
                    } else if name != "end" {
                        commands.push(command);
                    }
                }
                _ => commands.push(command),
            }
        }
        (commands, end)
    }

    fn parse_loop(&mut self) -> (Vec<SpannedCommand<'a>>, usize) {
        self.end_stack.push(());
        let stack_len = self.end_stack.len();
        let mut commands = Vec::new();
        let mut end = 0;
        while let Some(command) = self.next() {
            end = command.end;
            match command.command {
                Command::FunctionCall(name) => {
                    if name == "end" && self.end_stack.len() == stack_len{
                        self.end_stack.pop();
                        break;
                    } else if name != "end" {
                        commands.push(command);
                    }
                }
                _ => commands.push(command),
            }
        }
        (commands, end)
    }

    fn parse_function_definition(&mut self) -> (&'a str, Vec<SpannedCommand<'a>>, usize) {
        self.end_stack.push(());
        let stack_len = self.end_stack.len();
        let mut commands = Vec::new();
        let mut end;
        let name = match self.next() {
            Some(command) => {
                end = command.end;
                match command.command {
                    Command::FunctionCall(name) => name,
                    _ => panic!("Expected function name"),
                }
            }
            None => panic!("Expected function name"),
        };
        while let Some(command) = self.next() {
            end = command.end;
            match command.command {
                Command::FunctionCall(name) => {
                    if name == "end" && self.end_stack.len() == stack_len {
                        self.end_stack.pop();
                        break;
                    } else if name != "end" {
                        commands.push(command);
                    }
                }
                _ => commands.push(command),
            }
        }
        (name, commands, end)
    }

    fn parse_get_function(&mut self) -> (&'a str, usize) {
        match self.next() {
            Some(command) => {
                match command.command {
                    Command::FunctionCall(name) => (name, command.end),
                    _ => panic!("Expected function name"),
                }
            }
            None => panic!("Expected function name"),
        }
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = SpannedCommand<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end;
        let start;
        let token = match self.char_indices.next() {
            Some((i, c)) => {
                start = i;
                end = i + c.len_utf8();
                match c {
                    c if c.is_alphabetic() || c == '_' => {
                        while let Some((i, c)) = self.char_indices.peek() {
                            let i = *i;
                            let c = *c;
                            if c.is_alphabetic() || c == '_' {
                                self.char_indices.next();
                                end = i + c.len_utf8();
                            } else {
                                break;
                            }
                        }
                        let command = &self.input[start..end];
                        match command {
                            "if" => {
                                let (then_commands, else_commands, end) = self.parse_if();
                                SpannedCommand {
                                    command: Command::If(then_commands, else_commands),
                                    start,
                                    end,
                                }
                            }
                            "while" => {
                                let (commands, end) = self.parse_while();
                                SpannedCommand {
                                    command: Command::While(commands),
                                    start,
                                    end,
                                }
                            }
                            "loop" => {
                                let (commands, end) = self.parse_loop();
                                SpannedCommand {
                                    command: Command::Loop(commands),
                                    start,
                                    end,
                                }
                            }
                            "fun" => {
                                let (name, commands, end) = self.parse_function_definition();
                                SpannedCommand {
                                    command: Command::FunctionDefinition(name, commands),
                                    start,
                                    end,
                                }
                            }
                            "getfun" => {
                                let (name, end) = self.parse_get_function();
                                SpannedCommand {
                                    command: Command::GetFunction(name),
                                    start,
                                    end,
                                }
                            }
                            "call" => {
                                SpannedCommand {
                                    command: Command::CallFunction,
                                    start,
                                    end,
                                }
                            }
                            _ => {
                                SpannedCommand {
                                    command: Command::FunctionCall(command),
                                    start,
                                    end,
                                }
                            }
                        }
                    }
                    '0'..='9' => {
                        while let Some((i, c)) = self.char_indices.peek() {
                            let i = *i;
                            let c = *c;
                            if c.is_numeric() {
                                self.char_indices.next();
                                end = i + c.len_utf8();
                            } else {
                                break;
                            }
                        }
                        let command = &self.input[start..end];
                        SpannedCommand {
                            command: Command::AddInteger(command.parse().unwrap()),
                            start,
                            end,
                        }
                    }
                    '"' => {
                        let mut found_backslash = false;
                        while let Some((i, c)) = self.char_indices.next() {
                            if c == '"' && !found_backslash {
                                end = i;
                                break;
                            } else if c == '\\' {
                                found_backslash = true;
                            } else if found_backslash {
                                found_backslash = false;
                            } else {
                                found_backslash = false;
                            }
                        }
                        let acc = &self.input[start + 1..end];
                        SpannedCommand {
                            command: Command::AddString(acc),
                            start,
                            end,
                        }
                    }
                    '+' => {
                        if let Some((i, '>')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::RightAdd(None),
                                start,
                                end,
                            }
                        } else if let Some((_, _)) = self.char_indices.peek() {
                            let mut acc = String::new();
                            while let Some((i, c)) = self.char_indices.peek() {
                                let i = *i;
                                let c = *c;
                                if c == '>' {
                                    self.char_indices.next();
                                    end = i + 1;
                                    break;
                                } else {
                                    acc.push(c);
                                    self.char_indices.next();
                                    end = i + 1;
                                }
                            }
                            SpannedCommand {
                                command: Command::RightAdd(Some(acc.parse().unwrap())),
                                start,
                                end,
                            }
                        } else {
                            SpannedCommand {
                                command: Command::Increment,
                                start,
                                end,
                            }
                        }
                    }
                    '-' => {
                        if let Some((i, '>')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::RightSubtract(None),
                                start,
                                end,
                            }
                        } else if let Some((_, c)) = self.char_indices.peek() {
                            let c = *c;
                            if c.is_numeric() {
                                let mut acc = String::new();
                                while let Some((i, c)) = self.char_indices.peek() {
                                    let i = *i;
                                    let c = *c;
                                    if c == '>' {
                                        self.char_indices.next();
                                        end = i + 1;
                                        break;
                                    } else {
                                        acc.push(c);
                                        self.char_indices.next();
                                        end = i + 1;
                                    }
                                }
                                SpannedCommand {
                                    command: Command::RightSubtract(Some(acc.parse().unwrap())),
                                    start,
                                    end,
                                }
                            } else {
                                SpannedCommand {
                                    command: Command::Decrement,
                                    start,
                                    end,
                                }
                            }   
                        } else {
                            SpannedCommand {
                                command: Command::Decrement,
                                start,
                                end,
                            }
                        }
                    }
                    '*' => {
                        if let Some((i, '>')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::RightMultiply(None),
                                start,
                                end,
                            }
                        } else if let Some((_, _)) = self.char_indices.peek() {
                            let mut acc = String::new();
                            while let Some((i, c)) = self.char_indices.peek() {
                                let i = *i;
                                let c = *c;
                                if c == '>' {
                                    self.char_indices.next();
                                    end = i + 1;
                                    break;
                                } else {
                                    acc.push(c);
                                    self.char_indices.next();
                                    end = i + 1;
                                }
                            }
                            SpannedCommand {
                                command: Command::RightMultiply(Some(acc.parse().unwrap())),
                                start,
                                end,
                            }
                        } else {
                            SpannedCommand {
                                command: Command::LeftMultiply(None),
                                start,
                                end,
                            }
                        }
                    }
                    '/' => {
                        if let Some((i, '>')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::RightDivide(None),
                                start,
                                end,
                            }
                        } else if let Some((_, _)) = self.char_indices.peek() {
                            let mut acc = String::new();
                            while let Some((i, c)) = self.char_indices.peek() {
                                let i = *i;
                                let c = *c;
                                if c == '>' {
                                    self.char_indices.next();
                                    end = i + 1;
                                    break;
                                } else {
                                    acc.push(c);
                                    self.char_indices.next();
                                    end = i + 1;
                                }
                            }
                            SpannedCommand {
                                command: Command::RightDivide(Some(acc.parse().unwrap())),
                                start,
                                end,
                            }
                        } else {
                            SpannedCommand {
                                command: Command::LeftDivide(None),
                                start,
                                end,
                            }
                        }
                    }
                    '%' => {
                        if let Some((i, '>')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::RightModulo(None),
                                start,
                                end,
                            }
                        } else if let Some((_, _)) = self.char_indices.peek() {
                            let mut acc = String::new();
                            while let Some((i, c)) = self.char_indices.peek() {
                                let i = *i;
                                let c = *c;
                                if c == '>' {
                                    self.char_indices.next();
                                    end = i + 1;
                                    break;
                                } else {
                                    acc.push(c);
                                    self.char_indices.next();
                                    end = i + 1;
                                }
                            }
                            SpannedCommand {
                                command: Command::RightModulo(Some(acc.parse().unwrap())),
                                start,
                                end,
                            }
                        } else {
                            SpannedCommand {
                                command: Command::LeftModulo(None),
                                start,
                                end,
                            }
                        }
                    }
                    '<' => {
                        if let Some((i, '+')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::LeftAdd(None),
                                start,
                                end,
                            }
                        } else if let Some((i, '-')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::LeftSubtract(None),
                                start,
                                end,
                            }
                        } else if let Some((i, '*')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::LeftMultiply(None),
                                start,
                                end,
                            }
                        } else if let Some((i, '/')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::LeftDivide(None),
                                start,
                                end,
                            }
                        } else if let Some((i, '%')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::LeftModulo(None),
                                start,
                                end,
                            }
                        } else if let Some((i, '|')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::ReadMoveLeft,
                                start,
                                end,
                            }
                        } else if let Some((_, c)) = self.char_indices.peek() {
                            let c = *c;
                            if c.is_numeric() {
                                let mut acc = String::new();
                                let mut operation = "";
                                while let Some((i, c)) = self.char_indices.peek() {
                                    let c = *c;
                                    let i = *i;
                                    if c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == ')' {
                                        self.char_indices.next();
                                        end = i + 1;
                                        match c {
                                            '+' => operation = "add",
                                            '-' => operation = "sub",
                                            '*' => operation = "mul",
                                            '/' => operation = "div",
                                            '%' => operation = "mod",
                                            ')' => operation = "move",
                                            _ => panic!("Invalid operation"),
                                        }
                                        break;
                                    } else {
                                        acc.push(c);
                                        self.char_indices.next();
                                        end = i + 1;
                                    }
                                }
                                let command = match operation {
                                    "add" => Command::LeftAdd(Some(acc.parse().unwrap())),
                                    "sub" => Command::LeftSubtract(Some(acc.parse().unwrap())),
                                    "mul" => Command::LeftMultiply(Some(acc.parse().unwrap())),
                                    "div" => Command::LeftDivide(Some(acc.parse().unwrap())),
                                    "mod" => Command::LeftModulo(Some(acc.parse().unwrap())),
                                    "move" => Command::MoveNLeft(acc.parse().unwrap()),
                                    _ => panic!("Invalid operation"),
                                };
                                SpannedCommand {
                                    command,
                                    start,
                                    end,
                                }
                            } else {
                                SpannedCommand {
                                    command: Command::MoveLeft,
                                    start,
                                    end,
                                }
                            }
                        } else {
                            SpannedCommand {
                                command: Command::MoveLeft,
                                start,
                                end,
                            }
                        }
                    }
                    '>' => SpannedCommand {
                        command: Command::MoveRight,
                        start,
                        end,
                    },
                    '|' => {
                        if let Some((i, '>')) = self.char_indices.peek() {
                            let i = *i;
                            self.char_indices.next();
                            end = i + 1;
                            SpannedCommand {
                                command: Command::ReadMoveRight,
                                start,
                                end,
                            }
                        } else {
                            panic!("Expected '>' after '|'");
                        }
                    }
                    '(' => {
                        let mut acc = String::new();
                        while let Some((i, c)) = self.char_indices.peek() {
                            let c = *c;
                            let i = *i;
                            if c == '>' {
                                self.char_indices.next();
                                end = i + 1;
                                break;
                            } else {
                                acc.push(c);
                                self.char_indices.next();
                                end = i + 1;
                            }
                        }
                        SpannedCommand {
                            command: Command::MoveNRight(acc.parse().unwrap()),
                            start,
                            end,
                        }
                    }
                    '.' => SpannedCommand {
                        command: Command::OutputNumber,
                        start,
                        end,
                    },
                    ',' => SpannedCommand {
                        command: Command::OutputChar,
                        start,
                        end,
                    },
                    '?' => SpannedCommand {
                        command: Command::ReadKey,
                        start,
                        end,
                    },
                    '[' => {
                        if let Some((i, '"')) = self.char_indices.peek() {
                            let i = *i;
                            let string_start = i;
                            self.char_indices.next();
                            end = i + 1;
                            let mut found_backslash = false;
                            while let Some((i, c)) = self.char_indices.next() {
                                if c == '"' && !found_backslash {
                                    end = i;
                                    break;
                                } else if c == '\\' {
                                    found_backslash = true;
                                } else if found_backslash {
                                    found_backslash = false;
                                } else {
                                    found_backslash = false;
                                }
                            }
                            if let Some((end_i, ']')) = self.char_indices.next() {
                                let acc = &self.input[string_start + 1..end];
                                SpannedCommand {
                                    command: Command::WriteString(acc),
                                    start,
                                    end: end_i + 1,
                                }
                            } else {
                                panic!("Expected ']' after string");
                            }
                        } else {
                            let mut acc = String::new();
                            while let Some((i, c)) = self.char_indices.peek() {
                                let c = *c;
                                let i = *i;
                                if c == ']' {
                                    self.char_indices.next();
                                    end = i + 1;
                                    break;
                                } else {
                                    acc.push(c);
                                    self.char_indices.next();
                                    end = i + 1;
                                }
                            }
                            SpannedCommand {
                                command: Command::WriteInteger(acc.parse().unwrap()),
                                start,
                                end,
                            }
                        }
                    }
                    '#' => {
                        while let Some((i, c)) = self.char_indices.peek() {
                            let c = *c;
                            let i = *i;
                            if c == '\n' {
                                self.char_indices.next();
                                end = i + 1;
                                break;
                            } else {
                                self.char_indices.next();
                                end = i + 1;
                            }
                        }
                        SpannedCommand {
                            command: Command::Comment,
                            start,
                            end,
                        }
                    }
                    c if c.is_whitespace() => {
                        while let Some((_, c)) = self.char_indices.peek() {
                            let c = *c;
                            //let i = *i;
                            if c.is_whitespace() {
                                self.char_indices.next();
                                //end = i + 1;
                            } else {
                                break;
                            }
                        }
                        return self.next();
                    }
                    _ => panic!("Unexpected character"),
                }
            }
            None => return None,
        };
        Some(token)
    }
}


pub fn parse<'a>(input: &'a str) -> Vec<SpannedCommand<'a>> {
    Lexer::new(input).collect()
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_add_integer() {
        let input = "123";
        let expected = vec![
            SpannedCommand {
                command: Command::AddInteger(123),
                start: 0,
                end: 3,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_add_string() {
        let input = "\"hello\"";
        let expected = vec![
            SpannedCommand {
                command: Command::AddString("hello"),
                start: 0,
                end: 6,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_write_integer() {
        let input = "[123]";
        let expected = vec![
            SpannedCommand {
                command: Command::WriteInteger(123),
                start: 0,
                end: 5,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_write_string() {
        let input = "[\"hello\"]";
        let expected = vec![
            SpannedCommand {
                command: Command::WriteString("hello"),
                start: 0,
                end: 9,
            },
        ];
        assert_eq!(parse(input), expected);
    }
    
    #[test]
    fn test_parse() {
        let input = "fun add_one +> end";
        let expected = vec![
            SpannedCommand {
                command: Command::FunctionDefinition("add_one", vec![SpannedCommand {
                    command: Command::RightAdd(None),
                    start: 12,
                    end: 14,
                }]),
                start: 0,
                end: 18,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_if() {
        let input = "if add_one +> else add_two -> end";
        let expected = vec![
            SpannedCommand {
                command: Command::If(
                    vec![SpannedCommand {
                        command: Command::RightAdd(None),
                        start: 11,
                        end: 13,
                    }],
                    Some(vec![SpannedCommand {
                        command: Command::RightSubtract(None),
                        start: 27,
                        end: 29,
                    }]),
                ),
                start: 0,
                end: 33,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_single_if() {
        let input = "if add_one +> end";
        let expected = vec![
            SpannedCommand {
                command: Command::If(
                    vec![SpannedCommand {
                        command: Command::RightAdd(None),
                        start: 11,
                        end: 13,
                    }],
                    None,
                ),
                start: 0,
                end: 17,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_while() {
        let input = "while add_one +> end";
        let expected = vec![
            SpannedCommand {
                command: Command::While(vec![SpannedCommand {
                    command: Command::RightAdd(None),
                    start: 14,
                    end: 16,
                }]),
                start: 0,
                end: 20,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_function_definition() {
        let input = "fun add_one +> end";
        let expected = vec![
            SpannedCommand {
                command: Command::FunctionDefinition("add_one", vec![SpannedCommand {
                    command: Command::RightAdd(None),
                    start: 12,
                    end: 14,
                }]),
                start: 0,
                end: 18,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_get_function() {
        let input = "getfun add_one";
        let expected = vec![
            SpannedCommand {
                command: Command::GetFunction("add_one"),
                start: 0,
                end: 14,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_call_function() {
        let input = "call";
        let expected = vec![
            SpannedCommand {
                command: Command::CallFunction,
                start: 0,
                end: 4,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_output_number() {
        let input = ".";
        let expected = vec![
            SpannedCommand {
                command: Command::OutputNumber,
                start: 0,
                end: 1,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_output_char() {
        let input = ",";
        let expected = vec![
            SpannedCommand {
                command: Command::OutputChar,
                start: 0,
                end: 1,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_read_key() {
        let input = "?";
        let expected = vec![
            SpannedCommand {
                command: Command::ReadKey,
                start: 0,
                end: 1,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_comment() {
        let input = "# this is a comment\n";
        let expected = vec![
            SpannedCommand {
                command: Command::Comment,
                start: 0,
                end: 20,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_move_left() {
        let input = "<";
        let expected = vec![
            SpannedCommand {
                command: Command::MoveLeft,
                start: 0,
                end: 1,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_move_right() {
        let input = ">";
        let expected = vec![
            SpannedCommand {
                command: Command::MoveRight,
                start: 0,
                end: 1,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_read_move_left() {
        let input = "|>";
        let expected = vec![
            SpannedCommand {
                command: Command::ReadMoveRight,
                start: 0,
                end: 2,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_read_move_right() {
        let input = "<|";
        let expected = vec![
            SpannedCommand {
                command: Command::ReadMoveLeft,
                start: 0,
                end: 2,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_move_n_left() {
        let input = "<123)";
        let expected = vec![
            SpannedCommand {
                command: Command::MoveNLeft(123),
                start: 0,
                end: 5,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_move_n_right() {
        let input = "(123>";
        let expected = vec![
            SpannedCommand {
                command: Command::MoveNRight(123),
                start: 0,
                end: 5,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_increment() {
        let input = "+";
        let expected = vec![
            SpannedCommand {
                command: Command::Increment,
                start: 0,
                end: 1,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_decrement() {
        let input = "-";
        let expected = vec![
            SpannedCommand {
                command: Command::Decrement,
                start: 0,
                end: 1,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_left_add() {
        let input = "<+";
        let expected = vec![
            SpannedCommand {
                command: Command::LeftAdd(None),
                start: 0,
                end: 2,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_left_add_with_value() {
        let input = "<123+";
        let expected = vec![
            SpannedCommand {
                command: Command::LeftAdd(Some(123)),
                start: 0,
                end: 5,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_nested_if() {
        let input = "if add_one +> else if add_two -> end end";
        let expected = vec![
            SpannedCommand {
                command: Command::If(
                    vec![SpannedCommand {
                        command: Command::RightAdd(None),
                        start: 11,
                        end: 13,
                    }],
                    Some(vec![SpannedCommand {
                        command: Command::If(
                            vec![SpannedCommand {
                                command: Command::RightSubtract(None),
                                start: 30,
                                end: 32,
                            }],
                            None,
                        ),
                        start: 19,
                        end: 40,
                    }]),
                ),
                start: 0,
                end: 40,
            },
        ];
        assert_eq!(parse(input), expected);
    }
}
