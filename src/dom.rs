use std::collections::{HashMap, HashSet};

type AttrMap = HashMap<String, String>;

// DOM 节点
#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

// DOM 节点类型, 只抽象了文本与元素类型
#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

// 元素包含：标签名、多个属性(KV存储)
#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

// 节点构造函数
pub fn text(data: String) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node{
        children: children,
        node_type: NodeType::Element(ElementData{
            tag_name: name,
            attributes: attrs,
        })
    }
}

// ElementData 方法: 获取 id 和 class
impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}