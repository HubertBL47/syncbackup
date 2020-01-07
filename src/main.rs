extern crate syncbackup;

use syncbackup::args::Args;
use syncbackup::errorcode::ErrorCode;
use syncbackup::sync::make_sync;

fn main() {
    let help_message = "\n\nFor more information use \"--help\"\n";
    let args = Args::new().unwrap_or_else( |err| {
        eprintln!("{}{}", err.err_message(), help_message);
        std::process::exit(*err.err_code() as i32);
    });

    make_sync(args).unwrap_or_else( |err| {
        eprintln!("{}{}", err.err_message(), help_message);
        std::process::exit(*err.err_code() as i32);
    });

    std::process::exit(ErrorCode::NoError as i32);
}