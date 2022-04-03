use std::env;
use std::path::PathBuf;

const MAKE_CC_INVOCATION: &str = "riscv32-unknown-elf-gcc";
const REGEX_PATTERN: &str = ".*riscv32-unknown-elf-gcc *-c test.c -o *.*test.o (.*)";

fn extract_clang_args() -> Vec<String> {
    let make = std::process::Command::new("make")
        .args(["-C", "dummy", "dummy.c", "build", "--trace"])
        .output()
        .expect("Could not run make");
    println!("{}", String::from_utf8(make.stderr).unwrap());
    assert!(make.status.success(), "make failed");
    let output = String::from_utf8(make.stdout)
        .expect("invalid utf8")
        .lines()
        .filter(|s| s.contains(MAKE_CC_INVOCATION))
        .next()
        .unwrap()
        .to_owned();

    let args = regex::Regex::new(REGEX_PATTERN)
        .unwrap()
        .captures(&output)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .replace("-include ", "include"); // GCC to Clang compat
    let args = args.split_whitespace();
    let defines = args.clone().filter(|s| s.starts_with("-D"));
    let includes = args.clone().filter(|s| s.contains("-include"));
    let link_search = args.filter(|s| s.starts_with("-I"));
    includes
        .chain(defines)
        .chain(link_search)
        .map(String::from)
        .collect()
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-search=rtos/pmsis_bsp/include");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=rtos/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default().use_core().emit_builtins();

    if target != host {
        builder = builder.clang_args(["-target", &target]);
    }

    // The whole pulp sw stack is built upon makefiles and it's a bit of a nightmare
    // to track what options / configs are being used.
    // To get the list of libraries to include, header files and compiler options run make
    // on a dummy file and reflect thos in bindgen clang args
    let args = extract_clang_args();
    builder = builder.clang_args(&args);

    let bindings = builder
        .header("rtos/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
