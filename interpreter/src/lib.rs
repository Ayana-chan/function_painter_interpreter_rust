use std::fs::File;

mod lexer;
mod parser;
mod exception;

pub struct Interpreter {
    parser: parser::ParserManager,
}

impl Interpreter {
    pub fn new(file: File) -> Self {
        Interpreter {
            parser: parser::ParserManager::new(file),
        }
    }

    pub fn set_coordinate_range(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) {
        self.parser.set_coordinate_range(min_x, max_x, min_y, max_y);
    }

    ///开始解释，返回结果集
    pub fn interpret(&mut self) -> Result<Vec<(f64, f64)>, ()> {
        self.parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        let file = File::open("parse_test.txt").unwrap();
        let mut interpreter_obj = Interpreter::new(file);
        // interpreter_obj.set_coordinate_range(-100.0, 200.0, -100.0, 200.0);
        let point_result = interpreter_obj.interpret().unwrap();
    }
}
