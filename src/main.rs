mod cargo;

use cargo::RustupChecker;
use cargo::CargoWrapper;

use std::process::exit;

const RUSTUP_CHK_ERR: i32 = 1;
const CARGO_CONFIG_ERR: i32 = 2;
const CARGO_RUN_ERR: i32 = 3;

fn print_and_exit<T: std::fmt::Display>(err: T, exit_code: i32) -> ! {
    eprintln!("{}", err);

    exit(exit_code);
}

fn main() {
    if let Err(e) = RustupChecker::check_required_components() {
        print_and_exit(e, RUSTUP_CHK_ERR);
    }

    if let Err(e) = CargoWrapper::config() {
        print_and_exit(e, CARGO_CONFIG_ERR);
    }

    if let Err(e) = CargoWrapper::run() {
        print_and_exit(e, CARGO_RUN_ERR);
    }
}
