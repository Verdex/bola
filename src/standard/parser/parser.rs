
// parser : needs env and chars returns word or Error/Fatal while also leaving the definition in the dictionary
// parser : (usize string -- (Ok|Result|Error|Fatal) usize string [word params*] )

use crate::data::*;

pub fn parse_whitespace() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let start_index = env.pop_data_as("parse_whitespace::start_index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        let input = env.pop_data_as("parse_whitespace::input".to_owned(), pattern!(IlData::String(x) => x))?;

        let mut target = input[start_index..].chars();

        let mut i : usize = 0;

        loop {
            match target.next() {
                Some(c) if c.is_whitespace() => { i += 1; },
                _ => { break; },
            }
        }

        env.push_data(IlData::String(input));
        env.push_data(IlData::Usize(start_index + i));
        if i == 0 {
            env.push_data(IlData::Symbol(ERROR_SYM.to_owned()));
        }
        else {
            env.push_data(IlData::Symbol(OK_SYM.to_owned()));
        }

        Ok(())
    }

    Word::Il(vec![Il::Instr { name: "parse_whitespace".to_owned(), f : word }])
}

pub fn parse_word_symbol() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let start_index = env.pop_data_as("parse_word_symbol::start_index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        let input = env.pop_data_as("parse_word_symbol::input".to_owned(), pattern!(IlData::String(x) => x))?;

        let mut target = input[start_index..].chars();

        let mut i : usize = 0;
        let mut word_symbol = vec![];

        match target.next() {
            Some(c) if c.is_alphabetic() || c == '_' => { i += 1; word_symbol.push(c); },
            _ => { 
                env.push_data(IlData::String(input));
                env.push_data(IlData::Usize(start_index));
                env.push_data(IlData::Symbol(ERROR_SYM.to_owned()));
                return Ok(());
            },
        }

        loop {
            match target.next() {
                Some(c) if c.is_alphanumeric() || c == '_' => { i += 1; word_symbol.push(c); },
                _ => { break; },
            }
        }

        let word_symbol = word_symbol.into_iter().collect::<String>();

        let target = env.lookup_word(&word_symbol)?;
        env.push_data(IlData::Word(target));

        env.push_data(IlData::String(input));
        env.push_data(IlData::Usize(start_index + i));
        env.push_data(IlData::Symbol(RESULT_SYM.to_owned()));

        Ok(())
    }

    Word::Il(vec![Il::Instr { name: "parse_word_symbol".to_owned(), f : word }])
}

pub fn parse_float() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let start_index = env.pop_data_as("parse_float::start_index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        let input = env.pop_data_as("parse_float::input".to_owned(), pattern!(IlData::String(x) => x))?;

        let mut target = input[start_index..].chars();

        let mut i : usize = 0;
        let mut number = vec![];

        match target.next() {
            Some(c) if c.is_digit(10) => { i += 1; number.push(c); },
            Some('-') => match target.next() {
                Some(c) if c.is_digit(10) => { i += 2; number.push('-'); number.push(c); },
                _ => {
                    env.push_data(IlData::String(input));
                    env.push_data(IlData::Usize(start_index));
                    env.push_data(IlData::Symbol(ERROR_SYM.to_owned()));
                    return Ok(());
                },
            },
            _ => { 
                env.push_data(IlData::String(input));
                env.push_data(IlData::Usize(start_index));
                env.push_data(IlData::Symbol(ERROR_SYM.to_owned()));
                return Ok(());
            },
        }

        loop {
            match target.next() {
                Some(c) if c.is_digit(10) 
                        || c == 'e' 
                        || c == 'E'
                        || c == '+'
                        || c == '-'
                        || c == '.' => { i += 1; number.push(c); },
                _ => { break; },
            }
        }

        match number.into_iter().collect::<String>().parse::<f64>() {
            Ok(f) => {
                let target = std::rc::Rc::new(crate::standard::word::push_float(f));
                env.push_data(IlData::Word(target));

                env.push_data(IlData::String(input));
                env.push_data(IlData::Usize(start_index + i));
                env.push_data(IlData::Symbol(RESULT_SYM.to_owned()));
            },
            Err(_) => {
                env.push_data(IlData::String(input));
                env.push_data(IlData::Usize(start_index));
                env.push_data(IlData::Symbol(FATAL_SYM.to_owned()));
            },
        }

        Ok(())
    }

    Word::Il(vec![Il::Instr { name: "parse_float".to_owned(), f : word }])
}
