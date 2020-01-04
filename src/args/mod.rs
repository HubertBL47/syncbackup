pub mod enums;

use std::env; // for args
pub use enums::PossiblesOptions;
use enums::ArgsType;
use super::help;
use std::path::PathBuf;
use super::errorcode::*;

pub struct Args{
    command: String,
    path1: PathBuf,
    path2: PathBuf,
    options: u32,
}


impl Args {
    pub fn new() -> Result<Args, Error>{
        // constuctor 
        let mut args = env::args();
        let command: String = args.next().unwrap();
        let mut options: u32 = 0;

        // get the first argument passed
        let path1 = Args::get_next_argument(&mut args, &mut options)?;
        
        // get the second argument passed
        let path2 = Args::get_next_argument(&mut args, &mut options)?;

        loop{ // get all remaining options
            match Args::get_arg_value(args.next()) {
                ArgsType::Argument(s) => return Err(Error::new(ErrorCode::OptionError, format!("Unknown arguments \"{}\"", s))),
                ArgsType::Option(s) => Args::add_option(s, &mut options)?,
                ArgsType::None => break,          
            };
        }

        return Ok(Args{
            command,
            path1,
            path2,
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

    fn add_option(new_option: String, all_options:&mut u32) -> Result<(), Error>{
        // add the matching argument to all_options passed
        match new_option.as_ref(){
            "copyto" => *all_options |= PossiblesOptions::CopyTo as u32,
            "copyfrom" => *all_options |= PossiblesOptions::CopyFrom as u32,
            "help" => help::show_help(),
            _ => return Err(Error::new(ErrorCode::OptionError, format!("Unknown option \"{}\"", new_option))),
        };
        return Ok(());
    }

    fn get_next_argument(args: &mut env::Args, options: &mut u32) -> Result<PathBuf, Error>{
        // read the next argument. if it is an option, add it to the options counter
        loop{ // get option until there is an argument
            match Args::get_arg_value(args.next()) {
                ArgsType::Option(v) => Args::add_option(v, options)?,
                ArgsType::Argument(path) => return Ok(Args::verified_path(&path)?),
                ArgsType::None => return Err(Error::new(ErrorCode::MissingArgs, 
                    String::from("Missing argument : File or directory not specified"))
                    ),
            };
        }
    }

    fn verified_path(path_string: &String) ->Result<PathBuf, Error>{
        let mut path: PathBuf = PathBuf::new();
        path.push(path_string);
        return match path.exists(){
            true => Ok(path),
            false => Err(Error::new(ErrorCode::PathError, format!("Error, the path {:?} cannot be accessed", path_string))),
        };
    }

    pub fn options(&self)-> u32{
        return self.options;
    }

    pub fn command(&self) -> &String{
        return &self.command;
    }

    pub fn path1(&self) -> &PathBuf{
        return &self.path1;
    }

    pub fn path2(&self) -> &PathBuf{
        return &self.path2;
    }

}