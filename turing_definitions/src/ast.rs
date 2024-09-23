

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct SpannedCommand<'a> {
    pub command: Command<'a>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Command<'a> {
    AddInteger(i64),
    AddString(&'a str),
    WriteInteger(i64),
    WriteString(&'a str),
    // Movement
    MoveLeft,
    MoveRight,
    ReadMoveLeft,
    ReadMoveRight,
    MoveNLeft(u64),
    MoveNRight(u64),
    // Arithmetic
    Increment,
    Decrement,
    LeftAdd(Option<usize>),
    RightAdd(Option<usize>),
    LeftSubtract(Option<usize>),
    RightSubtract(Option<usize>),
    LeftMultiply(Option<usize>),
    RightMultiply(Option<usize>),
    LeftDivide(Option<usize>),
    RightDivide(Option<usize>),
    LeftModulo(Option<usize>),
    RightModulo(Option<usize>),
    // Control Flow
    If(Vec<SpannedCommand<'a>>, Option<Vec<SpannedCommand<'a>>>),
    While(Vec<SpannedCommand<'a>>),
    Loop(Vec<SpannedCommand<'a>>),
    FunctionDefinition(&'a str, Vec<SpannedCommand<'a>>),
    FunctionCall(&'a str),
    GetFunction(&'a str),
    CallFunction,
    // IO
    OutputNumber,
    OutputChar,
    ReadKey,
    Comment,
}
