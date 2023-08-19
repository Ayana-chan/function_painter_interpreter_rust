use crate::lexer;

pub trait Exception {
    fn print_exception(&self);
}

pub enum AnalysisException {
    ErrorToken(ErrorToken)
}

pub enum RuntimeException {}

pub struct ErrorToken {
    token: lexer::Token,
}

impl Exception for AnalysisException {
    fn print_exception(&self) {
        let mut exception_string = String::new();
        exception_string.push_str("\n");
        exception_string.push_str("*** Analysis Error ***");

        exception_string.push_str("\n");
        print!("{}",exception_string);
    }
}

impl Exception for RuntimeException {
    fn print_exception(&self) {
        let mut exception_string = String::new();
        exception_string.push_str("\n");
        exception_string.push_str("*** Runtime Error ***");


        exception_string.push_str("\n");
        print!("{}",exception_string);
    }
}

