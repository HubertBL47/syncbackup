use std::env; // for args
use std::collections::HashSet;

enum ArgsType{
    Argument(String),
    Option(String),
    None,
}


pub struct Args{
    command: String,
    path_to_copy: String,
    location: String,
    options: HashSet<String>,
}


impl Args {
    pub fn new() -> Result<Args, String>{
        // constuctor 
        let mut args = env::args();
        let command: String = args.next().unwrap();
        let path_to_copy: String;
        let location: String;
        let mut options: HashSet<String> = HashSet::new();

        loop{ 
            match Args::get_arg_value(args.next()) {
                ArgsType::None => return Err(String::from("Missing argument : File or directory not specified")),
                ArgsType::Option(v) => options.insert(v),
                ArgsType::Argument(v) => {
                    path_to_copy = v;
                    break;},       
            };
        }
        
        loop{ 
            match Args::get_arg_value(args.next()) {
                ArgsType::None => return Err(String::from("Missing argument : location not specified")),
                ArgsType::Option(v) => options.insert(v),
                ArgsType::Argument(v) => {
                    location = v;
                    break;},            
            };
        }

        loop{ 
            match Args::get_arg_value(args.next()) {
                ArgsType::Argument(v) => return Err(format!("Unknown arguments \"{}\"", v)),
                ArgsType::Option(v) => options.insert(v),
                ArgsType::None => break,          
            };
        }

        return Ok(Args{
            command,
            path_to_copy,
            location,
            options,
        });
    }

    fn get_arg_value(arg: Option<String>) -> ArgsType{
        // find if it is an option or a normal argument
        return match arg{
            None => ArgsType::None,
            Some(v) => {
                if v.starts_with("--"){
                    ArgsType::Option(v.replace("--", ""))
                } else {
                    ArgsType::Argument(v)
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
