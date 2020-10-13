mod cargo;

use cargo::RustupChecker;
use cargo::CargoWrapper;

fn main() {
    RustupChecker::check_required_components();

    CargoWrapper::config();
}
