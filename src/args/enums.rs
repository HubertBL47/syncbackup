use strum_macros::EnumIter;

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
    Help = 0,
    CopyTo = 0b1,
    CopyFrom = 0b10,
}
