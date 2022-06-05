use std::io;
use std::io::*;

mod lib;

fn main() {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    let mut interpreter = lib::interpreter::Interpreter::new(lib::parser::Program::new());

    loop {
        let mut input = String::new();

        stdout.write(">> ".as_bytes()).unwrap();
        stdout.flush().unwrap();
        stdin
            .read_line(&mut input)
            .expect("error: unable to read user input");

        if input == "exit\n" {
            break;
        }

        let mut scanner = lib::scanner::Scanner::from_source(&input);
        let mut parser =
            lib::parser::Parser::new(scanner.scan_tokens().expect("Failed at scanner"));
        let mut declarations = parser.parse().to_declarations();
        interpreter.evaluate_declarations(&mut declarations);
    }
}
