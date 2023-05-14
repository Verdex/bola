
use crate::data::*;

pub fn parse_def_word() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let start_index = env.pop_data_as("parse_def_word::start_index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        let input = env.pop_data_as("parse_def_word::input".to_owned(), pattern!(IlData::String(x) => x))?;

        let mut target = input[start_index..].chars();

        let mut i : usize = 0;

        let target = match target.next() {
            Some(':') => {
                target.take_while(|c| *c != ';').collect::<String>();
            },
            _ => {
                env.push_data(IlData::String(input));
                env.push_data(IlData::Usize(start_index));
                env.push_data(IlData::Symbol(ERROR_SYM.to_owned()));
                return Ok(());
            },
        };


        // TODO:  Not sure if we can tell if the ; was there or not
        // TODO:  you can extend i by the length of the target string + 1

        Ok(())
    }

    Word::Il(vec![Il::Instr { name: "parse_def_word".to_owned(), f : word }])
}

pub fn parse_anon_word() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let start_index = env.pop_data_as("parse_anon_word::start_index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        let input = env.pop_data_as("parse_anon_word::input".to_owned(), pattern!(IlData::String(x) => x))?;

        let mut target = input[start_index..].chars();

        let target = match target.next() {
            Some('[') => {
                try_take_while(&mut target, |c| c != ']')
            },
            z => {
                println!("blarg {:?}", z);
                env.push_data(IlData::String(input));
                env.push_data(IlData::Usize(start_index));
                env.push_data(IlData::Symbol(ERROR_SYM.to_owned()));
                return Ok(());
            },
        };

        if let Some(target) = target {
            let target_len = target.len();
            // TODO need to grab first symbol for name
            let main = env.with_clean_data_stack();

            crate::machine::quote(target, env)?;
            let quoted_word = env.with_clean_data_stack();
            env.restore_data_stack(main);

            if quoted_word.len() < 2 {
                return Err(MachineError::FatalParse("parse_anon_word::running quote resulted in invalid data stack".to_owned()));
            }

            let end = quoted_word.len() - 2;
            let anon_word_body = &quoted_word[0..end];

            println!("blarg ::: {:?}\n\n", anon_word_body);

            env.push_data(IlData::String(input));
            env.push_data(IlData::Usize(start_index + target_len + 1));
            env.push_data(IlData::Symbol(OK_SYM.to_owned())); // TODO should be RESULT with the anon word
            // TODO need to check the final length
        }
        else {
            // Note:  There was no end bracket
            let failure_index = input.len() - 1;
            env.push_data(IlData::String(input));
            env.push_data(IlData::Usize(failure_index));
            env.push_data(IlData::Symbol(FATAL_SYM.to_owned()));
        }

        Ok(())
    }

    Word::Il(vec![Il::Instr { name: "parse_anon_word".to_owned(), f : word }])
}

fn try_take_while<'a>( input : &mut std::str::Chars<'a>, pred : fn (char) -> bool) -> Option<String> {
    let mut cs = vec![];
    loop {
        match input.next() {
            Some(c) if !pred(c) => { break; },
            Some(c) => { cs.push(c); },
            None => { return None; },
        }
    }
    Some(cs.into_iter().collect::<String>())
}