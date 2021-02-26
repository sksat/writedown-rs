use std::rc::Rc;

#[derive(Debug)]
pub enum Node {
    Top(Option<Header>, Rc<Node>),
    Paragraph(Vec<Node>),
    Func,
    List,
    Block(Block),
    Math,
    InlineCode,
    Link,
    Style(String),
    Unknown(String),
}

#[derive(Debug)]
pub struct Header {
    pub doctype: Option<()>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum Block {
    Code,
    Quote,
}

pub trait AST {}
