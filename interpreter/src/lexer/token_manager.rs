use std::collections::hash_map::HashMap;

#[derive(Debug,Clone)]
pub struct Token {
    token_type: TokenTypeEnum,
    //Token类型
    lexeme: String,
    //原始串内容
    value: f64, //数值
}

#[derive(Debug,Clone,PartialEq)]
pub enum TokenTypeEnum {
    Id,
    //注释（在词法分析时直接被丢掉了）
    Comment,

    //保留字
    Origin,
    Scale,
    Rot,
    Is,
    To,
    Step,
    Draw,
    For,
    From,

    //参数
    T,

    //分隔符
    Semico,
    LBracket,
    RBracket,
    Comma,

    //运算符
    Plus,
    Minus,
    Mul,
    Div,
    Power,

    //函数名
    Func,
    //常数（数值字面量、命名常量）
    ConstId,

    //源程序结束（#/EOF）
    NonToken,
    //错误Token
    ErrToken,
}

impl Token {
    pub fn new(token_type: TokenTypeEnum, lexeme: &str, value: f64) -> Self {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            value,
        }
    }

    pub fn token_type(&self) -> &TokenTypeEnum {
        &self.token_type
    }
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_token_type(&mut self, token_type: TokenTypeEnum) {
        self.token_type = token_type;
    }
    pub fn set_lexeme(&mut self, lexeme: &str) {
        let lexeme= String::from(lexeme);
        self.lexeme = lexeme;
    }
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn generate_token_match_map() -> HashMap<String, Token> {
        let mut string_trans_token_map = HashMap::new();
        //保留字
        string_trans_token_map.insert(String::from("ORIGIN"), Token::new(TokenTypeEnum::Origin, "ORIGIN", 0.0));
        string_trans_token_map.insert(String::from("SCALE"), Token::new(TokenTypeEnum::Scale, "SCALE", 0.0));
        string_trans_token_map.insert(String::from("ROT"), Token::new(TokenTypeEnum::Rot, "ROT", 0.0));
        string_trans_token_map.insert(String::from("IS"), Token::new(TokenTypeEnum::Is, "IS", 0.0));
        string_trans_token_map.insert(String::from("TO"), Token::new(TokenTypeEnum::To, "TO", 0.0));
        string_trans_token_map.insert(String::from("STEP"), Token::new(TokenTypeEnum::Step, "STEP", 0.0));
        string_trans_token_map.insert(String::from("DRAW"), Token::new(TokenTypeEnum::Draw, "DRAW", 0.0));
        string_trans_token_map.insert(String::from("FOR"), Token::new(TokenTypeEnum::For, "FOR", 0.0));
        string_trans_token_map.insert(String::from("FROM"), Token::new(TokenTypeEnum::From, "FROM", 0.0));
        //分隔符
        string_trans_token_map.insert(String::from(";"), Token::new(TokenTypeEnum::Semico, ";", 0.0));
        string_trans_token_map.insert(String::from("("), Token::new(TokenTypeEnum::LBracket, "(", 0.0));
        string_trans_token_map.insert(String::from(")"), Token::new(TokenTypeEnum::RBracket, ")", 0.0));
        string_trans_token_map.insert(String::from(","), Token::new(TokenTypeEnum::Comma, ",", 0.0));
        //运算符
        string_trans_token_map.insert(String::from("+"), Token::new(TokenTypeEnum::Plus, "+", 0.0));
        string_trans_token_map.insert(String::from("-"), Token::new(TokenTypeEnum::Minus, "-", 0.0));//"--"前缀
        string_trans_token_map.insert(String::from("*"), Token::new(TokenTypeEnum::Mul, "*", 0.0));//"**"前缀
        string_trans_token_map.insert(String::from("/"), Token::new(TokenTypeEnum::Div, "/", 0.0));//"//"前缀
        string_trans_token_map.insert(String::from("**"), Token::new(TokenTypeEnum::Power, "**", 0.0));//
        //函数名
        string_trans_token_map.insert(String::from("SIN"), Token::new(TokenTypeEnum::Func, "SIN", 0.0));
        string_trans_token_map.insert(String::from("COS"), Token::new(TokenTypeEnum::Func, "COS", 0.0));
        string_trans_token_map.insert(String::from("TAN"), Token::new(TokenTypeEnum::Func, "TAN", 0.0));//
        string_trans_token_map.insert(String::from("LN"), Token::new(TokenTypeEnum::Func, "LN", 0.0));
        string_trans_token_map.insert(String::from("EXP"), Token::new(TokenTypeEnum::Func, "EXP", 0.0));//
        string_trans_token_map.insert(String::from("SQRT"), Token::new(TokenTypeEnum::Func, "SQRT", 0.0));
        //参数
        string_trans_token_map.insert(String::from("T"), Token::new(TokenTypeEnum::T, "T", 0.0));//"TAN"前缀
        //常数
        string_trans_token_map.insert(String::from("PI"), Token::new(TokenTypeEnum::ConstId, "PI", std::f64::consts::PI));
        string_trans_token_map.insert(String::from("E"), Token::new(TokenTypeEnum::ConstId, "E", std::f64::consts::E));//"EXP"前缀
        //注释
        string_trans_token_map.insert(String::from("//"), Token::new(TokenTypeEnum::Comment, "//", 0.0));//
        string_trans_token_map.insert(String::from("--"), Token::new(TokenTypeEnum::Comment, "--", 0.0));//

        string_trans_token_map
    }

    ///获取EOF token
    pub fn generate_eof_token() -> Token {
        Token::new(TokenTypeEnum::NonToken, "EOF(#)", 0.0)
    }
}





