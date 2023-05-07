
mod data;
mod standard;
mod machine;

fn main() {
    use crate::data::*;
    use crate::standard::word::*;

    let mut env = Env::new();

    crate::machine::execute("", &mut env).unwrap();

    println!("{:?}", env);
}
