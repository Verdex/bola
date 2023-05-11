
use crate::data::*;

pub fn parse_def_word() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let start_index = env.pop_data_as("parse_word_symbol::start_index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        let input = env.pop_data_as("parse_word_symbol::start_index".to_owned(), pattern!(IlData::String(x) => x))?;
        Ok(())
    }
    Word::Il(vec![])
}