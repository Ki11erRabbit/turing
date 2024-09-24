# Turing Machine Simulator
## How to install
1. First, install a [Rust compiler](https://www.rust-lang.org/tools/install).
2. Then clone the repo with a `git clone https://github.com/Ki11erRabbit/turing/`
3. Then run `cargo build` to compile
4. Then run `cargo run` to run the program


## Commandline arguments
To pass commandline arguments to the program from `cargo run` you will need to put a `--` after the command and then any flags you want.

The current commandline arguments are:
* `-h` or `--help`
* `-f` or `--file` to pass in a file to be executed
* `-m` or `--manual` to display the manual and quit


## Manual
Numbers:
  - Numbers move the tape head over by one and set that cell to the number
  - Example: 5

Strings:
  - Strings move the tape head over by each character and set that cell to the UTF32 value of the character
  - Example: "Hello"

Writing in Place:
  - Writing in place rather than moving the tape head is done by wrapping the value in brackets.
  - List will still move the tape head
  - Example: \[5\]

Commands:
  - <: Move the tape head to the left
  - \>: Move the tape head to the right
  - <|: Move the tape head to the left by the value of the current cell
  - |>: Move the tape head to the right by the value of the current cell
  - <#): Move the tape head to the left by #
  - (#>): Move the tape head to the right by #
  - +: Increment the value of the current cell
  - -: Decrement the value of the current cell

Arithmetic:
Uses +, -, *, /, %
  - <\_: A _ B = A <_ B
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

See this website for more information: https://killerrabbit.xyz/pl/turing/
