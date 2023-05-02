
use std::rc::Rc;
use std::collections::HashMap;

use crate::data::*;

// TODO no more main
// TODO &str of input
pub fn execute( env : &mut Env ) -> Result<(), MachineError> {

    let mut ip : usize = 0;

    let mut current_word : Rc<Word> = env.get_dict("main").unwrap();

    'main_loop : loop {
        match &*current_word {
            Word::Func(_) => {

            },
            Word::Il(instrs) => {
                for instr in instrs.1.iter() {
                    instr(env)?;
                }
            },
            Word::Exit => {
                break 'main_loop;
            }
        }
    }

    Ok(())
}