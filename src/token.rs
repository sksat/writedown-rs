use std::iter::Enumerate;
use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pos: usize,
    len: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Str(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Comment, // # comment
    Newline,
    Indent(usize),
    TabIndent(usize),
    Title,
    Literal(Literal),
    Func, // @<hoge>(arg){block}
    FuncArg,
    FuncBlock,
    Math,       // $y = f(x)$
    InlineCode, // `printf("hello");`
    Quote,      // > quote
    CodeBlock,
    Unknown,
}

//#[derive(PartialEq)]
//enum TokenizerState {
//    Normal,
//    Newline,
//    Indent(usize),
//    TabIndent(usize),
//    FuncArg,
//    CodeBlock,
//    Unknown,
//}

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
    state: TokenKind,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = None;
        if self.state == TokenKind::Newline {
            token = self.get_top_token();
        }

        if let Some(t) = &token {
            assert!(self.pos <= t.pos);
            self.pos = t.pos + t.len;
        }
        token
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            state: TokenKind::Newline,
        }
    }
    pub fn get_src(&self) -> &str {
        &self.src[self.pos..]
    }
    pub fn get_str(&self, token: &Token) -> &str {
        &self.src[token.pos..(token.pos + token.len)]
    }

    pub fn get_top_token(&mut self) -> Option<Token> {
        //let c = self.src.chars().nth(0).unwrap();
        let c = self.get_src().chars().nth(0).unwrap();

        match c {
            '=' => {
                let title = get_title(&self.src);
                if title.is_some() {
                    let (level, name) = title.unwrap();
                    //println!("title({}): \"{}\"", level, name);

                    return Some(Token {
                        kind: TokenKind::Title,
                        pos: self.pos + level + 1,
                        len: name.len(),
                    });
                }
            }
            _ => {
                println!("char: {}", c);
            }
        }

        None
    }
}

fn get_title(mut s: &str) -> Option<(usize, &str)> {
    let mut level = 0;
    let mut it = s.chars().enumerate();

    // get level
    loop {
        let c = it.next();
        if c.is_none() {
            return None;
        }
        let (i, c) = c.unwrap();
        match c {
            ' ' => {
                s = &s[level + 1..];
                s = s.split('\n').next().unwrap();
                break;
            }
            '=' => level += 1,
            _ => return None,
        }
    }

    Some((level, s))
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn simple() {
        let s = r#"= title level 1
hoge fuga
aaa===beabnea

== title level 2
"#;

        let mut tokenizer = token::Tokenizer::new(s);
        println!("string:\n{}", s);
        for t in tokenizer.clone() {
            //println!("token: {:?}", token.collect::<Vec<token::Token>>());
            println!("\"{}\": {:?}", tokenizer.get_str(&t), t);
        }
    }

    #[test]
    fn title() {
        let (n, s) = token::get_title("== hoge").unwrap();
        assert_eq!(n, 2);
        assert_eq!(s, "hoge");
    }
}
