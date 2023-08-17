use std::fs::File;
use std::io::{BufReader, BufRead};
// use std::str::Chars;

pub struct TextReader {
    aim_file_reader: BufReader<File>,
    line_buffer: Vec<char>,
    //line_buffer的索引，指向当前正在处理的char
    curr_handle_index: usize,
}

impl TextReader {
    pub fn new(aim_file: File) -> Self {
        let mut ret = TextReader {
            aim_file_reader: BufReader::new(aim_file),
            line_buffer: vec![],
            curr_handle_index: 0,
        };
        //刚开始先吃一个以达成一致性
        ret.eat_char();
        ret
    }

    /// 获取当前的char
    /// EOF时返回None
    pub fn get_char(&self) -> Option<char> {
        return match self.line_buffer.get(self.curr_handle_index) {
            Some(ch) => Some(*ch),
            None => None
        };
    }

    /// 读取下一个char，覆盖当前的char
    /// 读取是按行读的，读完缓存的一行后再从文件读下一行
    pub fn eat_char(&mut self) {
        self.curr_handle_index += 1;
        //读完了就再读一行（一直是None就是EOF了）
        if self.get_char() == None {
            let _ = self.read_line();
            self.curr_handle_index = 0;
        }
    }

    /// 从目标文件中读取一行存入缓存
    fn read_line(&mut self) -> Result<usize, usize> {
        let mut line = String::new();
        let size = self.aim_file_reader.read_line(&mut line).unwrap();
        self.line_buffer = line.chars().collect::<Vec<char>>();
        return match size {
            0 => Err(0),
            _ => Ok(size),
        };
    }
}

//尝试版本
// struct TextReader1<'a> {
//     curr_char: Option<char>,
//     aim_file_reader: BufReader<File>,
//     line_itr: Option<Chars<'a>>,
//     line: String,
// }
//
// impl<'a> TextReader1<'a> {
//     pub fn new(aim_file: File) -> Self {
//         let ret = TextReader1 {
//             curr_char: None,
//             aim_file_reader: BufReader::new(aim_file),
//             line_itr: None,
//             line: String::new(),
//         };
//         ret
//     }
//
//     /// 获取当前的char
//     /// EOF时返回None
//     pub fn get_char(&mut self) -> Option<char> {
//         self.curr_char
//     }
//
//     /// 读取下一个char，覆盖当前的char
//     /// 读取是按行读的，读完缓存的一行后再从文件读下一行
//     pub fn eat_char(&'a mut self) {
//         if let Some(itr) = &mut self.line_itr {
//             self.curr_char = itr.next()
//         }
//         //读完了就再读一行（一直是None就是EOF了）
//         if self.curr_char == None {
//             self.read_line();
//         }
//     }
//
//     /// 从目标文件中读取一行存入缓存
//     fn read_line(&'a mut self) {
//         self.aim_file_reader.read_line(&mut self.line).unwrap();
//         self.line_itr = Some(self.line.chars());
//     }
// }

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    fn test_continuous_read() {
        let file = File::open("example.txt").unwrap();

        let mut tr = TextReader::new(file);

        tr.eat_char();
        loop {
            let ch = tr.get_char();
            match ch {
                Some(c) => print!("{}", c),
                None => return
            }
            tr.eat_char()
        }
    }

    #[test]
    fn test_read_file() {
        // 打开文件并创建一个 BufReader
        let file = File::open("example.txt").unwrap();
        let reader = BufReader::new(file);

        // 逐个字符地读取文件
        for line in reader.lines() {
            if let Ok(line) = line {
                for character in line.chars() {
                    println!("{}", character);
                }
            }
        }
    }
}
