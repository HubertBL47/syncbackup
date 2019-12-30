extern crate syncbackup;

use syncbackup::args::Args;
use syncbackup::errorcode::ErrorCode;
use syncbackup::sync::make_sync;

fn main() {
    let args = Args::new().unwrap_or_else( |err| {
        eprintln!("{}\n\nFor more information use \"--help\"\n", &err);
        std::process::exit(ErrorCode::Args as i32);
    });

    make_sync(args).unwrap_or_else( |err| {
        eprintln!("{}\n\nFor more information use \"--help\"\n", &err);
        std::process::exit(ErrorCode::Sync as i32);
    });

    std::process::exit(ErrorCode::NoError as i32);
}