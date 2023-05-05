
use std::rc::Rc;
use std::collections::HashMap;

use crate::data::*;

pub fn execute( prog : &str, env : &mut Env ) -> Result<(), MachineError> {

    let mut ip : usize = 0;

    let mut current_word : Rc<Word> = env.get_dict("main").unwrap();

    'main_loop : loop {
        match &*current_word {
            Word::Func(_) => {

            },
            Word::Il(instrs) => {
                for instr in instrs.iter() {
                    instr.call(env)?;
                }
            },
            Word::Exit => {
                break 'main_loop;
            }
        }
    }

    Ok(())
}