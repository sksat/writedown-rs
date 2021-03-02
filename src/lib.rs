pub mod token;

pub fn parse(src: &str) -> Result<(), ()> {
    let tokenizer = token::Tokenizer::new(src);
    let token: Vec<token::Token> = tokenizer.collect();

    for t in token {
        println!("token: {}", t.get_str(src));
    }

    Ok(())
}
