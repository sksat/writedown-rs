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
    Sentence,
    Indent(usize),
    TabIndent(usize),
    Title(usize),
    Literal(Literal),
    AtString, // @sksat@mstdn.maud.io
    Func,     // @<hoge>(arg){block}
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
        } else {
            token = self.get_token();
        }

        if let Some(t) = &token {
            assert!(self.pos <= t.pos);
            self.pos = t.pos + t.len;

            self.set_state(&t.kind);
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
    pub fn src(&self) -> &str {
        &self.src[self.pos..]
    }
    pub fn get_str(&self, token: &Token) -> &str {
        &self.src[token.pos..(token.pos + token.len)]
    }

    fn set_state(&mut self, last: &TokenKind) {
        self.state = match last {
            _ => last,
        }
        .clone();
    }

    pub fn get_top_token(&mut self) -> Option<Token> {
        if self.src().len() == 0 {
            return None;
        }
        let c = &self.src().chars().nth(0).unwrap();

        match c {
            '=' => {
                let title = get_title(&self.src());
                if title.is_some() {
                    let (level, name) = title.unwrap();
                    let title = TokenKind::Title(level);
                    println!("title({}): \"{}\"", level, name);

                    return Some(Token {
                        kind: title,
                        pos: self.pos + level + 1,
                        len: name.len(),
                    });
                }
            }
            '\n' => {
                return Some(Token {
                    kind: TokenKind::Newline,
                    pos: self.pos,
                    len: 1,
                });
            }
            '@' => {
                let at = get_at(&self.src());
                if let Some(at) = at {
                    let (kind, at) = at;
                    //println!("at: {}", at);

                    return Some(Token {
                        kind,
                        pos: self.pos + 1,
                        len: at.len(),
                    });
                }
            }
            _ => {
                let mut src = self.src().splitn(2, |c| match c {
                    '\n' => true,
                    _ => false,
                });
                let s = src.next();
                if let Some(s) = s {
                    return Some(Token {
                        kind: TokenKind::Sentence,
                        pos: self.pos,
                        len: s.len(),
                    });
                }
            }
        }

        None
    }

    pub fn get_token(&self) -> Option<Token> {
        match &self.src().chars().nth(0).unwrap() {
            '\n' => {
                return Some(Token {
                    kind: TokenKind::Newline,
                    pos: self.pos,
                    len: 1,
                });
            }
            '@' => {
                let at = get_at(&self.src());
                if let Some(at) = at {
                    let (kind, at) = at;
                    //println!("at: {}", at);

                    return Some(Token {
                        kind,
                        pos: self.pos + 1,
                        len: at.len(),
                    });
                }
            }
            _ => {
                let mut src = self.src().splitn(2, |c| match c {
                    '\n' => true,
                    _ => false,
                });
                let s = src.next();
                if let Some(s) = s {
                    return Some(Token {
                        kind: TokenKind::Sentence,
                        pos: self.pos,
                        len: s.len(),
                    });
                }
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

fn get_at(s: &str) -> Option<(TokenKind, &str)> {
    let mut it = s.chars();
    assert_eq!(it.next().unwrap(), '@');
    let first = it.next().unwrap();

    let it = it.enumerate();
    if first == '<' {
        // func
        for c in it {
            let (i, c) = c;
            //println!("{}, {}", i, c);
            if c == '>' {
                return Some((TokenKind::Func, &s[..i + 3]));
            }
        }
        return None;
    }

    let mut n = 1;

    for c in it {
        let (i, c) = c;
        match c {
            'a'..='z' | 'A'..='Z' => continue,
            '0'..='9' | '.' | '_' => continue,
            _ => {
                n += i;
                break;
            }
        }
    }

    return Some((TokenKind::AtString, &s[..n]));
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

@sksat_tty
@<func>

function test: @<func>
SNS test: @sksat_tty @sksat@mstdn.maud.io
email test: sksat@sksat.net
"#;

        let mut tokenizer = token::Tokenizer::new(s);
        println!("string:\n{}", s);
        for t in tokenizer.clone() {
            //println!("token: {:?}", token.collect::<Vec<token::Token>>());
            println!("{:?}: \"{}\"", t, tokenizer.get_str(&t));
        }
    }

    #[test]
    fn title() {
        let (n, s) = token::get_title("== hoge").unwrap();
        assert_eq!(n, 2);
        assert_eq!(s, "hoge");
    }
}
