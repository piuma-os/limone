use std::{fmt::Write as _, fs, path::Path};

use anyhow::Result;

const TARGETS: [&str; 4] = [
    "x86_64-unknown-none",
    "aarch64-unknown-none",
    "riscv64-unknown-none",
    "loongarch64-unknown-none",
];

fn delete_if_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    if fs::metadata(path.as_ref()).is_ok() {
        if path.as_ref().is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    delete_if_exists("../src/sys")?;
    delete_if_exists("../src/sys.rs")?;

    fs::create_dir_all("../src/sys")?;

    let mut sys_module = String::new();

    for target in TARGETS {
        let target_arch = target
            .strip_suffix("-unknown-none")
            .expect("Internal error");

        let bindings = bindgen::builder()
            .use_core()
            .header("../limine/limine.h")
            .clang_arg(format!("--target={}", target))
            .generate()?;

        bindings.write_to_file(format!("../src/sys/{target_arch}.rs"))?;

        writeln!(&mut sys_module, "#[cfg(target_arch = \"{target_arch}\")]")?;
        writeln!(&mut sys_module, "mod {target_arch};")?;
        writeln!(&mut sys_module, "#[cfg(target_arch = \"{target_arch}\")]")?;
        writeln!(&mut sys_module, "pub use {target_arch}::*;")?;
    }

    fs::write("../src/sys.rs", sys_module)?;

    Ok(())
}
