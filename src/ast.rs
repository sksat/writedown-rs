use crate::token;

#[derive(Debug)]
pub enum Node {
    //Top(Option<Header>, Rc<Node>),
    Section(Section),
    Paragraph(Paragraph),
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
pub struct Section {
    pub level: usize,
    pub title: String,
    pub child: Vec<Node>,
}

#[derive(Debug)]
pub struct Paragraph {
    pub child: Vec<ParagraphChild>,
}

#[derive(Debug)]
pub enum ParagraphChild {
    Sentence(String),
    Func(Func),
}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub arg: Option<Vec<String>>,
    pub block: Option<String>,
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
    Code(String),
    Quote,
}

impl Section {
    pub fn new(title: token::Title) -> Self {
        Self {
            level: title.level,
            title: title.name,
            child: vec![],
        }
    }
}
