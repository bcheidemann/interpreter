use std::io::*;
use std::{env, fs, io};

use lib::environment::Environment;
use lib::parser::LiteralValue;

mod lib;

fn environment_with_globals() -> Environment {
    let mut env = Environment::new();

    env.assign(
        &"VERSION".to_string(),
        LiteralValue::String(env!("CARGO_PKG_VERSION").to_string()),
    );

    env
}

fn environment_from_args(args: &Vec<String>) -> Environment {
    let mut env = environment_with_globals();

    for (i, arg) in args.iter().enumerate() {
        let identifier = format!("ARG_{}", i).to_string();
        let value = {
            if let Ok(value) = arg.parse() {
                LiteralValue::Number(value)
            } else {
                LiteralValue::String(arg.to_string())
            }
        };
        env.assign(&identifier, value);
    }

    env
}

fn repl() {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    let mut interpreter = lib::interpreter::Interpreter::new(environment_with_globals());

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
        let declarations = parser.parse();
        interpreter.run(&declarations);
    }
}

fn run_script(script_file: &String, environment: Environment) {
    let input = fs::read_to_string(script_file).expect("Something went wrong reading the file");
    let mut interpreter = lib::interpreter::Interpreter::new(environment);
    let mut scanner = lib::scanner::Scanner::from_source(&input);
    let mut parser = lib::parser::Parser::new(scanner.scan_tokens().expect("Failed at scanner"));
    let program = parser.parse();
    interpreter.run(&program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let no_args = args.len();

    match no_args {
        1 => repl(),
        _ => {
            let env = environment_from_args(&args);
            run_script(&args[1], env);
        }
    }

    if no_args == 0 {
        repl();
    }
}
