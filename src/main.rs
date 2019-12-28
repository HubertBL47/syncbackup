extern crate syncbackup;

use syncbackup::args::Args;
use syncbackup::errorcode::ErrorCode;

fn main() {
    let args = Args::new().unwrap_or_else( |err| {
        eprintln!("{}\n\nFor more information use \"--help\"\n", &err);
        std::process::exit(ErrorCode::Args as i32);
    });

    std::process::exit(ErrorCode::NoError as i32);
}