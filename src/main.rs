
mod data;
mod standard;
mod machine;

fn main() {
    use crate::data::*;
    use crate::standard::word::*;

    let mut env = Env::new();

    let two = push_float(2.0);

    env.define_word("two".to_owned(), &two);
    env.define_word("main".to_owned(), &two);

    crate::machine::execute("", &mut env).unwrap();

    println!("{:?}", env);
}
