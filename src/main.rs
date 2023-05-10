
mod data;
mod standard;
mod machine;


pub fn blarg() -> crate::data::Word {
    let r = crate::data::Il::Instr { name : "blarg".to_owned()
                      , f: |env| { println!("blarg"); Ok(()) }
                      };
    crate::data::Word::Il(vec![r])
}

fn main() {
    use crate::data::*;
    use crate::standard::word::*;

    let blarg = blarg();

    let mut env = Env::new();

    env.define_word("blarg".to_owned(), blarg);

    env.define_word("parse_whitespace".to_owned(), crate::standard::parser::parse_whitespace()).unwrap();
    env.parsers.push(env.lookup_word("parse_whitespace").unwrap());

    env.define_word("parse_word_symbol".to_owned(), crate::standard::parser::parse_word_symbol()).unwrap();
    env.parsers.push(env.lookup_word("parse_word_symbol").unwrap());


    let result = crate::machine::execute("  blarg".to_owned(), &mut env);

    println!("{:?}", result);
    println!("{:?}", env);
}
