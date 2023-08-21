use std::fs::File;

mod lexer;
mod parser;
mod exception;

pub fn interpret(file: File) -> Result<Vec<(f64, f64)>, ()> {
    let mut parser = parser::ParserManager::new(file);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        let file = File::open("parse_test.txt").unwrap();
        let result = interpret(file);
        println!("Result: {:?}", result);
    }
}
