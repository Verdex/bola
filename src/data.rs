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

pub struct Il(pub String, pub Vec<fn(&mut Env) -> Result<(), MachineError>>);

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
    Exit,
}

#[derive(Debug, Clone)]
pub struct Env {
    def_stack : Vec<HashMap<String, IlData>>,
    data_stack : Vec<IlData>,
    func_stack : Vec<(Rc<Word>, usize)>,
    dict : HashMap<String, Rc<Word>>,
    parsers : Vec<Rc<Word>>,
}

impl Env {
    pub fn new() -> Self {
        Env { def_stack : vec![]
            , data_stack : vec![]
            , func_stack : vec![]
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

    pub fn push_data(&mut self, data : IlData) {
        self.data_stack.push(data);
    }

    pub fn pop_data(&mut self) -> Result<IlData, MachineError> {
        // TODO error
        self.data_stack.pop().ok_or(MachineError::Failure)
    }

    pub fn push_func(&mut self, word : &Rc<Word>, ip : usize) {
        self.func_stack.push((word.clone(), ip));
    }

    pub fn pop_func(&mut self) -> Result<(Rc<Word>, usize), MachineError> {
        // TODO error
        self.func_stack.pop().ok_or(MachineError::Failure)
    }

    pub fn get_dict(&self, target : &str) -> Result<Rc<Word>, MachineError> {
        // TODO error
        self.dict.get(target).ok_or(MachineError::Failure).map(|x| x.clone())
    }

    pub fn set_dict(&mut self, target : String, word : &Rc<Word>) -> Result<(), MachineError> {
        // TODO error (collision)
        self.dict.insert(target, word.clone());
        Ok(())
    }
}
