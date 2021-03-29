use std::ops::Fn;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pos: usize,
    len: usize,
}

impl Token {
    pub fn get_str<'a>(&self, src: &'a str) -> &'a str {
        &src[self.pos..self.pos + self.len]
    }
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
    Title(Title),
    Literal(Literal),
    AtString, // @sksat@mstdn.maud.io
    Tag,      // @[tag]
    Func,     // @<hoge>(arg){block}
    FuncArgOpen,
    FuncArgClose,
    FuncArg,
    FuncBlock,
    Math,       // $y = f(x)$
    InlineCode, // `printf("hello");`
    Quote,      // > quote
    CodeBlock,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Title {
    pub level: usize,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
    before: TokenKind,
    now: Option<Token>,
    peeked: Option<Option<Token>>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.now = match self.peeked.take() {
            Some(v) => {
                //println!("next(peek): {:?}", v);
                v
            }
            None => {
                let n = self.next_token();
                //println!("next(new):   {:?}", n);
                //self.next_token()
                n
            }
        };
        self.now.clone()
    }
}

//unsafe impl<'a> SourceIter for &mut Tokenizer<'a> {
//    type Source = Tokenizer<'a>;
//
//    unsafe fn as_inner(&mut self) -> &mut Self::Source {
//        self
//    }
//}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            before: TokenKind::Newline,
            now: None,
            peeked: None,
        }
    }
    pub fn src(&self) -> &str {
        &self.src[self.pos..]
    }
    pub fn get_str(&self, token: &Token) -> &str {
        &self.src[token.pos..(token.pos + token.len)]
    }

    pub fn now(&self) -> Option<Token> {
        self.now.clone()
    }

    pub fn peek(&mut self) -> Option<Token> {
        return if self.peeked.is_some() {
            self.peeked.clone().unwrap()
        } else {
            let t = self.next_token();
            self.peeked.get_or_insert(t).clone()
        };
        //let t = self.next_token();
        //self.peeked.get_or_insert(t).clone()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        //let mut token = None;
        let token = match self.before {
            TokenKind::Newline => self.get_top_token(),
            TokenKind::Func => self.get_func_ext_or_default(),
            TokenKind::FuncArgOpen | TokenKind::FuncArg => self.get_func_arg(),
            TokenKind::FuncArgClose | TokenKind::FuncBlock => self.get_func_block_or_default(),
            _ => self.get_token(),
        };

        if token.is_none() {
            return None;
        }
        let t = token.unwrap();

        assert!(self.pos <= t.pos);
        assert!(t.len != 0);
        self.pos = t.pos + t.len;

        self.before = t.kind.clone();

        // skip
        match t.kind {
            TokenKind::Tag => {
                let _ = self.skip_one(']');
            }
            TokenKind::Func => {
                let _ = self.skip_one('>');
            }
            TokenKind::FuncArgOpen | TokenKind::FuncArg => {
                let _ = self.skip_one(',');
                self.skip_whitespace();
            }
            TokenKind::FuncBlock => {
                let _ = self.skip_one('}');
            }
            TokenKind::InlineCode => {
                let _ = self.skip_one('`');
            }
            TokenKind::CodeBlock => {
                let _ = self.skip_one('`');
                let _ = self.skip_one('`');
                let _ = self.skip_one('`');
            }
            _ => {}
        }

        //println!("next_token: {:?}", &t);
        Some(t)
    }

    pub fn skip_one(&mut self, c: char) -> Option<()> {
        let src = self.src();
        let s0 = src.chars().nth(0)?;

        if s0 == c {
            self.pos += 1;
            return Some(());
        }

        None
    }

    pub fn skip_char<F>(&mut self, f: F)
    where
        F: Fn(char) -> bool,
    {
        let src = self.src().chars();
        let mut n = 0;
        for c in src {
            if f(c) {
                n += 1;
            }
            {
                break;
            }
        }
        self.pos += n;
    }

    pub fn skip_whitespace(&mut self) {
        self.skip_char(|c| c.is_whitespace())
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
                    let t = title.unwrap();
                    let level = t.level;
                    let len = t.name.len();
                    let kind = TokenKind::Title(t);

                    return Some(Token {
                        kind,
                        pos: self.pos + level + 1,
                        len,
                    });
                }
            }
            '\n' => {
                return Some(Token {
                    kind: TokenKind::Newline,
                    pos: self.pos,
                    len: 1,
                })
            }
            '@' => return self.get_at(),
            '`' => return self.get_code(),
            _ => {
                let s = get_sentence(self.src());
                return Some(Token {
                    kind: TokenKind::Sentence,
                    pos: self.pos,
                    len: s.len(),
                });
            }
        }

        None
    }

    pub fn get_token(&mut self) -> Option<Token> {
        match &self.src().chars().nth(0).unwrap() {
            '\n' => {
                return Some(Token {
                    kind: TokenKind::Newline,
                    pos: self.pos,
                    len: 1,
                });
            }
            '@' => {
                return self.get_at();
            }
            _ => {
                let s = get_sentence(self.src());
                return Some(Token {
                    kind: TokenKind::Sentence,
                    pos: self.pos,
                    len: s.len(),
                });
            }
        }
    }

    pub fn get_at(&mut self) -> Option<Token> {
        assert_eq!(self.src().chars().nth(0).unwrap(), '@');
        self.skip_one('@');

        let src = self.src();
        let mut c = src.char_indices();
        let first = c.next().unwrap().1;
        match first {
            '<' | '[' => loop {
                let c = c.next();
                if c.is_none() {
                    return None;
                }
                let (i, c) = c.unwrap();

                let (end, kind) = match first {
                    '[' => (']', TokenKind::Tag),
                    '<' => ('>', TokenKind::Func),
                    _ => unreachable!(),
                };
                if c == end {
                    return Some(Token {
                        kind,
                        pos: self.pos + 1,
                        len: i - 1,
                    });
                }
            },
            _ => {}
        }

        for c in c {
            let (i, c) = c;
            match c {
                'a'..='z' | 'A'..='Z' => continue,
                '0'..='9' | '.' | '_' => continue,
                _ => {
                    return Some(Token {
                        kind: TokenKind::AtString,
                        pos: self.pos,
                        len: i,
                    });
                }
            }
        }
        None
    }

    pub fn get_code(&mut self) -> Option<Token> {
        assert_eq!(self.src().chars().nth(0).unwrap(), '`');
        self.skip_one('`');

        let src = self.src();
        let mut src = src.char_indices();
        let f = src.next().unwrap().1;
        if f != '`' {
            // inline code
            for c in src {
                let (i, c) = c;
                if c == '`' {
                    return Some(Token {
                        kind: TokenKind::InlineCode,
                        pos: self.pos,
                        len: i,
                    });
                }
            }

            todo!("error: no end mark of inline code");
        }

        let c = src.next().unwrap().1;
        if c != '`' {
            unimplemented!("code block error");
        }

        let c = src.next().unwrap().1;
        if c == '\n' {
            // default code block(no language)
            let mut count = 0;
            for c in src {
                let (i, c) = c;
                if c == '`' {
                    count += 1;
                    if count == 3 {
                        return Some(Token {
                            kind: TokenKind::CodeBlock,
                            pos: self.pos + 2,
                            len: i - 4,
                        });
                    }
                    continue;
                }
                count = 0;
            }
            todo!("code block error");
        } else if c == ':' {
            // code block with language
            todo!("get language");
        } else {
            todo!("code block error");
        }
    }

    pub fn get_func_ext_or_default(&mut self) -> Option<Token> {
        let src = &self.src();
        return match &src.chars().nth(0).unwrap() {
            '(' => Some(Token {
                kind: TokenKind::FuncArgOpen,
                pos: self.pos,
                len: 1,
            }),
            '{' => self.get_func_block_or_default(),
            _ => self.get_token(),
        };
    }

    pub fn get_func_arg(&self) -> Option<Token> {
        let src = &self.src();
        if src.chars().nth(0).unwrap() == ')' {
            return Some(Token {
                kind: TokenKind::FuncArgClose,
                pos: self.pos,
                len: 1,
            });
        }

        let src = src.chars().enumerate();

        for c in src {
            let (i, c) = c;
            match c {
                ',' | ')' => {
                    //println!("arg: {}", &self.src()[..i]);
                    return Some(Token {
                        kind: TokenKind::FuncArg,
                        pos: self.pos,
                        len: i,
                    });
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_func_block_or_default(&mut self) -> Option<Token> {
        let src = &self.src();
        if src.chars().nth(0).unwrap() != '{' {
            return self.get_token();
        }

        let mut n = 0;
        for c in src.char_indices() {
            let (i, c) = c;
            if c == '}' {
                n = i;
                break;
            }
        }

        Some(Token {
            kind: TokenKind::FuncBlock,
            pos: self.pos + 1,
            len: n - 1,
        })
    }
}

fn get_sentence(s: &str) -> &str {
    let it = s.chars().enumerate();

    let mut before = 'A';
    let mut n = 0;
    for c in it {
        let (i, c) = c;

        match c {
            '\n' => break,
            '@' => {
                if before.is_whitespace() {
                    break;
                }
            }
            _ => {}
        }
        n = i;
        before = c;
    }
    let n = s.char_indices().map(|(i, _)| i).nth(n + 1).unwrap();
    //println!("sentence: \"{}\"", &s[..n]);
    &s[..n]
}

fn get_title(mut s: &str) -> Option<Title> {
    let mut level = 0;
    let mut it = s.chars().enumerate();

    // get level
    loop {
        let c = it.next();
        if c.is_none() {
            return None;
        }
        let (_, c) = c.unwrap();
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

    Some(Title {
        level,
        name: s.to_string(),
    })
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
@<func>(arg)
@<func>(arg1,arg2)
@<func>(arg1,arg2,arg3){block}
@<func>(a1, a2, a3){
    b0
    b1
    b3
}

function test: @<func>
SNS test: @sksat_tty @sksat@mstdn.maud.io
email test: sksat@sksat.net

適当な文章 @<func>(a)がある
脚注だいすき！いちばんすきな注です！ @<ft>{そうか？}

=== tag

@[hoge]

タグが打てると，うれしいんじゃ @[footnote]

@<jmp>(hoge)
@<ftref>(footnote){脚注もタグにくっつけられると，うれしいんじゃ(分けて書きたい時もあるため)}

`println!("hello, world!");`

code block

```
#include <stdio.h>

int main(int argc, char **argv){
    printf("hello, world!");
    return 0;
}
```

code block(set language option)

```
#include <iostream>

int main(int argc, char **argv){
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
```

"#;

        let tokenizer = token::Tokenizer::new(s);
        println!("string:\n{}", s);
        for t in tokenizer {
            println!("{:?}: \"{}\"", t, t.get_str(&s));
        }
    }

    #[test]
    fn title() {
        let t = token::get_title("== hoge").unwrap();
        assert_eq!(t.level, 2);
        assert_eq!(t.name, "hoge");
    }
}
