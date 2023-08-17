mod text_reader;

use std::fs::File;
use std::collections::hash_map::HashMap;

use super::token_manager::*;

pub struct Lexer {
    //字符读取器
    text_reader: text_reader::TextReader,
    //符号表
    token_match_map: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(file: File) -> Self {
        Lexer {
            text_reader: text_reader::TextReader::new(file),
            token_match_map: TokenTypeEnum::generate_token_match_map(),
        }
    }

    pub fn fetch_token(&mut self) -> Token {
        return Token::new(TokenTypeEnum::ErrToken, "123abc", 0.0);
    }

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
    fn test_read_token_basic() {
        let file = File::open("example.txt").unwrap();
        let lexer = Lexer::new(file);

        let text = "+";

        let mut token1 = lexer.match_token(text);
        println!("token: {:?}", token1);

        token1.set_lexeme("//");
        println!("change local token: {:?}", token1);

        let mut token2 = lexer.match_token(text);
        println!("again get token: {:?}", token2);

        assert_eq!(text, token2.lexeme());
    }
}






