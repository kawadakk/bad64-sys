use std::ffi::OsStr;

fn main() {
    let ignorelist: Vec<&OsStr> = ["test.c", "gofer.c", "format.c", "encodings_fmt.c"]
        .iter()
        .map(OsStr::new)
        .collect();

    let dotc_files = glob::glob("arch-arm64/disassembler/*.c")
        .expect("Failed to read glob pattern")
        .map(|x| x.unwrap())
        .filter(|x| !ignorelist.as_slice().contains(&x.file_name().unwrap()));

    // Compile the library
    cc::Build::new()
        .files(dotc_files)
        .include("arch-arm64/disassembler")
        .compile("arm64decode");

    // Generate the bindings

    // Tell cargo to tell rustc to link the compiled disassembler
    println!("cargo:rustc-link-lib=arm64decode");
}
