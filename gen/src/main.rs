use anyhow::Result;
use heck::ToUpperCamelCase;
use regex::Regex;
use std::{fmt::Write as FmtWrite, fs, path::Path};
use tracing::debug;

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

    debug!("Deleting old bindings...");

    delete_if_exists("../src/sys")?;
    delete_if_exists("../src/sys.rs")?;
    delete_if_exists("../src/magic.rs")?;

    fs::create_dir_all("../src/sys")?;

    let mut sys_module = String::new();
    let mut magic_module = String::new();

    write_bindings(&mut sys_module)?;
    write_magic(&mut magic_module)?;

    fs::write("../src/sys.rs", sys_module)?;
    fs::write("../src/magic.rs", magic_module)?;

    debug!("Successfully generated all bindings!");

    Ok(())
}

fn write_bindings<W: FmtWrite>(w: &mut W) -> Result<()> {
    for target in TARGETS {
        let target_arch = target
            .strip_suffix("-unknown-none")
            .expect("Internal error");

        debug!("Generating bindings for {target_arch}...");

        let bindings = bindgen::builder()
            .use_core()
            .header("../limine/limine.h")
            .clang_arg(format!("--target={}", target))
            .generate()?;

        bindings.write_to_file(format!("../src/sys/{target_arch}.rs"))?;

        writeln!(w, "#[cfg(target_arch = \"{target_arch}\")]")?;
        writeln!(w, "mod {target_arch};")?;
        writeln!(w, "#[cfg(target_arch = \"{target_arch}\")]")?;
        writeln!(w, "pub use {target_arch}::*;")?;
    }

    Ok(())
}

fn write_magic<W: FmtWrite>(w: &mut W) -> Result<()> {
    let header = fs::read_to_string("../limine/limine.h")?;

    let magic_regex = Regex::new(
        r"#define\s*LIMINE_COMMON_MAGIC\s(?<first>0x[a-z0-9]*)\s*,\s*(?<second>0x[a-z0-9]*)",
    )?;

    let requests_regex = Regex::new(
        r"#\s*define\s*LIMINE_(?<name>.*)_REQUEST\s*\{\s*LIMINE_COMMON_MAGIC,\s*(?<first>0x[a-z0-9]*)\s*,\s*(?<second>0x[a-z0-9]*)\s*\}",
    )?;

    let captures = magic_regex.captures(&header).unwrap();
    let common_first = captures.name("first").unwrap().as_str();
    let common_second = captures.name("second").unwrap().as_str();

    for cap in requests_regex.captures_iter(&header) {
        let name = cap.name("name").unwrap().as_str();
        let first = cap.name("first").unwrap().as_str();
        let second = cap.name("second").unwrap().as_str();

        debug!("Found request {name} with magic {first}, {second}");

        writeln!(w,"pub const LIMINE_{name}_MAGIC: [u64; 4] = [{common_first}, {common_second}, {first}, {second}];")?;

        writeln!(
            w,
            r#"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Limine{struct_name}Magic([u64; 4]);

impl Limine{struct_name}Magic {{
    pub const fn new() -> Self {{
        Self(LIMINE_{name}_MAGIC)
    }}
}}

impl Default for Limine{struct_name}Magic {{
    fn default() -> Self {{
        Self::new()
    }}
}}
"#,
            struct_name = name.to_upper_camel_case()
        )?;
    }

    Ok(())
}
