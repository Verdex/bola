
use std::rc::Rc;
use std::collections::HashMap;

use crate::data::*;

pub fn execute( prog : &str, env : &mut Env ) -> Result<(), MachineError> {

    let mut ip : usize = 0;

    let mut current_word : Rc<Word> = env.get_dict("main").unwrap();

    'main_loop : loop {
        match &*current_word {
            Word::Func(words) if ip == words.len() => {
                // End of word
                env.pop_def_level();
                match env.pop_func_restore_point() {
                    Some((word, new_ip)) => { 
                        current_word = word;
                        ip = new_ip;
                    },
                    None => { break 'main_loop; },
                }
            },
            Word::Func(words) if ip > words.len() => {
                todo!(); // error condition
            },
            Word::Func(words) => {
                env.push_func_restore_point(&current_word, ip + 1);
                env.new_def_level();

                current_word = words[ip].clone();
                ip = 0;
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