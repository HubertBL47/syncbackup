extern crate syncbackup;

use syncbackup::args::Args;

enum ErrorCode {
    Args,
    App,
}

fn main() {
    let args = Args::new().unwrap_or_else( |err| {
        eprintln!("{}",&err);
        std::process::exit(ErrorCode::Args as i32);
    });
}