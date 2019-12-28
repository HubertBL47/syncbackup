use strum_macros::EnumIter; // etc.

pub enum ResultArgument{
    Ok(String),
    NoArgs,
    ErrOption(String),
}


pub enum ArgsType{
    Argument(String),
    Option(String),
    None,
}

// possible type of option passed
// these option will be in a int where each bit indicate 1 option
#[derive(Debug, EnumIter)]
pub enum PossiblesOptions{
    CopyTo = 0b1,
    CopyFrom = 0b10,
    Help = 0b100,

}
