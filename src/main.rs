extern crate syncbackup;

use syncbackup::args::Args;
use syncbackup::args::enums::PossiblesOptions;

enum ErrorCode {
    NoError = 0,
    Args,
    App,
}

fn main() {
    let args = Args::new().unwrap_or_else( |err| {
        eprintln!("{}\n\nFor more information use \"--help\"\n", &err);
        std::process::exit(ErrorCode::Args as i32);
    });

    if args.options() & PossiblesOptions::Help as u32 == PossiblesOptions::Help as u32{
        println!("Help me");
    }


    std::process::exit(ErrorCode::NoError as i32);
}