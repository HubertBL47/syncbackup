use super::args;
use std::fs;

pub fn make_sync(args: args::Args) -> Result<(), String>{
    println!("path 1 : {:?}",args.path1());
    println!("path 2 : {:?}",args.path2());



    let file = fs::metadata(args.path1()).unwrap();
    println!("{:?}", file.file_type());
    println!("len {:?}", file.len());

    let dir = fs::metadata(args.path2()).unwrap();
    println!("{:?}", dir.is_dir());
    println!("len {:?}", dir.len());

    if dir.modified().unwrap() > file.modified().unwrap(){
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

fn copy_file_to(){}