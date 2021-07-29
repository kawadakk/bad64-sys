use std::{env, fs, path::PathBuf};

fn main() {
    let genbinding_pkg_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let pkg_path = genbinding_pkg_path.parent().unwrap();
    env::set_current_dir(pkg_path).unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // common derives
        .derive_debug(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_partialeq(true)
        .use_core()
        .rustified_enum("OperandClass")
        .rustified_enum("ShiftType")
        .rustified_enum("ArrangementSpec")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the /binding/bindings.rs file.
    fs::create_dir_all("./binding").unwrap();
    bindings
        .write_to_file("./binding/bindings.rs")
        .expect("Couldn't write bindings!");
}
