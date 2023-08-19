use std::collections::HashMap;

mod ast_tree;

pub struct ExpressionManager{
    symbol_table: HashMap<String,f64>, //符号表，符号名->值
}

impl ExpressionManager{
    pub fn new()->Self{
        ExpressionManager{
            symbol_table: HashMap::new()
        }
    }

    // pub fn match_expression
}













