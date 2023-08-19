use std::fs::File;

use crate::lexer::*;
use crate::exception;
use crate::exception::{Exception, ExceptionTrait};

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

    pub fn parse(&mut self) {
        //TODO 异常接收处理
        let parse_result = self.parse_program();
        if let Err(e) = parse_result {
            e.print_exception();
            panic!("Program Terminated.");
        }
    }

    ///读取一次token并覆盖当前token
    fn fetch_token(&mut self) {
        self.curr_token = self.lexer.fetch_token();
    }

    fn get_curr_token(&self) -> &Token {
        &self.curr_token
    }

    fn get_curr_token_type(&self) -> TokenTypeEnum {
        self.curr_token.token_type()
    }

    ///检查当前token是否匹配目标，如果匹配则成功并读取一次token，否则会返回语法错误SyntaxError
    fn match_and_eat_token(&mut self, expected_token_type: TokenTypeEnum) -> exception::Result<()> {
        if self.curr_token.token_type() == expected_token_type {
            return Err(exception::SyntaxError::new(&self.curr_token, &[expected_token_type]));
        }
        self.fetch_token();
        Ok(())
    }

    ///帮助自动生成语法错误，附有期望的token type
    fn generate_syntax_error(&self, expected_token_type: &[TokenTypeEnum]) -> exception::Result<()> {
        Err(exception::SyntaxError::new(&self.get_curr_token(), expected_token_type))
    }

    ///解析语句
    fn parse_program(&mut self) -> exception::Result<()> {
        while self.get_curr_token_type() != TokenTypeEnum::NonToken {
            //匹配一句
            self.parse_statement()?;
            self.match_and_eat_token(TokenTypeEnum::Semico)?;
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> exception::Result<()> {
        match self.get_curr_token_type() {
            TokenTypeEnum::Origin => self.parse_origin_statement()?,
            TokenTypeEnum::Scale => self.parse_origin_statement()?,
            TokenTypeEnum::Rot => self.parse_origin_statement()?,
            TokenTypeEnum::For => self.parse_origin_statement()?,
            _ => return self.generate_syntax_error(&[
                TokenTypeEnum::Origin, TokenTypeEnum::Scale, TokenTypeEnum::Rot, TokenTypeEnum::For
            ]),
        }
        Ok(())
    }

    fn parse_origin_statement(&mut self) -> exception::Result<()> {
        Ok(())
    }

    fn parse_scale_statement(&mut self) -> exception::Result<()> {
        Ok(())
    }

    fn parse_rot_statement(&mut self) -> exception::Result<()> {
        Ok(())
    }

    fn parse_for_statement(&mut self) -> exception::Result<()> {
        Ok(())
    }
}







