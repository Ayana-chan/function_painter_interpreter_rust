use std::cell::RefCell;
use std::rc::Rc;
use crate::lexer;
use crate::exception;

pub trait ASTNode {
    fn calculate(&self) -> exception::Result<f64>;
    fn print_tree(&self, level: i32);
}

//用于辅助print语法树的三个函数
fn print_tree_prefix_tab(level: i32) {
    print!("  |");
    for _ in 0..level {
        print!("       |");
    }
}

fn print_tree_prefix_begin(level: i32) {
    if level == 0 {
        print!("->/");
    } else {
        print_tree_prefix_tab(level - 1);
        print!("----->/");
    }
}

fn print_tree_prefix_end(level: i32) {
    if level == 0 {
        print!("  `");
    } else {
        print_tree_prefix_tab(level - 1);
        print!("       `");
    }
    println!();
}

///二元运算
pub struct BinaryNode {
    token_type: lexer::TokenTypeEnum,
    func: Rc<dyn Fn(&[f64]) -> exception::Result<f64>>,
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl BinaryNode {
    pub fn new(token: &lexer::Token, left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        BinaryNode {
            token_type: token.token_type(),
            func: token.func().clone(),
            left,
            right,
        }
    }
}

impl ASTNode for BinaryNode {
    fn calculate(&self) -> exception::Result<f64> {
        let left_result = self.left.calculate()?;
        let right_result = self.right.calculate()?;
        (self.func)(&[left_result, right_result])
    }

    fn print_tree(&self, level: i32) {
        print_tree_prefix_begin(level);
        println!("$ {:?}", self.token_type);

        print_tree_prefix_tab(level);
        println!();
        self.left.print_tree(level + 1);
        // print_tree_prefix_tab(level);
        // println!();
        self.right.print_tree(level + 1);

        print_tree_prefix_end(level);
    }
}

///常数
pub struct ConstNode {
    value: f64,
}

impl ConstNode {
    pub fn new(value: f64) -> Self {
        ConstNode {
            value,
        }
    }
}

impl ASTNode for ConstNode {
    fn calculate(&self) -> exception::Result<f64> {
        Ok(self.value)
    }

    fn print_tree(&self, level: i32) {
        print_tree_prefix_begin(level);
        println!("$ {:?}", self.value);

        print_tree_prefix_end(level);
    }
}

///函数
pub struct FuncNode {
    token_type: lexer::TokenTypeEnum,
    func_name: String,
    func: Rc<dyn Fn(&[f64]) -> exception::Result<f64>>,
    arg_nodes: Vec<Box<dyn ASTNode>>,
}

impl FuncNode {
    pub fn new(token: &lexer::Token, arg_nodes: Vec<Box<dyn ASTNode>>) -> Self {
        FuncNode {
            token_type: token.token_type(),
            func_name: token.lexeme().parse().unwrap(),
            func: token.func().clone(),
            arg_nodes,
        }
    }
}

impl ASTNode for FuncNode {
    fn calculate(&self) -> exception::Result<f64> {
        let mut args: Vec<f64> = Vec::new();
        for node in &self.arg_nodes {
            args.push(node.calculate()?);
        }
        (self.func)(&args)
    }

    fn print_tree(&self, level: i32) {
        print_tree_prefix_begin(level);
        println!("$ {:?}", self.token_type);
        print_tree_prefix_tab(level);
        println!(": {}", self.func_name);

        print_tree_prefix_tab(level);
        println!();
        for arg_node in &self.arg_nodes {
            arg_node.print_tree(level + 1)
        }

        print_tree_prefix_end(level);
    }
}

///参数T
pub struct TNode {
    value_reference: Rc<RefCell<f64>>,
}

impl TNode {
    pub fn new(value_reference: &Rc<RefCell<f64>>) -> Self {
        TNode {
            value_reference: value_reference.clone(),
        }
    }
}

impl ASTNode for TNode {
    fn calculate(&self) -> exception::Result<f64> {
        Ok(*(*self.value_reference).borrow())
    }

    fn print_tree(&self, level: i32) {
        print_tree_prefix_begin(level);
        println!("$ {:?}", lexer::TokenTypeEnum::T);

        print_tree_prefix_end(level);
    }
}

///变量
pub struct VariableNode {
    variable_name: String,
    expression_reference: Rc<RefCell<Box<dyn ASTNode>>>,
}

impl VariableNode {
    pub fn new(variable_name: &str, expression_reference: &Rc<RefCell<Box<dyn ASTNode>>>) -> Self {
        VariableNode {
            variable_name: String::from(variable_name),
            expression_reference: expression_reference.clone(),
        }
    }
}

impl ASTNode for VariableNode {
    fn calculate(&self) -> exception::Result<f64> {
        (*self.expression_reference).borrow().calculate()
    }

    fn print_tree(&self, level: i32) {
        print_tree_prefix_begin(level);
        println!("$ {:?}", lexer::TokenTypeEnum::Variable);
        print_tree_prefix_tab(level);
        println!(": {}", self.variable_name);

        //打印其所属语法树
        self.expression_reference.borrow_mut().print_tree(level + 1);

        print_tree_prefix_end(level);
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{TokenBuilder, TokenTypeEnum};

    use super::*;

    #[test]
    fn test_binary_node() {
        let token1 = TokenBuilder::new().token_type(TokenTypeEnum::Plus)
            .lexeme("+").func(Rc::new(|args| {
            let ans = args[0] + args[1];
            println!("binary_node {} + {} ans = {}", args[0], args[1], ans);
            Ok(ans)
        })).build();

        let const_node1 = ConstNode::new(12.5);
        let const_node2 = ConstNode::new(5.3);
        let binary_node = BinaryNode::new(&token1, Box::new(const_node1), Box::new(const_node2));

        let ans = binary_node.calculate().unwrap();
        assert_eq!(ans, 17.8);
        binary_node.print_tree(0);
    }

    #[test]
    fn test_func_node() {
        let token1 = TokenBuilder::new().token_type(TokenTypeEnum::Func)
            .lexeme("mutiall").func(Rc::new(|args| {
            let mut ans = 1.0;
            for arg in args {
                ans *= arg;
                // println!("mutiall mid ans = {}", ans);
            }
            Ok(ans)
        })).build();
        let token2 = TokenBuilder::new().token_type(TokenTypeEnum::Plus)
            .lexeme("+").func(Rc::new(|args| {
            let ans = args[0] + args[1];
            println!("binary_node {} + {} ans = {}", args[0], args[1], ans);
            Ok(ans)
        })).build();

        let const_node1 = ConstNode::new(4.2);
        let const_node2 = ConstNode::new(0.8);
        let const_node3 = ConstNode::new(5.0);
        let const_node4 = ConstNode::new(5.0);
        let binary_node = BinaryNode::new(&token2, Box::new(const_node1), Box::new(const_node2));

        let mut args: Vec<Box<dyn ASTNode>> = Vec::new();
        args.push(Box::new(binary_node));
        args.push(Box::new(const_node3));
        args.push(Box::new(const_node4));
        let func_node = FuncNode::new(&token1, args);

        let mut ans = func_node.calculate().unwrap();
        println!("func_node (4.2+0.8) * 5.0 * 5.0 ans = {}", ans);
        assert_eq!(ans, 125.0);
        ans = func_node.calculate().unwrap();
        assert_eq!(ans, 125.0);

        func_node.print_tree(0);
    }

    #[test]
    fn test_variable_node() {
        let token1 = TokenBuilder::new().token_type(TokenTypeEnum::Plus)
            .lexeme("+").func(Rc::new(|args| {
            let ans = args[0] + args[1];
            println!("binary_node {} + {} ans = {}", args[0], args[1], ans);
            Ok(ans)
        })).build();
        let token2 = TokenBuilder::new().token_type(TokenTypeEnum::Variable)
            .lexeme("val").build();

        let val = ConstNode::new(8.5);
        let val_refer: Rc<RefCell<Box<dyn ASTNode>>> = Rc::new(RefCell::new(Box::new(val)));
        let val_node = VariableNode::new(token2.lexeme(), &val_refer);
        let const_node = ConstNode::new(5.0);
        let binary_node = BinaryNode::new(&token1, Box::new(val_node), Box::new(const_node));

        let ans = binary_node.calculate().unwrap();
        assert_eq!(ans, 13.5);

        binary_node.print_tree(0);
    }
}

