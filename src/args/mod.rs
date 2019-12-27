pub mod enums;

use std::env; // for args
use enums::ResultArgument;
pub use enums::PossiblesOptions;
use enums::ArgsType;

pub struct Args{
    command: String,
    path_to_copy: String,
    location: String,
    options: u32,
}


impl Args {
    pub fn new() -> Result<Args, String>{
        // constuctor 
        let mut args = env::args();
        let command: String = args.next().unwrap();
        let mut options: u32 = 0;

        let path_to_copy: String = match Args::get_next_argument(&mut args, &mut options){
            ResultArgument::Ok(v) => v,
            ResultArgument::NoArgs => return Err(String::from("Missing argument : File or directory not specified")),
            ResultArgument::ErrOption(s) => return Err(s),
        };
        let location: String = match Args::get_next_argument(&mut args, &mut options){
            ResultArgument::Ok(v) => v,
            ResultArgument::NoArgs => return Err(String::from("Missing argument : location not specified")),
            ResultArgument::ErrOption(s) => return Err(s),
        };
        
        loop{ // get all remaining options
            match Args::get_arg_value(args.next()) {
                ArgsType::Argument(s) => return Err(format!("Unknown arguments \"{}\"", s)),
                ArgsType::Option(s) => Args::add_option(s, &mut options)?,
                ArgsType::None => break,          
            };
        }

        println!("path to copy: {}", path_to_copy);
        println!("location: {}", path_to_copy);
        println!("the value of option is {}",options );

        return Ok(Args{
            command,
            path_to_copy,
            location,
            options,
        });
    }

    fn get_arg_value(arg: Option<String>) -> ArgsType{
        /* find the type of the next value passed in the command line
            * "--" indicate an option
        */
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

    fn add_option(new_option: String, all_options:&mut u32) -> Result<(), String>{
        // add the matching argument to all_options passed
        match new_option.as_ref(){
            "copyto" => *all_options |= PossiblesOptions::CopyTo as u32,
            "copyfrom" => *all_options |= PossiblesOptions::CopyFrom as u32,
            "help" => *all_options |= PossiblesOptions::Help as u32,
            _ => return Err(format!("Unknown option \"{}\"", new_option)),
        }
        return Ok(());
    }

    fn get_next_argument(args: &mut env::Args, options: &mut u32) -> ResultArgument{
        // read the next argument. if it is an option, add it to the options counter
        loop{ // get option until there is an argument
            match Args::get_arg_value(args.next()) {
                ArgsType::None => return ResultArgument::NoArgs,
                ArgsType::Option(v) => match Args::add_option(v, options){
                    Ok(()) => continue,
                    Err(s) => return ResultArgument::ErrOption(s),
                },
                ArgsType::Argument(s) => return ResultArgument::Ok(s),
            };
        }
    }

    pub fn options(&self)-> u32{
        return self.options;
    }

}