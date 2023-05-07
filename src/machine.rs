
use std::rc::Rc;

use crate::data::*;

// TODO prog should be Box<str> ?
pub fn execute(prog : String, env : &mut Env) -> Result<(), MachineError> {

    // TODO:  Something like foreach parser try to create a word and then execute it
    // then keep going until the prog is empty
    // Might be able to just leave this function alone and create another one that calls it
    // or some other abstraction.
    let mut index = 0;
    let mut prog_len = prog.len();

    env.push_data(IlData::String(prog));
    env.push_data(IlData::Usize(0));
    while index < prog_len {

        let parsers = env.parsers.clone().into_iter();
        'parsing : for parser in parsers {
            execute_word(parser.clone(), env)?;

            let parse_result = env.pop_data_as("execute::parse_result".to_owned(), pattern!(IlData::Symbol(x) => x))?;
            match &parse_result[..] { 
                OK_SYM => { break 'parsing; },
                RESULT_SYM => { 
                    let index = env.pop_data_as("execute::result::index".to_owned(), pattern!(x @ IlData::Usize(_) => x))?;
                    let prog = env.pop_data_as("execute::result::prog".to_owned(), pattern!(x @ IlData::String(_) => x))?;

                    let word = env.pop_data_as("execute::result::word".to_owned(), pattern!(IlData::Word(x) => x))?;
                    execute_word(word, env)?;

                    env.push_data(prog);
                    env.push_data(index);
                },
                ERROR_SYM => { 
                    // NOTE:  Make sure that the index is reset for the next parser.
                    let _index = env.pop_data_as("execute::error::_index".to_owned(), pattern!(IlData::Usize(_) => ()))?;
                    env.push_data(IlData::Usize(index));
                },
                FATAL_SYM => return Err(MachineError::FatalParse),
                _ => unreachable!(),
            }
        }

        index = env.pop_data_as("execute::end_while::index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        env.push_data(IlData::Usize(index));
    }

    Ok(())
}

fn execute_word( mut current_word : Rc<Word>, env : &mut Env ) -> Result<(), MachineError> {

    let mut ip : usize = 0;

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