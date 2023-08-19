use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::{exception, parser};

mod ast_tree;

pub struct ExpressionParser {
    parser_kernel: Weak<RefCell<parser::ParserKernel>>,
}

impl ExpressionParser {
    pub fn new(parser_kernel: &Rc<RefCell<parser::ParserKernel>>) -> Self {
        ExpressionParser {
            parser_kernel: Rc::downgrade(parser_kernel),
        }
    }

    //TODO 控制语法树的打印开关
    ///分析表达式。加减
    fn parse_expression(&mut self) -> exception::Result<()> {
        Ok(())
    }

    ///乘除
    fn parse_term(&mut self) -> exception::Result<()> {
        Ok(())
    }

    ///一元正负
    fn parse_factor(&mut self) -> exception::Result<()> {
        Ok(())
    }

    ///乘方
    fn parse_component(&mut self) -> exception::Result<()> {
        Ok(())
    }

    ///常量、参数、括号、函数
    fn parse_atom(&mut self) -> exception::Result<()> {
        Ok(())
    }
}













