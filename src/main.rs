mod lib;

fn main() {
    let mut scanner = lib::scanner::Scanner::from_source("()");
    let mut parser = lib::parser::Parser::new(scanner.scan_tokens().unwrap());
    let result = parser.parse();
    println!("{result:?}");
}
