//! Wraps around cargo to emulate the behavior of cargo using cranelift

const HOST_PREFIX: &str = "host: ";

use std::process::Command;

pub struct CargoWrapper;

impl CargoWrapper {
    fn uname() -> Result<String, std::io::Error> {
        let mut uname = Command::new("uname");

        let output = uname.output()?;

        Ok(String::from_utf8(output.stdout).unwrap())
    }

    fn host_triple() -> Result<String, std::io::Error> {
        let rustc_out = Command::new("rustc").arg("-vV").output()?.stdout;
        let rustc_out_str = String::from_utf8(rustc_out).unwrap();
        let mut target_line = "";

        for line in rustc_out_str.lines() {
            if line.starts_with(HOST_PREFIX) {
                target_line = line;
            }
        }

        Ok(String::from(target_line.strip_prefix(HOST_PREFIX).unwrap()))
    }

    /// Configure the environment to use cranelift instead of LLVM
    pub fn config() -> Result<(), std::io::Error> {
        let uname = CargoWrapper::uname()?;
        let host_triple = CargoWrapper::host_triple()?;

        let dylib_ext = match uname.as_str() {
            "Linux" => "so",
            "Darwin" => "dylib",
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported OS")),
        };

        // FIXME: Add option to use different targets as in config.sh

        // FIXME: Support $CHANNEL

        std::env::set_var("RUSTC", "cg_clif");

        // FIXME: Use dylib_ext instead of .so
        std::env::set_var("RUSTDOCFLAGS",
        r#"-Ztrim-diagnostic-paths=no"
        "-Cpanic=abort"
        "-Zpanic-abort-tests"
        "-Zcodegen-backend=librustc_codegen_cranelift.so"
        "--sysroot cg_clif_build_sysroot"#);

        // FIXME: Add line regarding atomic shim in Darwin for RUSTFLAGS

        // FIXME: Incomplete
        let ld_library_path = format!("{}/lib", host_triple);

        std::env::set_var("LD_LIBRARY_PATH", &ld_library_path);
        std::env::set_var("DYLD_LIBRARY_PATH", &ld_library_path);

        std::env::set_var("CG_CLIF_DISPLAY_CG_TIME", "1");

        Ok(())
    }
}
