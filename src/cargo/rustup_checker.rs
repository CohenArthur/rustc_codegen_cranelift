//! Install the required rustup components or check for their existence

use std::process::Command;

const NB_COMPONENTS: usize = 3;
const COMPONENTS: [&str; NB_COMPONENTS] = ["rust-src", "rustc-dev", "llvm-tools-preview"];

pub struct RustupChecker;

impl RustupChecker {
    fn install_required_components() -> Result<(), std::io::Error> {
        let mut rustup = Command::new("rustup");

        rustup.arg("component")
            .arg("add")
            .args(&COMPONENTS);

        // FIXME: Don't expect
        rustup.status().expect("Couldn't install required components");

        Ok(())
    }

    // Install the required rustup components if needed
    pub fn check_required_components() -> Result<(), std::io::Error> {
        let mut cmd = Command::new("rustup");

        // List the installed rustup components
        cmd.arg("component")
            .arg("list")
            .arg("--installed");

        // FIXME: No unwrap, ugly, no expect
        let output = String::from_utf8(cmd
            .output()
            .expect("Failed to execute command \"rustup\"")
            .stdout).unwrap();

        let mut components_found = 0;

        for line in output.lines() {
            for comp in &COMPONENTS {
                if line.contains(comp) {
                    components_found += 1;
                    break;
                }
            }
        }

        if components_found != NB_COMPONENTS {
            RustupChecker::install_required_components()
        } else {
            Ok(())
        }
    }
}
