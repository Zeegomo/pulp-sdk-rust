use std::process::Command;
use std::path::Path;
const WRAPPER_LIB_DIR: &str = "dummy/BUILD/PULP/GCC_RISCV/dummy";

fn main() {
    Command::new("make").args(&["clean", "all"]).current_dir("dummy").status().unwrap();
    Command::new("ar").args(&["crus", "libwrapper.a", "dummy.o"])
                      .current_dir(&Path::new(WRAPPER_LIB_DIR))
                      .status().unwrap();
    println!("cargo:rustc-link-search=native={}", WRAPPER_LIB_DIR);
    println!("cargo:rustc-link-lib=static=wrapper");
}