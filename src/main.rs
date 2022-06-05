use std::io::*;
use std::{env, fs, io};

mod lib;

fn repl() {
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

fn run_script(script_file: &String) {
    let input = fs::read_to_string(script_file).expect("Something went wrong reading the file");
    let mut interpreter = lib::interpreter::Interpreter::new(lib::parser::Program::new());
    let mut scanner = lib::scanner::Scanner::from_source(&input);
    let mut parser = lib::parser::Parser::new(scanner.scan_tokens().expect("Failed at scanner"));
    let mut declarations = parser.parse().to_declarations();
    interpreter.evaluate_declarations(&mut declarations);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let no_args = args.len();

    match no_args {
        1 => repl(),
        2 => run_script(&args[1]),
        _ => {
            println!("Incorrect usage: should be either `cargo run` or `cargo run <script_file>`.");
        }
    }

    if no_args == 0 {
        repl();
    }
}
