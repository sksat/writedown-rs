use std::iter::Enumerate;
use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

pub enum Literal {
    Str(String),
    Int(i64),
    Float(f64),
}

pub enum TokenKind {
    Comment, // # comment
    Newline,
    Indent,
    TabIndent,
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

#[derive(PartialEq)]
enum TokenizerState {
    Normal,
    Newline,
    Indent(usize),
    TabIndent(usize),
    Func,
    FuncArg,
    CodeBlock,
    Unknown,
}

pub struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
    state: TokenizerState,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            state: TokenizerState::Newline,
        }
    }
    pub fn get_src(&self) -> &str {
        &self.src[self.pos..]
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == TokenizerState::Newline {
            let token = self.get_top_token();
            return token.as_ref();
        }
        None
    }
}

pub fn get_token(src: &str) -> Token {
    let c = src.chars().nth(0).unwrap();

    match c {
        '=' => {
            let title = get_title(&src);
            if title.is_some() {
                let (level, name) = title.unwrap();
                println!("title({}): \"{}\"", level, name);

                return Token { s: name };
            }
        }
        '\n' => {
            return Token { s: &src[..1] };
        }
        _ => {
            println!("char: {}", c);
        }
    }

    Token { s: &src[..1] }
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
        let token = token::tokenize(s);
        println!("string:\n{}", s);
        println!("token: {:?}", token.collect::<Vec<token::Token>>());
    }

    #[test]
    fn title() {
        let (n, s) = token::get_title("== hoge").unwrap();
        assert_eq!(n, 2);
        assert_eq!(s, "hoge");
    }
}
