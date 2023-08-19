use crate::lexer;

pub type Result<T> = std::result::Result<T, Exception>;

//所有异常都要实现的trait
pub trait ExceptionTrait {
    fn print_exception(&self);
}

//作为派生类的异常要实现的trait
trait BaseExceptionTrait {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Box<dyn ExceptionTrait>;
}

///异常
pub struct Exception {
    sub_exception: Box<dyn ExceptionTrait>,
}

impl BaseExceptionTrait for Exception {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Box<dyn ExceptionTrait> {
        Box::new(Self {
            sub_exception
        })
    }
}

impl ExceptionTrait for Exception {
    fn print_exception(&self) {
        println!();
        self.sub_exception.print_exception();
        println!();
    }
}

///编译时异常
pub struct AnalysisException {
    sub_exception: Box<dyn ExceptionTrait>,
}

impl BaseExceptionTrait for AnalysisException {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Box<dyn ExceptionTrait> {
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

///运行时异常
pub struct RuntimeException {
    sub_exception: Box<dyn ExceptionTrait>,
}

impl BaseExceptionTrait for RuntimeException {
    fn generate(sub_exception: Box<dyn ExceptionTrait>) -> Box<dyn ExceptionTrait> {
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

///无法识别的Token
pub struct UnknownTokenError {
    token: lexer::Token,
}

impl UnknownTokenError {
    pub fn new(token: &lexer::Token) -> Box<dyn ExceptionTrait> {
        AnalysisException::generate(Box::new(Self {
            token: token.clone()
        }))
    }
}

impl ExceptionTrait for UnknownTokenError {
    fn print_exception(&self) {
        println!("ErrorToken: {:?}", self.token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_exception() {
        UnknownTokenError::new(&lexer::TokenBuilder::new()
            .token_type(lexer::TokenTypeEnum::ErrToken)
            .lexeme("12uuu").build())
            .print_exception();
    }
}

