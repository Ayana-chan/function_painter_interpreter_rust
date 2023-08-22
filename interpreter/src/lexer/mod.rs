use std::collections::hash_map::HashMap;
use std::fs::File;

pub use token_manager::*;

mod text_reader;
mod token_manager;

pub struct Lexer {
    //字符读取器
    text_reader: text_reader::TextReader,
    //当前缓存（正在处理）的char
    curr_char: Option<char>,

    //符号表
    token_match_map: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(file: File) -> Self {
        let mut aim_text_reader = text_reader::TextReader::new(file);
        //先吃一个作为缓存，以保证逻辑一致性
        let curr_char = aim_text_reader.eat_char();
        Lexer {
            text_reader: aim_text_reader,
            curr_char,
            token_match_map: Token::generate_token_match_map(),
        }
    }

    ///获取下一个token
    pub fn fetch_token(&mut self) -> Token {
        // println!("Debug: fetch_token");
        //略过空白项
        self.skip_whitespace();

        if let Some(ch) = self.get_curr_char() {
            //根据开头字符，分为三种情况进行拼接
            if ch.is_ascii_digit() {
                //1.数字开头。必须是double。吃掉小数点、数字、字母，最后一定要符合double格式
                return self.collect_digit_token();
            } else if ch.is_ascii_alphabetic() {
                //2.字母开头。保留字、函数名、参数、常数。吃掉字母、数字，最后去Map进行匹配
                return self.collect_word_token();
            } else {
                //3.运算符、分隔符。只有单符号和双符号
                return self.collect_special_token();
            }
        }

        //None，表示EOF了
        return Token::generate_eof_token();
    }

    ///获取当前正在读取的字符的位置，(line,col)
    pub fn get_char_position(&self)->(u32,u32){
        self.text_reader.get_char_position()
    }

    ///略过空白项
    fn skip_whitespace(&mut self) {
        loop {
            if let Some(ch) = self.get_curr_char() {
                if ch.is_ascii_whitespace() {
                    self.read_new_char();
                    continue;
                }
            }
            break;
        }
    }

    ///数字开头。必须是数字字面值（视为double）。吃掉小数点、数字、字母，最后一定要符合double格式
    fn collect_digit_token(&mut self) -> Token {
        let mut lexeme_char_vec: Vec<char> = Vec::new();
        loop {
            if let Some(ch) = self.get_curr_char() {
                if *ch == '.' || ch.is_ascii_digit() || ch.is_ascii_alphabetic() {
                    lexeme_char_vec.push(ch.clone());
                    self.read_new_char();
                    continue;
                }
            }
            break;
        }

        let lexeme: String = lexeme_char_vec.into_iter().collect();

        match lexeme.parse::<f64>() {
            Ok(value) => TokenBuilder::new().token_type(TokenTypeEnum::ConstId).lexeme(&lexeme).value(value).build(),
            _ => Token::generate_err_token(&lexeme)
        }
    }

    ///字母开头。保留字、函数名、参数、变量、常数。吃掉字母、数字，最后去Map进行匹配
    fn collect_word_token(&mut self) -> Token {
        let mut lexeme_char_vec: Vec<char> = Vec::new();
        loop {
            if let Some(ch) = self.get_curr_char() {
                if ch.is_ascii_digit() || ch.is_ascii_alphabetic() {
                    lexeme_char_vec.push(ch.clone());
                    self.read_new_char();
                    continue;
                }
            }
            break;
        }
        let lexeme: String = lexeme_char_vec.into_iter().collect();

        match self.token_match_map.get(&lexeme) {
            Some(token) => {
                return (*token).clone();
            }
            None => {
                //可能是新的变量名
                for ch in lexeme.chars() {
                    //变量名不能包含字母数字以外的符号
                    if !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() {
                        return Token::generate_err_token(&lexeme);
                    }
                }
                return TokenBuilder::new().token_type(TokenTypeEnum::Variable).lexeme(&lexeme).build();
            }
        };
    }

    ///运算符、分隔符。只有单符号和双符号
    fn collect_special_token(&mut self) -> Token {
        let aim_char = self.get_curr_char().unwrap();
        self.read_new_char();

        //要先检测所有双符号
        //指数
        if aim_char == '*' {
            //要看下一个符号是什么
            if let Some(ch) = self.get_curr_char() {
                if *ch == '*' {
                    self.read_new_char();
                    return self.token_match_map.get("**").unwrap().clone();
                }
            }
        }

        //行注释
        if aim_char == '/' {
            //要看下一个符号是什么
            if let Some(ch) = self.get_curr_char() {
                if *ch == '/' {
                    self.read_new_char();
                    //读到行末或EOF
                    loop {
                        self.read_new_char();
                        if let Some(ch) = self.get_curr_char() {
                            if *ch == '\n' || *ch == '\r' { //换行
                                break;
                            }
                        } else {
                            break; //EOF
                        }
                    }
                    //查找下一个token以返回，保证上层一直接收到有效的token
                    return self.fetch_token();
                }
            }
        }

        return self.token_match_map.get(&String::from(aim_char)).unwrap().clone();
    }

    ///获取curr_char
    fn get_curr_char(&self) -> &Option<char> {
        &self.curr_char
    }

    ///读取新的char并覆盖当前curr_char
    fn read_new_char(&mut self) {
        self.curr_char = self.text_reader.eat_char();
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_lex() {
        let file = File::open("parse_test.txt").unwrap();
        let mut lexer = Lexer::new(file);

        // let token = lexer.fetch_token();
        loop {
            let token = lexer.fetch_token();
            println!("{:?}", token);
            if token.token_type() == TokenTypeEnum::NonToken {
                break;
            }
        }
    }
}






