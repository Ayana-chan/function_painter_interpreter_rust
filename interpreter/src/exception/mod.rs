use crate::lexer::*;

pub type Result<T> = std::result::Result<T, Exception>;

//所有异常都要实现的trait
pub trait ExceptionTrait {
    fn print_exception(&self);
}

//作为派生类的异常要实现的trait
trait BaseExceptionTrait {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Exception;
}

///异常
pub struct Exception {
    sub_exception: Box<dyn ExceptionTrait>,
}

impl BaseExceptionTrait for Exception {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Exception {
        Self {
            sub_exception
        }
    }
}

impl ExceptionTrait for Exception {
    fn print_exception(&self) {
        println!();
        self.sub_exception.print_exception();
        println!();
    }
}

///编译时异常------
struct AnalysisException {
    sub_exception: Box<dyn ExceptionTrait>,
}

impl BaseExceptionTrait for AnalysisException {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Exception {
        //指定基类为Exception
        Exception::generate(Box::new(Self {
            sub_exception
        }))
    }
}

impl ExceptionTrait for AnalysisException {
    fn print_exception(&self) {
        println!("*** Analysis Error ***");
        self.sub_exception.print_exception();
    }
}

///无法识别的Token
pub struct IllegalTokenError {
    lexeme: String,
}

impl IllegalTokenError {
    pub fn new(lexeme: &str) -> Exception {
        AnalysisException::generate(Box::new(Self {
            lexeme: String::from(lexeme)
        }))
    }
}

impl ExceptionTrait for IllegalTokenError {
    fn print_exception(&self) {
        println!("Illegal Symbol: {}", self.lexeme);
    }
}

///语法错误
pub struct SyntaxError {
    token: Token,
    expect_token_types: Vec<TokenTypeEnum>,
}

impl SyntaxError {
    pub fn new(token: &Token, expect_token_type: &[TokenTypeEnum]) -> Exception {
        AnalysisException::generate(Box::new(Self {
            token: token.clone(),
            expect_token_types: expect_token_type.to_vec(),
        }))
    }
}

impl ExceptionTrait for SyntaxError {
    fn print_exception(&self) {
        println!("Syntax Error: {:?}", self.token);
        println!("Expect: {:?}", self.expect_token_types);
        println!("Found : {:?}", self.token.token_type());
    }
}

///运行时异常------
struct RuntimeException {
    sub_exception: Box<dyn ExceptionTrait>,
}

impl BaseExceptionTrait for RuntimeException {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Exception {
        Exception::generate(Box::new(Self {
            sub_exception
        }))
    }
}

impl ExceptionTrait for RuntimeException {
    fn print_exception(&self) {
        println!("*** Runtime Error ***");
        self.sub_exception.print_exception();
    }
}

///未定义变量错误
pub struct UndefinedVariableError {
    variable_name: String,
}

impl UndefinedVariableError {
    pub fn new(variable_name: &str) -> Exception {
        RuntimeException::generate(Box::new(Self {
            variable_name: variable_name.parse().unwrap()
        }))
    }
}

impl ExceptionTrait for UndefinedVariableError {
    fn print_exception(&self) {
        println!("Undefined Variable Error: {:?}", self.variable_name);
    }
}

///参数数量不匹配错误
pub struct ArgumentNumberNotMatchError {
    func_name: String,
    received_num: usize,
    target_num: usize,
    variable_length_flag: bool
}

impl ArgumentNumberNotMatchError {
    pub fn new(func_name: &str,received_num: usize, target_num: usize,variable_length_flag: bool) -> Exception {
        RuntimeException::generate(Box::new(Self {
            func_name: String::from(func_name),
            received_num,
            target_num,
            variable_length_flag,
        }))
    }
}

impl ExceptionTrait for ArgumentNumberNotMatchError {
    fn print_exception(&self) {
        println!("Arguments' Number not Match Error:");
        println!("At Function : {:?}",self.func_name);
        print!("Expect : {:?}",self.target_num);
        if self.variable_length_flag{
            print!("+")
        }
        println!();
        println!("Receive: {:?}",self.received_num);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_exception() {
        IllegalTokenError::new("123abc")
            .print_exception();
    }
}

