mod lexer;
mod parser; //TODO enum代表树，每个变体都是一种树类型，附带对应结构体数据；树本身可以用trait来代替
mod exception;
mod token_manager;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
