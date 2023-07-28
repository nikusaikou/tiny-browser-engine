use crate::dom;
use std::collections::HashMap;

pub struct Parser {
    pub pos: usize, // 记录当前位置
    pub input: String,
}

impl Parser {
    // 处理 char， 直到 test 返回 false, 用来处理一串空字符或一串数字字母字符
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        // 泛型约束，接收一个参数为 char 返回值为 bool 的函数
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    pub fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap(); // next 返回 下标和字符，unwrap 隐式处理 Option，如果是 None 则 panic
        let (next_pos, _) = iter.next().unwrap_or((1, ' ')); // 如果是 None 则返回 (1, ' ')
        self.pos += next_pos;
        cur_char
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // 处理完所有输入返回 true
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

// 解析 HTML 返回根 DOM 节点
// pub fn parse(source: String) -> dom::Node {
//     let mut nodes = Parser {
//         pos: 0,
//         input: source,
//     }
//     .parse_nodes();

//     if nodes.len() == 1 {
//         nodes.swap_remove(0)
//     } else {
//         dom::elem("html".to_string(), HashMap::new(), nodes)
//     }
// }
