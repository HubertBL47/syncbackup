use super::args;
use super::errorcode::*;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

pub fn make_sync(args: args::Args) -> Result<(), Error>{

    // let metadata = fs::metadata(args.path1()).unwrap();
    // let first_component = File::new(args.path1(), metadata);

    // let metadata = fs::metadata(args.path1()).unwrap();
    // let second_component = File::new(args.path2(), metadata);
    // //let (path1, path2) = get_metadata(&args)?;

    // // copying file
    // first_component.copy_to(&second_component)?;

    copy_dir(&args.path1(), &args.path2())?;
    // for i in dir {
    //     let file = i.unwrap();
    //     println!("{}", file.path().display());
    //     if file.file_type().unwrap().is_dir(){
    //         println!{"IT WORK !!!!!!!"};
    //     }
    // };
    return Ok(());
}

fn copy_dir(dir_to_copy: &PathBuf, location: &PathBuf) -> Result<(), Error> {
    let dir = fs::read_dir(dir_to_copy).unwrap(); // il faut gerer ici si le dir n'existe pas

    if dir_to_copy == location{return Ok(());}
    // iterate trough the directorie
    for i in dir{

        // get evry file one by one
        let file = match i{
            Ok(f) => f,
            Err(e) => return Err(Error::new(ErrorCode::CopyError, format!("could not iterate due to {:?}",e)))
        };

        let file_path = file.path();
        
        // if it is a dir, iterate through this directorie
        if file.file_type().unwrap().is_dir(){
            let new_location = copy_and_push_path(location, &file.file_name());
            copy_dir(&file_path, &new_location)?;
        } else {
            copy_file(&file_path, location)?;
        }
    }

    Ok(())
}

fn copy_file(file_to_be_copy: &PathBuf, location: &PathBuf) -> Result<(), Error>{
    let file_name = file_to_be_copy.file_name().unwrap();
    if location.contain(file_name){

        let location_to_copy = copy_and_push_path(location, file_name);
        
        match fs::copy(&file_to_be_copy, location_to_copy){
            Ok(byte) => println!("copying file {:?}, {} bytes copied",file_name, byte),
            Err(e) => return Err(Error::new(ErrorCode::CopyError, format!("Copy failed dues to: {}", e))),
        };
    }


    Ok(())
}

fn copy_and_push_path(path: &PathBuf, to_be_add: &OsStr) -> PathBuf{
    // return a copy of a path with a addition
    let mut new_path = PathBuf::new();
        new_path.push(path);
        new_path.push(to_be_add);
    return new_path;
}

trait Contain{
    fn contain(&self, file: &OsStr) -> bool;
}

impl Contain for PathBuf{
    fn contain(&self, file: &OsStr) -> bool{
        for i in self.read_dir().unwrap(){
            if i.unwrap().file_name() == file {
                return true;
            }
        }
        return false;
    }
}