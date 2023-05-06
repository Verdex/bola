
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