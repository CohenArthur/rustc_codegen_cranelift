mod cargo;

use cargo::RustupChecker;

fn main() {
    RustupChecker::check_required_components();
}
