mod lib;

fn main() {
    let mut scanner = lib::scanner::Scanner::from_source("123 + 5");
    let mut parser = lib::parser::Parser::new(scanner.scan_tokens().unwrap());
    let expression = parser.parse();
    let result = lib::interpreter::evaluate_expression(expression);
    println!("{result:?}");
}
