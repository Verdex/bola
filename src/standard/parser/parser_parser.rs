
use renounce::*;

use crate::data::*;

/*struct Def {
    name : String,
    def : Vec<Rule>,
}

enum Rule {
    Call { result : String, parser : String },
    ZeroOrMore { result : String, parser : String },
    Maybe { result : String, parser : String },
    FatalCall { result : String, parser : String },
    Where { ??? },
    FatalWhere { ??? },
    Select { ??? },
    End,
    FatalEnd,
}*/

// parser : needs env and chars returns word or Error/Fatal while also leaving the definition in the dictionary
// parser : (usize string -- (Ok|Result|Error|Fatal) usize string [word params*] )

pub fn parse_parser_definition() -> Word {
    fn word(env : &mut Env) -> Result<(), MachineError> {
        let start_index = env.pop_data_as("parse_word_symbol::start_index".to_owned(), pattern!(IlData::Usize(x) => x))?;
        let input = env.pop_data_as("parse_word_symbol::start_index".to_owned(), pattern!(IlData::String(x) => x))?;
        Ok(())
    }
    Word::Il(vec![])
}


/*

    parser : name 
    sym <= parser_name;
    sym <= * parser_name;
    sym <= ? parser_name;
    sym <= ! parser_name;
    ! where ???;
    where ???;
    ! end;
    end;
    select ???;

*/