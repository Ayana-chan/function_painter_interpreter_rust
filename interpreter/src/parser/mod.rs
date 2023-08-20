use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::rc::Rc;

use crate::lexer::*;
use crate::exception;
use crate::exception::ExceptionTrait;

mod expression;
mod point_manager;

pub struct ParserManager {
    parser_kernel: Rc<RefCell<ParserKernel>>,
    //expression语法分析器
    expression_parser: expression::ExpressionParser,
    //点生成与管理器
    point_manager: point_manager::PointManager,
}

impl ParserManager {
    pub fn new(file: File) -> Self {
        let parser_kernel = Rc::new(RefCell::new(ParserKernel::new(file)));
        ParserManager {
            expression_parser: expression::ExpressionParser::new(&parser_kernel),
            parser_kernel, //一定要放在后面，否则会过早夺取所有权
            point_manager: point_manager::PointManager::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<(f64, f64)> {
        let parse_result = self.parse_program();
        if let Err(e) = parse_result {
            e.print_exception();
            panic!("Program Terminated due to an Error.");
        }
        //返回结果点集
        self.point_manager().point_storage()
    }

    ///分析程序
    fn parse_program(&mut self) -> exception::Result<()> {
        //EOF前一直读取
        while self.get_mut_parser_kernel().get_curr_token_type() != TokenTypeEnum::NonToken {
            //匹配一句
            self.parse_statement()?;
            self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Semico)?;
        }
        Ok(())
    }

    ///分析语句
    fn parse_statement(&mut self) -> exception::Result<()> {
        println!("Debug: parse a statement, begin token: {:?}", self.get_mut_parser_kernel().get_curr_token());
        let token_type = self.get_mut_parser_kernel().get_curr_token_type();
        match token_type {
            TokenTypeEnum::Origin => self.parse_origin_statement()?,
            TokenTypeEnum::Scale => self.parse_scale_statement()?,
            TokenTypeEnum::Rot => self.parse_rot_statement()?,
            TokenTypeEnum::For => self.parse_for_statement()?,
            _ => return self.get_mut_parser_kernel().generate_syntax_error(&[
                TokenTypeEnum::Origin, TokenTypeEnum::Scale, TokenTypeEnum::Rot, TokenTypeEnum::For
            ]),
        }
        Ok(())
    }

    ///ORIGIN IS (ex1,ex2)
    fn parse_origin_statement(&mut self) -> exception::Result<()> {
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Origin)?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Is)?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::LBracket)?;
        let x = self.expression_parser().parse_expression_entrance()?.calculate();
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Comma)?;
        let y = self.expression_parser().parse_expression_entrance()?.calculate();
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::RBracket)?;

        self.point_manager().set_var_origin((x, y));

        Ok(())
    }

    ///SCALE IS (ex1,ex2)
    fn parse_scale_statement(&mut self) -> exception::Result<()> {
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Scale)?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Is)?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::LBracket)?;
        let x = self.expression_parser().parse_expression_entrance()?.calculate();
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Comma)?;
        let y = self.expression_parser().parse_expression_entrance()?.calculate();
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::RBracket)?;

        self.point_manager().set_var_scale((x, y));

        Ok(())
    }

    ///ROT IS ex
    fn parse_rot_statement(&mut self) -> exception::Result<()> {
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Rot)?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Is)?;
        let r = self.expression_parser().parse_expression_entrance()?.calculate();

        self.point_manager().set_var_rot(r);

        Ok(())
    }

    ///FOR var FROM ex1 TO ex2 STEP ex3 DRAW(ex4,ex5)
    fn parse_for_statement(&mut self) -> exception::Result<()> {
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::For)?;
        //这里暂时只能是T。这也是为什么规定T和variable分开，如果功能拓展了就能一视同仁
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::T)?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::From)?;
        let from = self.expression_parser().parse_expression_entrance()?.calculate();
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::To)?;
        let to = self.expression_parser().parse_expression_entrance()?.calculate();
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Step)?;
        let step = self.expression_parser().parse_expression_entrance()?.calculate();
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Draw)?;

        //点生成函数
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::LBracket)?;
        let x_expression = self.expression_parser().parse_expression_entrance()?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::Comma)?;
        let y_expression = self.expression_parser().parse_expression_entrance()?;
        self.get_mut_parser_kernel().match_and_eat_token(TokenTypeEnum::RBracket)?;

        //生成所有点
        let mut curr_t = from;
        while curr_t <= to{
            self.expression_parser().set_t(curr_t);
            //TODO 统计越界点，打印warning
            let _ = self.point_manager().add_point((x_expression.calculate(),y_expression.calculate()));
            curr_t+=step;
        }

        Ok(())
    }

    pub fn get_mut_parser_kernel(&self) -> RefMut<ParserKernel> {
        self.parser_kernel.borrow_mut()
    }

    fn point_manager(&mut self) -> &mut point_manager::PointManager {
        &mut self.point_manager
    }

    fn expression_parser(&mut self) -> &mut expression::ExpressionParser {
        &mut self.expression_parser
    }
}

///对parser底层进行一次封装
pub struct ParserKernel {
    curr_token: Token,
    lexer: Lexer,
}

impl ParserKernel {
    pub fn new(file: File) -> Self {
        let mut lexer = Lexer::new(file);
        Self {
            curr_token: lexer.fetch_token(), //刚开始读一个以保证逻辑一致性
            lexer,
        }
    }

    ///检查当前token是否匹配目标，如果匹配则成功并读取一次token，否则会返回语法错误SyntaxError
    pub fn match_and_eat_token(&mut self, expected_token_type: TokenTypeEnum) -> exception::Result<()> {
        if self.curr_token.token_type() == TokenTypeEnum::ErrToken {
            return Err(exception::IllegalTokenError::new(&self.curr_token.lexeme()));
        }
        if self.curr_token.token_type() != expected_token_type {
            return Err(exception::SyntaxError::new(&self.curr_token, &[expected_token_type]));
        }
        self.curr_token = self.lexer.fetch_token();
        Ok(())
    }

    pub fn get_curr_token(&self) -> &Token {
        &self.curr_token
    }

    pub fn get_curr_token_type(&self) -> TokenTypeEnum {
        self.curr_token.token_type()
    }

    ///帮助自动生成语法错误，附有期望的token type
    pub fn generate_syntax_error<T>(&self, expected_token_type: &[TokenTypeEnum]) -> exception::Result<T> {
        Err(exception::SyntaxError::new(&self.get_curr_token(), expected_token_type))
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_parse() {
        let file = File::open("parse_test.txt").unwrap();
        let mut parser = ParserManager::new(file);
        parser.parse();
    }
}




