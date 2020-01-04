use super::args::enums::PossiblesOptions;
use strum::IntoEnumIterator;
use super::errorcode::ErrorCode;

pub fn show_help(){
    println!("\nSync 2 version of files or a directorys togheter. The latest version is copied into to oldest one\n");
    println!("usage: syncbackup <File or Directory> <File or Directory> [--options]\n");
    println!("Options :");
    for opt in PossiblesOptions::iter(){
        print!("  \"{:?}\": ", opt );
        match opt {
            PossiblesOptions::Help => help(),
            PossiblesOptions::CopyTo => copy_to(),
            PossiblesOptions::CopyFrom => copy_from(),
        };
        println!();
    }
    println!();
    std::process::exit(ErrorCode::NoError as i32);
}

fn help(){
    print!("Display help ")
}

fn copy_to(){
    print!("Force the copy of the given file or directory into the location")
}

fn copy_from(){
    print!("Force the copy of the given file or directory from the location")
}