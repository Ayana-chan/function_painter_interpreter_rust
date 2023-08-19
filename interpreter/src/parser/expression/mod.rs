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

    fn parse_expression(&mut self) -> exception::Result<()> {
        Ok(())
    }
}













