use crate::ast;
use crate::token;

use token::TokenKind;
use token::Tokenizer;

pub struct Parser<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
    pub root: ast::Node,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Parser<'a> {
        Self {
            tokenizer,
            root: ast::Node::Section(ast::Section::new(0)),
        }
    }

    pub fn parse(&mut self) {
        let tok = &mut self.tokenizer;

        match &mut self.root {
            ast::Node::Section(ref mut sec) => {
                let _ = parse_section(tok, sec).unwrap();
            }
            _ => {}
        }
    }
}

fn parse_section(tok: &mut Tokenizer, section: &mut ast::Section) -> Result<(), ()> {
    println!("parse_section");
    //assert_eq!(section, ast::Node::Section);
    loop {
        //let mut tok = tok.peekable();
        let l = &tok.before;
        let t = tok.peek();
        if t.is_none() {
            break;
        }
        let t = t.unwrap();

        dbg!(&t.kind);
        match t.kind {
            TokenKind::Title(level) => {
                // child section
                tok.next();
                let mut sec = ast::Section::new(level);
                let _ = parse_section(tok, &mut sec).unwrap();
                section.child.push(ast::Node::Section(sec));
            }
            TokenKind::Comment => {
                let _ = tok.next();
            }
            TokenKind::Unknown => {
                panic!("unknown token")
            }
            _ => {
                let p = get_paragraph(tok);
                if let Some(p) = p {
                    section.child.push(ast::Node::Paragraph(p));
                }
            }
        }
    }

    Ok(())
}

fn get_paragraph(tok: &mut Tokenizer) -> Option<ast::Paragraph> {
    println!("get_paragraph");
    let mut child = Vec::new();

    loop {
        //let mut tok = tokenizer.by_ref().peekable();
        let t = tok.peek();
        if t.is_none() {
            break;
        }
        let t = t.unwrap();

        dbg!(&t.kind);
        match t.kind {
            TokenKind::Newline => {
                //let tn = tok.peek();
                //if tn.is_none() {
                //    break;
                //}
                //let tn = tn.unwrap();
                let _ = tok.next();
                if tok.before == TokenKind::Newline {
                    break;
                }
            }
            TokenKind::Sentence => {
                let t = tok.next().unwrap();
                let s = tok.get_str(&t);
                child.push(ast::ParagraphChild::Sentence(s.to_string()));
            }
            _ => {
                tok.next();
                break;
            }
        }
    }

    //if child.is_empty() {
    //    return None;
    //}
    Some(ast::Paragraph { child })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple() {
        let s = r#"sentence0
= title level 1
sentence1
sentence2

p1s0
p1s1
"#;
        let mut tokenizer = token::Tokenizer::new(s);
        let t2 = tokenizer.clone();
        let token: Vec<token::Token> = t2.collect();
        for t in token {
            println!("{:?}: \"{}\"", t.kind, t.get_str(s));
        }

        let mut parser = parse::Parser::new(&mut tokenizer);
        parser.parse();

        dbg!(parser.root);
    }
}
