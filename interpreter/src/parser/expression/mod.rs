use std::cell::RefCell;
use std::rc::Rc;
use crate::lexer;
//
//TODO T类型的值使用引用

pub trait ASTNode {
    fn calculate(&self) -> f64;
    //TODO print方法
}

///二元运算
pub struct BinaryNode {
    token_type: lexer::TokenTypeEnum,
    func: Rc<dyn Fn(&[f64]) -> f64>,
    left: Rc<dyn ASTNode>,
    right: Rc<dyn ASTNode>,
}

impl BinaryNode {
    pub fn new(token: lexer::Token, left: Rc<dyn ASTNode>, right: Rc<dyn ASTNode>) -> Self {
        BinaryNode {
            token_type: token.token_type(),
            func: token.func().clone(),
            left,
            right,
        }
    }
}

impl ASTNode for BinaryNode {
    fn calculate(&self) -> f64 {
        let left_result = self.left.calculate();
        let right_result = self.right.calculate();
        (self.func)(&[left_result, right_result])
    }
}

///常数
pub struct ConstNode {
    token_type: lexer::TokenTypeEnum,
    value: f64,
}

impl ConstNode {
    pub fn new(token: lexer::Token) -> Self {
        ConstNode {
            token_type: token.token_type(),
            value: token.value(),
        }
    }
}

impl ASTNode for ConstNode {
    fn calculate(&self) -> f64 {
        self.value
    }
}

///函数
pub struct FuncNode {
    token_type: lexer::TokenTypeEnum,
    func: Rc<dyn Fn(&[f64]) -> f64>,
    arg_nodes: Vec<Rc<dyn ASTNode>>,
}

impl FuncNode {
    pub fn new(token: lexer::Token, arg_nodes: Vec<Rc<dyn ASTNode>>) -> Self {
        FuncNode {
            token_type: token.token_type(),
            func: token.func().clone(),
            arg_nodes,
        }
    }
}

impl ASTNode for FuncNode {
    fn calculate(&self) -> f64 {
        let mut args: Vec<f64> = Vec::new();
        for node in &self.arg_nodes {
            args.push(node.calculate());
        }
        (self.func)(&args)
    }
}

///变量
pub struct VariableNode {
    token_type: lexer::TokenTypeEnum,
    value_reference: Rc<RefCell<f64>>,
}

impl VariableNode {
    pub fn new(token: lexer::Token, value_reference: &Rc<RefCell<f64>>) -> Self {
        VariableNode {
            token_type: token.token_type(),
            value_reference: value_reference.clone(),
        }
    }
}

impl ASTNode for VariableNode {
    fn calculate(&self) -> f64 {
        *self.value_reference.borrow()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::lexer::{Token, TokenBuilder, TokenTypeEnum};

    use super::*;

    #[test]
    fn test_binary_node() {
        let token1 = TokenBuilder::new().token_type(TokenTypeEnum::Plus)
            .lexeme("+").func(Rc::new(|args| {
            let ans = args[0] + args[1];
            println!("binary_node {} + {} ans = {}", args[0], args[1], ans);
            ans
        })).build();
        let token2 = TokenBuilder::new().token_type(TokenTypeEnum::ConstId)
            .lexeme("12.5").value(12.5).build();
        let token3 = TokenBuilder::new().token_type(TokenTypeEnum::ConstId)
            .lexeme("5.3").value(5.3).build();

        let const_node1 = ConstNode::new(token2);
        let const_node2 = ConstNode::new(token3);
        let binary_node = BinaryNode::new(token1, Rc::new(const_node1), Rc::new(const_node2));

        let ans = binary_node.calculate();
        assert_eq!(ans, 17.8);
    }

    #[test]
    fn test_func_node() {
        let token1 = TokenBuilder::new().token_type(TokenTypeEnum::Func)
            .lexeme("mutiall").func(Rc::new(|args| {
            let mut ans = 1.0;
            for arg in args {
                ans *= arg;
                println!("mutiall mid ans = {}", ans);
            }
            ans
        })).build();
        let token2 = TokenBuilder::new().token_type(TokenTypeEnum::Plus)
            .lexeme("+").func(Rc::new(|args| {
            let ans = args[0] + args[1];
            println!("binary_node {} + {} ans = {}", args[0], args[1], ans);
            ans
        })).build();
        let token3 = TokenBuilder::new().token_type(TokenTypeEnum::ConstId)
            .lexeme("4.2").value(4.2).build();
        let token4 = TokenBuilder::new().token_type(TokenTypeEnum::ConstId)
            .lexeme("0.8").value(0.8).build();
        let token5 = TokenBuilder::new().token_type(TokenTypeEnum::ConstId)
            .lexeme("5.0").value(5.0).build();
        let token6 = TokenBuilder::new().token_type(TokenTypeEnum::ConstId)
            .lexeme("5.0").value(5.0).build();

        let const_node1 = ConstNode::new(token3);
        let const_node2 = ConstNode::new(token4);
        let const_node3 = ConstNode::new(token5);
        let const_node4 = ConstNode::new(token6);
        let binary_node = BinaryNode::new(token2, Rc::new(const_node1), Rc::new(const_node2));

        let mut args: Vec<Rc<dyn ASTNode>> = Vec::new();
        args.push(Rc::new(binary_node));
        args.push(Rc::new(const_node3));
        args.push(Rc::new(const_node4));
        let func_node = FuncNode::new(token1, args);

        let mut ans = func_node.calculate();
        println!("func_node (4.2+0.8) * 5.0 * 5.0 ans = {}", ans);
        assert_eq!(ans, 125.0);
        ans = func_node.calculate();
        assert_eq!(ans, 125.0);
    }

    #[test]
    fn test_variable_node() {
        let token1 = TokenBuilder::new().token_type(TokenTypeEnum::Plus)
            .lexeme("+").func(Rc::new(|args| {
            let ans = args[0] + args[1];
            println!("binary_node {} + {} ans = {}", args[0], args[1], ans);
            ans
        })).build();
        let token2 = TokenBuilder::new().token_type(TokenTypeEnum::Variable)
            .lexeme("val").build();
        let token3 = TokenBuilder::new().token_type(TokenTypeEnum::ConstId)
            .lexeme("5.0").value(5.0).build();

        let val: f64 = 8.5;
        let val_refer: Rc<RefCell<f64>> = Rc::new(RefCell::new(val));
        let val_node = VariableNode::new(token2, &val_refer);
        let const_node = ConstNode::new(token3);
        let binary_node = BinaryNode::new(token1, Rc::new(val_node), Rc::new(const_node));

        let mut ans = binary_node.calculate();
        assert_eq!(ans, 13.5);

        //修改值
        *val_refer.borrow_mut()+=1.0;

        ans = binary_node.calculate();
        assert_eq!(ans, 14.5);
    }
}

