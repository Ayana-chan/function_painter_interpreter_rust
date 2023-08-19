use std::fs::File;

use crate::lexer::*;
use crate::exception;

mod expression;

pub struct Parser {
    curr_token: Token,
    lexer: Lexer,
}

impl Parser {
    pub fn new(file: File) -> Self {
        let mut lexer = Lexer::new(file);
        Self {
            curr_token: lexer.fetch_token(), //刚开始读一个以保证逻辑一致性
            lexer,
        }
    }

    pub fn parse(&mut self) {}

    ///读取一次token并覆盖当前token
    fn fetch_token(&mut self) {
        self.curr_token = self.lexer.fetch_token();
    }

    fn get_curr_token(&self) -> &Token {
        &self.curr_token
    }

    ///检查当前token是否匹配目标，如果匹配则成功并读取一次token，否则会返回语法错误SyntaxError
    fn match_and_eat_token(&mut self, expected_token_type: TokenTypeEnum) -> exception::Result<()> {
        if self.curr_token.token_type() == expected_token_type {
            return Err(exception::SyntaxError::new(&self.curr_token, expected_token_type));
        }
        self.fetch_token();
        return Ok(());
    }
}







