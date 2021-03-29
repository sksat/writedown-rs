use std::fs;
use std::io::{self, Read};
use std::path::Path;

use clap::{App, Arg};

use writedown::token;

fn main() {
    let matches = App::new("wd-tokenize")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("SRC").help("source file").index(1))
        .get_matches();

    let fname = matches.value_of("SRC").unwrap();

    let file = Path::new(fname);
    let mut file = io::BufReader::new(fs::File::open(file).unwrap());

    let mut src = String::new();
    let _ = file.read_to_string(&mut src).unwrap();

    let mut tokenizer = token::Tokenizer::new(&src);
    let token: Vec<token::Token> = tokenizer.collect();
    for t in token {
        println!("{:?}: \"{}\"", t.kind, t.get_str(&src));
    }
}
