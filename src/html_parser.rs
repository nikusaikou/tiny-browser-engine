use crate::dom;
use std::collections::HashMap;

struct Parser {
    pos: usize, // 记录当前位置
    input: String,
}

impl Parser {
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    // 解析单个节点，第一个字符是 < 解析为节点，否则为文本
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    // 解析元素节点，不断循环解析标签、属性、子节点
    fn parse_element(&mut self) -> dom::Node {
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        let children = self.parse_nodes();

        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        dom::elem(tag_name, attrs, children)
    }

    // 解析属性，attrName="attrValue" 类型的 kv 对
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // 解析标签名
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            // 使用匿名函数传递闭包给 consume_while
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }
    // 处理 char， 直到 test 返回 false, 用来处理一串空字符或一串数字字母字符
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool, // 泛型约束，接收一个参数为 char 返回值为 bool 的函数
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    // 处理单个字符，返回当前字符并将 pos 加 1
    fn consume_char(&mut self) -> char {
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
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}
