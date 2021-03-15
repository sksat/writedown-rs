//#![feature(inplace_iteration)]

pub mod ast;
pub mod parse;
pub mod token;

use ast::Node;
use parse::ParseError;

pub fn parse(src: &str) -> Result<Node, ParseError> {
    let mut tokenizer = token::Tokenizer::new(src);
    let parser = parse::Parser::new(&mut tokenizer);
    parser.parse()
}

pub trait Render {
    fn render(&self) -> String;
}
