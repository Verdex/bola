
// parser : needs env and chars returns word or Error/Fatal while also leaving the definition in the dictionary
// parser : (string usize -- (Ok|Result|Error|Fatal) [word params*] string usize)

use crate::data::*;

pub fn parse_whitespace() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let input = env.pop_data_as(|d| match d { IlData::String(s) => Ok(s), _ => Err(d), })?;
        let start_index = env.pop_data_as(|d| match d { IlData::Usize(x) => Ok(x), _ => Err(d), })?;

        let mut target = input[start_index..].chars();

        let mut i : usize = 0;

        loop {
            match target.next() {
                Some(c) if c.is_whitespace() => { i += 1; },
                _ => { break; },
            }
        }

        env.push_data(IlData::Usize(start_index + i));
        env.push_data(IlData::String(input));

        Ok(())
    }

    Word::Il(vec![Il::Instr { name: "parse_whitespace".to_owned(), f : word }])
}
