use super::args;
use std::fs;

pub fn make_sync(args: args::Args) -> Result<(), String>{
    let (path1, path2) = get_metadata(&args)?;
    if path1.modified().unwrap() > path2.modified().unwrap(){
        println!("most recent is second file");
    } else {
        println!("most recent is first file");
    }

    let dir = fs::read_dir(args.path2()).unwrap();
    for i in dir {
        println!("{}", i.unwrap().path().display() )
    };
    return Ok(());
}

fn get_metadata(args: &args::Args) -> Result<(fs::Metadata, fs::Metadata), String>{
    // verified if path are good if not, give an Err
    let path1 = match fs::metadata(args.path1()){
        Ok(v) => v,
        Err(_) => return Err(format!("problem while opening \"{:?}\"", args.path1())),
    };
    let path2 = match fs::metadata(args.path2()){
        Ok(v) => v,
        Err(_) => return Err(format!("problem while opening \"{:?}\"", args.path1())),
    };

    return Ok((path1, path2));
}

fn copy_file_to(){

}