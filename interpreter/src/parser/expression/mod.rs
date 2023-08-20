use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::{exception, parser, lexer};

pub use ast_tree::ASTNode;

mod ast_tree;

pub struct ExpressionParser {
    parser_kernel: Rc<RefCell<parser::ParserKernel>>,
}

impl ExpressionParser {
    pub fn new(parser_kernel: &Rc<RefCell<parser::ParserKernel>>) -> Self {
        ExpressionParser {
            parser_kernel: parser_kernel.clone(),
        }
    }

    //TODO 控制语法树的打印开关
    ///分析表达式，返回语法树。加减，左结合
    pub fn parse_expression(&mut self) -> exception::Result<Box<dyn ASTNode>> {
        let mut left_node_ref = self.parse_term()?;
        let token_type = self.get_mut_parser_kernel().get_curr_token_type();

        //迭代
        while token_type == lexer::TokenTypeEnum::Plus || token_type == lexer::TokenTypeEnum::Minus {
            let expression_token = self.get_mut_parser_kernel().get_curr_token().clone();
            self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
            let right_node_ref = self.parse_term()?;
            let ans_node = ast_tree::BinaryNode::new(
                &expression_token, Rc::from(left_node_ref), Rc::from(right_node_ref),
            );
            left_node_ref = Box::new(ans_node);
        }

        Ok(left_node_ref)
    }

    ///乘除，左结合
    fn parse_term(&mut self) -> exception::Result<Box<dyn ASTNode>> {
        let mut left_node_ref = self.parse_factor()?;
        let token_type = self.get_mut_parser_kernel().get_curr_token_type();

        //迭代
        while token_type == lexer::TokenTypeEnum::Mul || token_type == lexer::TokenTypeEnum::Div {
            let term_token = self.get_mut_parser_kernel().get_curr_token().clone();
            self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
            let right_node_ref = self.parse_factor()?;
            let ans_node = ast_tree::BinaryNode::new(
                &term_token, Rc::from(left_node_ref), Rc::from(right_node_ref),
            );
            left_node_ref = Box::new(ans_node);
        }

        Ok(left_node_ref)
    }

    ///一元正负
    fn parse_factor(&mut self) -> exception::Result<Box<dyn ASTNode>> {
        let token_type = self.get_mut_parser_kernel().get_curr_token_type();
        let token = self.get_mut_parser_kernel().get_curr_token().clone();
        self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
        let num_node_ref = self.parse_component()?;

        //-num看成0-num
        if token_type == parser::TokenTypeEnum::Minus {
            let left_zero_node = Rc::new(ast_tree::ConstNode::new(0.0));
            let ans_node = ast_tree::BinaryNode::new(&token, left_zero_node, Rc::from(num_node_ref));
            return Ok(Box::new(ans_node));
        }

        //+num则直接视为num
        Ok(num_node_ref)
    }

    ///乘方，右结合
    fn parse_component(&mut self) -> exception::Result<Box<dyn ASTNode>> {
        let left_node_ref = self.parse_atom()?;
        let token_type = self.get_mut_parser_kernel().get_curr_token_type();
        if token_type == lexer::TokenTypeEnum::Power {
            let power_token = self.get_mut_parser_kernel().get_curr_token().clone();
            self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
            let right_node_ref = self.parse_component()?;
            let ans_node = ast_tree::BinaryNode::new(&power_token, Rc::from(left_node_ref), Rc::from(right_node_ref));
            return Ok(Box::new(ans_node));
        }

        //如果没有乘方，仅仅是Atom的话也要放行
        Ok(left_node_ref)
    }

    ///常量、参数、括号（子表达式）、函数
    fn parse_atom(&mut self) -> exception::Result<Box<dyn ASTNode>> {
        let token_type = self.get_mut_parser_kernel().get_curr_token_type();
        return match token_type {
            //常量
            lexer::TokenTypeEnum::ConstId => {
                let ans_node = ast_tree::ConstNode::new(
                    self.get_mut_parser_kernel().get_curr_token().value()
                );
                self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
                Ok(Box::new(ans_node))
            }
            //参数
            lexer::TokenTypeEnum::Variable => {
                let mut pk = self.get_mut_parser_kernel(); //让生命周期略延长
                let var_name = pk.get_curr_token().lexeme().clone();
                let var_value = pk.variable_symbol_table().get(&var_name);
                if let Some(variable_reference) = var_value {
                    let ans_node = ast_tree::VariableNode::new(
                        self.get_mut_parser_kernel().get_curr_token(), variable_reference,
                    );
                    self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
                    Ok(Box::new(ans_node))
                } else {
                    Err(exception::UndefinedVariableError::new(&var_name))
                }
            }
            //括号（子表达式）
            lexer::TokenTypeEnum::LBracket => {
                self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
                let ans_node_ref = self.parse_expression()?;
                self.get_mut_parser_kernel().match_and_eat_token(lexer::TokenTypeEnum::RBracket)?;
                Ok(ans_node_ref)
            }
            //函数
            lexer::TokenTypeEnum::Func => {
                let func_token = self.get_mut_parser_kernel().get_curr_token().clone();
                self.get_mut_parser_kernel().match_and_eat_token(token_type)?;
                self.get_mut_parser_kernel().match_and_eat_token(lexer::TokenTypeEnum::LBracket)?;
                let mut arg_nodes: Vec<Rc<dyn ASTNode>> = Vec::new();
                let mut first_arg_flag = true;//用于忽略匹配第一个参数前的逗号
                loop {
                    let token_type = self.get_mut_parser_kernel().get_curr_token_type();
                    match token_type {
                        lexer::TokenTypeEnum::RBracket => {
                            self.get_mut_parser_kernel().match_and_eat_token(lexer::TokenTypeEnum::RBracket)?;
                            break;
                        }
                        _ => {
                            if !first_arg_flag {
                                self.get_mut_parser_kernel().match_and_eat_token(lexer::TokenTypeEnum::Comma)?;
                            }
                            first_arg_flag = false;
                            //获取参数表达式
                            let new_arg_node = self.parse_expression()?;
                            arg_nodes.push(Rc::from(new_arg_node));
                        }
                    }
                }

                let ans_node = ast_tree::FuncNode::new(self.get_mut_parser_kernel().get_curr_token(), arg_nodes);
                Ok(Box::new(ans_node))
            }
            _ => {
                let ans = self.get_mut_parser_kernel().generate_syntax_error(&[
                    lexer::TokenTypeEnum::ConstId, lexer::TokenTypeEnum::Variable, lexer::TokenTypeEnum::LBracket, lexer::TokenTypeEnum::Func
                ]);
                ans
            }
        };
    }

    pub fn get_mut_parser_kernel(&self) -> RefMut<parser::ParserKernel> {
        self.parser_kernel.borrow_mut()
    }
}













