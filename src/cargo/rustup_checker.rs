//! Install the required rustup components or check for their existence

use std::process::{Command, ExitStatus};

const NB_COMPONENTS: usize = 3;
const COMPONENTS: [&str; NB_COMPONENTS] = ["rust-src", "rustc-dev", "llvm-tools-preview"];

pub struct RustupChecker;

impl RustupChecker {
    fn install_required_components() -> Result<ExitStatus, std::io::Error> {
        let mut rustup = Command::new("rustup");

        // Add the required rustup components for cranelift
        rustup.arg("component")
            .arg("add")
            .args(&COMPONENTS);

        rustup.status()
    }

    fn get_installed_components() -> Result<String, std::io::Error> {
        let mut rustup = Command::new("rustup");

        // List the installed rustup components
        rustup.arg("component")
            .arg("list")
            .arg("--installed");

        let output = rustup.output()?.stdout;

        Ok(String::from_utf8(output).unwrap())
    }

    // Install the required rustup components if needed
    pub fn check_required_components() -> Result<(), std::io::Error> {
        let installed_components = RustupChecker::get_installed_components().unwrap();

        let mut components_found = 0;

        // FIXME: Ugly?
        for line in installed_components.lines() {
            for comp in &COMPONENTS {
                if line.contains(comp) {
                    components_found += 1;
                    break;
                }
            }
        }

        if components_found != NB_COMPONENTS {
            RustupChecker::install_required_components()?;
        }

        Ok(())
    }
}
