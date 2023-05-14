
use std::rc::Rc;
use crate::data::*;

pub fn push_float(value : f64) -> Word {
    let r = Il::InstrWithFloat { name : "push_float".to_owned()
                               , param: value
                               , f: |param, env| { env.push_data(IlData::Float(param)); Ok(()) }
                               };
    Word::Il(vec![r])
}

pub fn push_usize(value : usize) -> Word {
    let r = Il::InstrWithUsize { name : "push_usize".to_owned()
                               , param: value
                               , f: |param, env| { env.push_data(IlData::Usize(param)); Ok(()) }
                               };
    Word::Il(vec![r])
}

pub fn push_word(value : Rc<Word>) -> Word {
    let r = Il::InstrWithWord { name : "push_word".to_owned()
                              , param: value
                              , f: |param, env| { env.push_data(IlData::Word(param)); Ok(()) }
                              };
    Word::Il(vec![r])
}

pub fn push_string(value : String) -> Word {
    let r = Il::InstrWithString { name : "push_string".to_owned()
                                , param: value
                                , f: |param, env| { env.push_data(IlData::String(param)); Ok(()) }
                                };
    Word::Il(vec![r])
}

pub fn push_symbol(value : String) -> Word {
    let r = Il::InstrWithSymbol { name : "push_symbol".to_owned()
                                , param: value
                                , f: |param, env| { env.push_data(IlData::Symbol(param)); Ok(()) }
                                };
    Word::Il(vec![r])
}

pub fn def_word() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let sym = env.pop_data().unwrap(); // TODO error
        let def = env.pop_data().unwrap(); // TODO error

        // TODO can instead use pop_data_as
        if let (IlData::Symbol(name), IlData::List(code)) = (sym, def) {
            let mut func_addrs = vec![];
            for func_name in code.iter() {
                match func_name {
                    IlData::Symbol(word_name) => { // TODO all of these should be IlData::Word 
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

    Word::Il(vec![Il::Instr { name: "def_word".to_owned(), f : word }])
}