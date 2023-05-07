
use std::rc::Rc;

use crate::data::*;

pub fn push_float(value : f64) -> Rc<Word> {
    let r = Il::InstrWithFloat { name : "push_float".to_owned()
                               , param: value
                               , f: |param, env| { env.push_data(IlData::Float(param)); Ok(()) }
                               };
    Rc::new(Word::Il(vec![r]))
}

pub fn push_string(value : String) -> Rc<Word> {
    let r = Il::InstrWithString { name : "push_string".to_owned()
                                , param: value
                                , f: |param, env| { env.push_data(IlData::String(param)); Ok(()) }
                                };
    Rc::new(Word::Il(vec![r]))
}

pub fn push_symbol(value : String) -> Rc<Word> {
    let r = Il::InstrWithSymbol { name : "push_symbol".to_owned()
                                , param: value
                                , f: |param, env| { env.push_data(IlData::Symbol(param)); Ok(()) }
                                };
    Rc::new(Word::Il(vec![r]))
}

pub fn def_word() -> Rc<Word> {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let sym = env.pop_data().unwrap(); // TODO error
        let def = env.pop_data().unwrap(); // TODO error

        if let (IlData::Symbol(name), IlData::List(code)) = (sym, def) {
            let mut func_addrs = vec![];
            for func_name in code.iter() {
                match func_name {
                    IlData::Symbol(word_name) => {
                        let addr = env.lookup_word(word_name).unwrap(); // TODO
                        func_addrs.push(addr);
                    },
                    _ => todo!(), // TODO unreachable?
                }
            }
            env.define_word(name, Word::Func(func_addrs))?;
        }
        else {
            // TODO
        }

        Ok(())
    }

    Rc::new(Word::Il(vec![Il::Instr { name: "def_word".to_owned(), f : word }]))
}