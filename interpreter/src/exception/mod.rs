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
        println!();//TODO 错误定位
        self.sub_exception.print_exception();
        println!();
    }
}

///编译时异常
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

///运行时异常
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

///无法识别的Token
pub struct UnknownTokenError {
    token: Token,
}

impl UnknownTokenError {
    pub fn new(token: &Token) -> Exception {
        AnalysisException::generate(Box::new(Self {
            token: token.clone()
        }))
    }
}

impl ExceptionTrait for UnknownTokenError {
    fn print_exception(&self) {
        println!("Unknown Token: {:?}", self.token);
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
        println!("Except: {:?}", self.expect_token_types);
        println!("Found : {:?}", self.token.token_type());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_exception() {
        UnknownTokenError::new(&TokenBuilder::new()
            .token_type(TokenTypeEnum::ErrToken)
            .lexeme("12uuu").build())
            .print_exception();
    }
}

