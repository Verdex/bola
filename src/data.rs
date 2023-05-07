use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum IlData {
    Float(f64),
    Usize(usize),
    Symbol(String),
    String(String),
    Pattern(IlPat),
    Tuple(Vec<IlData>),
    List(Vec<IlData>),
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

pub enum Il {
    Instr { name: String, f : fn(&mut Env) -> Result<(), MachineError> },
    InstrWithUsize { name: String, param : usize, f : fn(usize, &mut Env) -> Result<(), MachineError> },
    InstrWithFloat { name: String, param : f64, f : fn(f64, &mut Env) -> Result<(), MachineError> },
    InstrWithString { name: String, param : String, f : fn(String, &mut Env) -> Result<(), MachineError> },
    InstrWithSymbol { name: String, param : String, f : fn(String, &mut Env) -> Result<(), MachineError> },
}

impl Il {
    pub fn name(&self) -> &str {
        match self {
            Il::Instr { name, .. } => name,
            Il::InstrWithUsize { name, .. } => name,
            Il::InstrWithFloat { name, .. } => name,
            Il::InstrWithString { name, .. } => name,
            Il::InstrWithSymbol { name, .. } => name,
        }
    }

    pub fn call(&self, env : &mut Env) -> Result<(), MachineError> {
        match self {
            Il::Instr { f, .. } => f(env),
            Il::InstrWithUsize { f, param, .. } => f(*param, env),
            Il::InstrWithFloat { f, param, .. } => f(*param, env),
            Il::InstrWithString { f, param, .. } => f(param.clone(), env),
            Il::InstrWithSymbol { f, param, .. } => f(param.clone(), env),
        }
    }
}

impl std::fmt::Debug for Il {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Il")
         .field("name", &self.name())
         .finish()
    }
}

#[derive(Debug)]
pub enum Word {
    Il(Vec<Il>),
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

    pub fn new_def_level(&mut self) {
        self.def_stack.push(HashMap::new());
    }

    pub fn pop_def_level(&mut self) {
        self.def_stack.pop();
    }

    pub fn push_data(&mut self, data : IlData) {
        self.data_stack.push(data);
    }

    pub fn pop_data(&mut self) -> Result<IlData, MachineError> {
        // TODO error
        self.data_stack.pop().ok_or(MachineError::Failure)
    }

    pub fn push_func_restore_point(&mut self, word : &Rc<Word>, ip : usize) {
        self.func_stack.push((word.clone(), ip));
    }

    pub fn pop_func_restore_point(&mut self) -> Option<(Rc<Word>, usize)> {
        self.func_stack.pop()
    }

    pub fn lookup_word(&self, target : &str) -> Result<Rc<Word>, MachineError> {
        // TODO error
        self.dict.get(target).ok_or(MachineError::Failure).map(|x| x.clone())
    }

    pub fn define_word(&mut self, target : String, word : Word) -> Result<(), MachineError> {
        // TODO error (collision)
        self.dict.insert(target, Rc::new(word));
        Ok(())
    }
}
