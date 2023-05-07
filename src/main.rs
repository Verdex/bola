
mod data;
mod standard;
mod machine;

fn main() {
    use crate::data::*;
    use crate::standard::word::*;

    let mut env = Env::new();

    env.define_word("parse_whitespace".to_owned(), crate::standard::parser::parse_whitespace());
    env.parsers.push(env.lookup_word("parse_whitespace").unwrap());


    let result = crate::machine::execute("  ".to_owned(), &mut env);

    println!("{:?}", result);
    println!("{:?}", env);
}
