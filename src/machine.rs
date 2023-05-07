
use std::rc::Rc;

use crate::data::*;

pub fn execute( prog : String, env : &mut Env ) -> Result<(), MachineError> {

    let mut ip : usize = 0;

    // TODO:  Something like foreach parser try to create a word and then execute it
    // then keep going until the prog is empty
    // Might be able to just leave this function alone and create another one that calls it
    // or some other abstraction.

    let mut current_word : Rc<Word> = env.lookup_word("main").unwrap();

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
                match env.pop_func_restore_point() {
                    Some((word, new_ip)) => { 
                        current_word = word;
                        ip = new_ip;
                    },
                    None => { break 'main_loop; },
                }
            },
            Word::Exit => {
                break 'main_loop;
            }
        }
    }

    Ok(())
}