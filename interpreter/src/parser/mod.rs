use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::fs::File;
use std::rc::Rc;

use crate::lexer::*;
use crate::exception;
use crate::exception::ExceptionTrait;

mod expression;

pub struct ParserManager {
    parser_kernel: Rc<RefCell<ParserKernel>>,
    //expression语法分析器
    expression_parser: expression::ExpressionParser,
}

impl ParserManager {
    pub fn new(file: File) -> Self {
        let parser_kernel = Rc::new(RefCell::new(ParserKernel::new(file)));
        ParserManager {
            expression_parser: expression::ExpressionParser::new(&parser_kernel),
            parser_kernel, //一定要放在后面，否则会过早夺取所有权
        }
    }

    pub fn parse(&mut self) {
        let parse_result = self.parse_program();
        if let Err(e) = parse_result {
            e.print_exception();
            panic!("Program Terminated.");
        }
    }

    ///解析语句
    fn parse_program(&mut self) -> exception::Result<()> {
        while self.parser_kernel().borrow_mut().get_curr_token_type() != TokenTypeEnum::NonToken {
            //匹配一句
            self.parse_statement()?;
            self.parser_kernel().borrow_mut().match_and_eat_token(TokenTypeEnum::Semico)?;
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> exception::Result<()> {
        match self.parser_kernel().borrow_mut().get_curr_token_type() {
            TokenTypeEnum::Origin => self.parse_origin_statement()?,
            TokenTypeEnum::Scale => self.parse_origin_statement()?,
            TokenTypeEnum::Rot => self.parse_origin_statement()?,
            TokenTypeEnum::For => self.parse_origin_statement()?,
            _ => return self.parser_kernel().borrow_mut().generate_syntax_error(&[
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

    pub fn parser_kernel(&self) -> Rc<RefCell<ParserKernel>> {
        self.parser_kernel.clone()
    }
}

///对parser底层进行一次封装
pub struct ParserKernel {
    curr_token: Token,
    lexer: Lexer,
    symbol_table: HashMap<String, f64>, //符号表，符号名->值
}

impl ParserKernel {
    pub fn new(file: File) -> Self {
        let mut lexer = Lexer::new(file);
        Self {
            curr_token: lexer.fetch_token(), //刚开始读一个以保证逻辑一致性
            lexer,
            symbol_table: HashMap::new(),
        }
    }

    ///检查当前token是否匹配目标，如果匹配则成功并读取一次token，否则会返回语法错误SyntaxError
    pub fn match_and_eat_token(&mut self, expected_token_type: TokenTypeEnum) -> exception::Result<()> {
        if self.curr_token.token_type() == expected_token_type {
            return Err(exception::SyntaxError::new(&self.curr_token, &[expected_token_type]));
        }
        self.fetch_token();
        Ok(())
    }

    ///读取一次token并覆盖当前token
    pub fn fetch_token(&mut self) {
        self.curr_token = self.lexer.fetch_token();
    }

    pub fn get_curr_token(&self) -> &Token {
        &self.curr_token
    }

    pub fn get_curr_token_type(&self) -> TokenTypeEnum {
        self.curr_token.token_type()
    }

    pub fn symbol_table(&mut self) -> &mut HashMap<String, f64> {
        &mut self.symbol_table
    }

    ///帮助自动生成语法错误，附有期望的token type
    pub fn generate_syntax_error(&self, expected_token_type: &[TokenTypeEnum]) -> exception::Result<()> {
        Err(exception::SyntaxError::new(&self.get_curr_token(), expected_token_type))
    }
}



