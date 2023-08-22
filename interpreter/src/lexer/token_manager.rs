use std::collections::hash_map::HashMap;
use std::rc::Rc;
use crate::exception;

#[derive(Clone)]
pub struct Token {
    //Token类型
    token_type: TokenTypeEnum,
    //原始串内容
    lexeme: String,
    //数值
    value: f64,
    //函数
    func: Rc<dyn Fn(&[f64]) -> exception::Result<f64>>,
}

///用于建造Token（建造者模式）
pub struct TokenBuilder {
    //Token类型
    token_type: Option<TokenTypeEnum>,
    //原始串内容
    lexeme: Option<String>,
    //数值
    value: Option<f64>,
    //函数
    func: Option<Rc<dyn Fn(&[f64]) -> exception::Result<f64>>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenTypeEnum {
    //注释（在词法分析时直接被丢掉了，可以说完全没用）
    Comment,

    //保留字
    Origin,
    Scale,
    Rot,
    Is,
    For,
    From,
    To,
    Step,
    Draw,
    Def,
    Let,

    //for语句固定参数
    T,
    //变量
    Variable,

    //分隔符
    Semico,
    LBracket,
    RBracket,
    Comma,

    //赋值
    Assign,

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

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token {{")?;
        write!(f, " token_type: {:?},", self.token_type)?;
        write!(f, " lexeme: {:?}", self.lexeme)?;
        write!(f, " }}")
    }
}

impl Token {
    pub fn token_type(&self) -> TokenTypeEnum {
        self.token_type
    }
    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn func(&self) -> &Rc<dyn Fn(&[f64]) -> exception::Result<f64>> {
        &self.func
    }

    // pub fn set_token_type(&mut self, token_type: TokenTypeEnum) {
    //     self.token_type = token_type;
    // }
    // pub fn set_lexeme(&mut self, lexeme: &str) {
    //     let lexeme = String::from(lexeme);
    //     self.lexeme = lexeme;
    // }
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    pub fn set_func(&mut self, func: Rc<dyn Fn(&[f64]) -> exception::Result<f64>>) {
        self.func = func;
    }

    //工具函数，判断参数数量是否等同于目标
    pub fn judge_arg_num_equal(func_name: &str, args: &[f64], target_num: usize) -> exception::Result<f64> {
        if args.len() != target_num {
            return Err(exception::ArgumentNumberNotMatchError::new(func_name, args.len(), target_num));
        }
        Ok(0.0)
    }

    pub fn generate_token_match_map() -> HashMap<String, Token> {
        let mut string_trans_token_map = HashMap::new();

        //保留字
        string_trans_token_map.insert(String::from("ORIGIN"), TokenBuilder::new().token_type(TokenTypeEnum::Origin).lexeme("ORIGIN").build());
        string_trans_token_map.insert(String::from("SCALE"), TokenBuilder::new().token_type(TokenTypeEnum::Scale).lexeme("SCALE").build());
        string_trans_token_map.insert(String::from("ROT"), TokenBuilder::new().token_type(TokenTypeEnum::Rot).lexeme("ROT").build());
        string_trans_token_map.insert(String::from("IS"), TokenBuilder::new().token_type(TokenTypeEnum::Is).lexeme("IS").build());
        string_trans_token_map.insert(String::from("FOR"), TokenBuilder::new().token_type(TokenTypeEnum::For).lexeme("FOR").build());
        string_trans_token_map.insert(String::from("FROM"), TokenBuilder::new().token_type(TokenTypeEnum::From).lexeme("FROM").build());
        string_trans_token_map.insert(String::from("TO"), TokenBuilder::new().token_type(TokenTypeEnum::To).lexeme("TO").build());
        string_trans_token_map.insert(String::from("STEP"), TokenBuilder::new().token_type(TokenTypeEnum::Step).lexeme("STEP").build());
        string_trans_token_map.insert(String::from("DRAW"), TokenBuilder::new().token_type(TokenTypeEnum::Draw).lexeme("DRAW").build());
        string_trans_token_map.insert(String::from("DEF"), TokenBuilder::new().token_type(TokenTypeEnum::Def).lexeme("DEF").build());
        string_trans_token_map.insert(String::from("LET"), TokenBuilder::new().token_type(TokenTypeEnum::Let).lexeme("LET").build());

        //分隔符
        string_trans_token_map.insert(String::from(";"), TokenBuilder::new().token_type(TokenTypeEnum::Semico).lexeme(";").build());
        string_trans_token_map.insert(String::from("("), TokenBuilder::new().token_type(TokenTypeEnum::LBracket).lexeme("(").build());
        string_trans_token_map.insert(String::from(")"), TokenBuilder::new().token_type(TokenTypeEnum::RBracket).lexeme(")").build());
        string_trans_token_map.insert(String::from(","), TokenBuilder::new().token_type(TokenTypeEnum::Comma).lexeme(",").build());

        //赋值
        string_trans_token_map.insert(String::from("="), TokenBuilder::new().token_type(TokenTypeEnum::Assign).lexeme("=").build());

        //运算符
        string_trans_token_map.insert(String::from("+"), TokenBuilder::new().token_type(TokenTypeEnum::Plus).lexeme("+")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("+",args, 2)?;
                Ok(args[0] + args[1])
            })).build());
        string_trans_token_map.insert(String::from("-"), TokenBuilder::new().token_type(TokenTypeEnum::Minus).lexeme("-")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("-",args, 2)?;
                Ok(args[0] - args[1])
            })).build());
        string_trans_token_map.insert(String::from("*"), TokenBuilder::new().token_type(TokenTypeEnum::Mul).lexeme("*")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("*",args, 2)?;
                Ok(args[0] * args[1])
            })).build());//"**"前缀
        string_trans_token_map.insert(String::from("/"), TokenBuilder::new().token_type(TokenTypeEnum::Div).lexeme("/")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("/",args, 2)?;
                Ok(args[0] / args[1])
            })).build());//"//"前缀
        string_trans_token_map.insert(String::from("**"), TokenBuilder::new().token_type(TokenTypeEnum::Power).lexeme("**")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("**",args, 2)?;
                Ok(args[0].powf(args[1]))
            })).build());

        //函数名
        string_trans_token_map.insert(String::from("SIN"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("SIN")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("SIN",args, 1)?;
                Ok(args[0].sin())
            })).build());
        string_trans_token_map.insert(String::from("COS"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("COS")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("COS",args, 1)?;
                Ok(args[0].cos())
            })).build());
        string_trans_token_map.insert(String::from("TAN"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("TAN")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("TAN",args, 1)?;
                Ok(args[0].tan())
            })).build());
        string_trans_token_map.insert(String::from("LN"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("LN")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("LN",args, 1)?;
                Ok(args[0].ln())
            })).build());
        string_trans_token_map.insert(String::from("EXP"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("EXP")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("EXP",args, 1)?;
                Ok(args[0].exp())
            })).build());
        string_trans_token_map.insert(String::from("SQRT"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("SQRT")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("SQRT",args, 1)?;
                Ok(args[0].sqrt())
            })).build());
        string_trans_token_map.insert(String::from("ABS"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("ABS")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("ABS",args, 1)?;
                Ok(args[0].abs())
            })).build());
        string_trans_token_map.insert(String::from("MAX"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("MAX")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("MAX",args, 2)?;
                Ok(args[0].max(args[1]))
            })).build());
        string_trans_token_map.insert(String::from("MIN"), TokenBuilder::new().token_type(TokenTypeEnum::Func).lexeme("MIN")
            .func(Rc::new(|args| {
                Token::judge_arg_num_equal("MIN",args, 2)?;
                Ok(args[0].min(args[1]))
            })).build());

        //参数
        string_trans_token_map.insert(String::from("T"), TokenBuilder::new().token_type(TokenTypeEnum::T).lexeme("T").build());

        //常数
        string_trans_token_map.insert(String::from("PI"), TokenBuilder::new().token_type(TokenTypeEnum::ConstId).lexeme("PI")
            .value(std::f64::consts::PI).build());
        string_trans_token_map.insert(String::from("E"), TokenBuilder::new().token_type(TokenTypeEnum::ConstId).lexeme("E")
            .value(std::f64::consts::E).build());//"EXP"前缀

        //注释
        string_trans_token_map.insert(String::from("//"), TokenBuilder::new().token_type(TokenTypeEnum::Comment).lexeme("//").build());//
        string_trans_token_map.insert(String::from("--"), TokenBuilder::new().token_type(TokenTypeEnum::Comment).lexeme("--").build());//

        string_trans_token_map
    }

    ///生成EOF token
    pub fn generate_eof_token() -> Token {
        TokenBuilder::new().token_type(TokenTypeEnum::NonToken).lexeme("EOF(#)").build()
    }

    ///生成Err token
    pub fn generate_err_token(lexeme: &str) -> Token {
        TokenBuilder::new().token_type(TokenTypeEnum::ErrToken).lexeme(lexeme).build()
    }
}

impl TokenBuilder {
    pub fn new() -> Self {
        TokenBuilder {
            token_type: None,
            lexeme: None,
            value: None,
            func: None,
        }
    }

    pub fn token_type(mut self, token_type: TokenTypeEnum) -> Self {
        self.token_type = Some(token_type);
        self
    }

    pub fn lexeme(mut self, lexeme: &str) -> Self {
        self.lexeme = Some(lexeme.parse().unwrap());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn func(mut self, func: Rc<dyn Fn(&[f64]) -> exception::Result<f64>>) -> Self {
        self.func = Some(func);
        self
    }

    pub fn build(self) -> Token {
        let mut res = Token {
            token_type: self.token_type.unwrap(),
            lexeme: self.lexeme.unwrap(),
            value: 0.0,
            func: Rc::new(|_args| { Ok(0.0) }),
        };

        if let Some(value) = self.value {
            res.set_value(value);
        }

        if let Some(func) = self.func {
            res.set_func(func);
        }

        res
    }
}



