use std::collections::hash_map::HashMap;
use std::fs::File;

use token_manager::*;

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
            Ok(value) => Token::new(TokenTypeEnum::ConstId, &lexeme, value),
            _ => Token::new(TokenTypeEnum::ErrToken, &lexeme, 0.0)
        }
    }

    ///字母开头。保留字、函数名、参数、常数。吃掉字母、数字，最后去Map进行匹配
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

        self.match_token(&lexeme)
    }

    ///运算符、分隔符。只有单符号和双符号
    fn collect_special_token(&mut self) -> Token {
        let aim_char = self.get_curr_char().unwrap();
        self.read_new_char();

        //要先检测所有双符号
        //指数
        if aim_char == '*' {
            //要看下一个符号是什么
            if let Some(ch)=self.get_curr_char(){
                if *ch == '*'{
                    return self.match_token("**");
                }
            }
        }

        //行注释
        if aim_char == '/'{
            //要看下一个符号是什么
            if let Some(ch)=self.get_curr_char(){
                if *ch == '/'{
                    //读到行末或EOF
                    loop {
                        self.read_new_char();
                        if let Some(ch) = self.get_curr_char() {
                            if *ch == '\n' || *ch == '\r' { //换行
                                break
                            }
                        }else{
                            break; //EOF
                        }
                    }
                    //查找下一个token以返回，保证上层一直接收到有效的token
                    return self.fetch_token();
                }
            }
        }

        return self.match_token(&String::from(aim_char));
    }

    ///获取curr_char
    fn get_curr_char(&self) -> &Option<char> {
        &self.curr_char
    }

    ///读取新的char并覆盖当前curr_char
    fn read_new_char(&mut self) {
        self.curr_char = self.text_reader.eat_char();
    }

    ///在符号表中匹配token
    fn match_token(&self, lexeme: &str) -> Token {
        return match self.token_match_map.get(lexeme) {
            Some(token) => (*token).clone(),
            None => Token::new(TokenTypeEnum::ErrToken, lexeme, 0.0)
        };
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_independence() {
        let file = File::open("lex_test.txt").unwrap();
        let lexer = Lexer::new(file);

        let text = "+";

        let mut token1 = lexer.match_token(text);
        println!("token: {:?}", token1);

        token1.set_lexeme("//");
        println!("change local token: {:?}", token1);

        let token2 = lexer.match_token(text);
        println!("again get token: {:?}", token2);

        assert_eq!(text, token2.lexeme());
    }

    #[test]
    fn test_read_token_until_eof() {
        let file = File::open("lex_test.txt").unwrap();
        let mut lexer = Lexer::new(file);

        // let token = lexer.fetch_token();
        loop {
            let token = lexer.fetch_token();
            println!("{:?}",token);
            if *token.token_type()==TokenTypeEnum::NonToken{
                break
            }
        }
    }
}






