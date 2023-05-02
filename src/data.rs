use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum IlData {
    Float(f64),
    Symbol(String),
    String(String),
    Tuple(Vec<IlData>),
    List(Vec<IlData>),
    Pattern(IlPat),
}

#[derive(Debug, Clone)]
pub enum IlPat {
    Float(f64),
    Symbol(String),
    UnboundVariable(String),
    Tuple(Vec<IlPat>),
}

#[derive(Debug)]
pub enum MachineError {
    Failure
}

pub struct Il(String, Vec<fn(&mut Env) -> Result<(), MachineError>>);

impl std::fmt::Debug for Il {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Il")
         .field("name", &self.0)
         .finish()
    }
}

#[derive(Debug)]
pub enum Word {
    Il(Il),
    Func(Vec<Rc<Word>>),
}

#[derive(Debug, Clone)]
pub struct Env {
    def_stack : Vec<HashMap<String, IlData>>,
    data_stack : Vec<IlData>,
    dict : HashMap<String, Rc<Word>>,
    parsers : Vec<Rc<Word>>,
}

impl Env {
    pub fn new() -> Self {
        Env { def_stack : vec![]
            , data_stack : vec![]
            , dict : HashMap::new()
            , parsers : vec![]
            }
    }

    pub fn get_def(&self, name : &String) -> Option<&IlData> {
        let target = self.def_stack.iter().rev().find(|map| map.contains_key(name));
        match target { 
            Some(map) => map.get(name),
            None => None,
        }
    } 

    pub fn set_def(&mut self, name : String, data : IlData) -> Result<(), MachineError> {
        // TODO collision?
        let last = self.def_stack.len() - 1;
        self.def_stack[last].insert(name, data);
        Ok(())
    }

    pub fn push_def(&mut self) {
        self.def_stack.push(HashMap::new());
    }

    pub fn pop_def(&mut self) -> Result<(), MachineError> {
        // TODO error
        self.def_stack.pop().ok_or(MachineError::Failure).map(|_| ())
    }
}
