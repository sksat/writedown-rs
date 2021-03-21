use crate::ast;
use crate::token;

use token::TokenKind;
use token::Tokenizer;

#[derive(Debug)]
pub enum ParseError {}

pub struct Parser<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
    pub root: ast::Node,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Parser<'a> {
        Self {
            tokenizer,
            root: ast::Node::Section(ast::Section::new(token::Title {
                level: 0,
                name: "".to_string(),
            })),
        }
    }

    pub fn parse(mut self) -> Result<ast::Node, ParseError> {
        let tok = &mut self.tokenizer;

        match &mut self.root {
            ast::Node::Section(ref mut sec) => {
                let _ = parse_section(tok, sec).unwrap();
            }
            _ => {}
        }

        Ok(self.root)
    }
}

fn parse_section(tok: &mut Tokenizer, section: &mut ast::Section) -> Result<(), ()> {
    //println!("parse_section");
    //assert_eq!(section, ast::Node::Section);
    loop {
        //let l = &tok.before;
        let t = tok.peek();
        if t.is_none() {
            break;
        }
        let t = t.unwrap();

        match t.kind {
            TokenKind::Title(title) => {
                // child section
                tok.next();
                //let t = tok.peek().unwrap();
                //if t.kind == TokenKind::Newline {
                let mut sec = ast::Section::new(title);
                let _ = parse_section(tok, &mut sec).unwrap();
                section.child.push(ast::Node::Section(sec));
            }
            TokenKind::Comment | TokenKind::Newline => {
                let _ = tok.next();
            }
            TokenKind::Unknown => {
                panic!("unknown token")
            }
            TokenKind::CodeBlock => {
                let blk = ast::Block::Code(tok.get_str(&t).to_string());
                section.child.push(ast::Node::Block(blk));
                let _ = tok.next();
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
    let mut child = Vec::new();

    loop {
        //let mut tok = tokenizer.by_ref().peekable();
        let t = tok.peek();
        if t.is_none() {
            break;
        }
        let t = t.unwrap();

        match t.kind {
            TokenKind::Newline => {
                //let tn = tok.peek();
                //if tn.is_none() {
                //    break;
                //}
                //let tn = tn.unwrap();
                if tok.now().unwrap().kind == TokenKind::Newline {
                    //println!("new paragraph");
                    let _ = tok.next();
                    break;
                }
                let _ = tok.next();
            }
            TokenKind::Sentence => {
                let t = tok.next().unwrap();
                let s = tok.get_str(&t);
                child.push(ast::ParagraphChild::Sentence(s.to_string()));
            }
            TokenKind::Title(_) | TokenKind::CodeBlock => break,
            _ => {
                dbg!(&t.kind);
                tok.next();
                break;
            }
        }
    }

    if child.is_empty() {
        return None;
    }
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
sentence3
sentence4

p1s0
p1s1
"#;
        let mut tokenizer = token::Tokenizer::new(s);
        let t2 = tokenizer.clone();
        let token: Vec<token::Token> = t2.collect();
        for t in token {
            println!("{:?}: \"{}\"", t.kind, t.get_str(s));
        }

        let parser = parse::Parser::new(&mut tokenizer);
        let ast = parser.parse().unwrap();

        dbg!(&ast);

        assert_matches!(&ast, ast::Node::Section(_));
        let s = match ast {
            ast::Node::Section(s) => s,
            _ => panic!(""),
        };
        assert_eq!(s.child.len(), 2);

        let s0 = &s.child[0];
        let s1 = &s.child[1];

        let s0 = match s0 {
            ast::Node::Paragraph(p) => p,
            _ => panic!(),
        };
        assert_eq!(s0.child.len(), 1);

        let s1 = match s1 {
            ast::Node::Section(s) => s,
            _ => panic!(""),
        };
        assert_eq!(s1.child.len(), 2);

        let s10 = &s1.child[0];
        let s11 = &s1.child[1];

        let s10 = match s10 {
            ast::Node::Paragraph(p) => p,
            _ => panic!(""),
        };
        let s11 = match s11 {
            ast::Node::Paragraph(p) => p,
            _ => panic!(""),
        };

        assert_eq!(s10.child.len(), 4);
        assert_eq!(s11.child.len(), 2);
    }
}
