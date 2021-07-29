use std::{cell::RefCell, env, fs, panic::AssertUnwindSafe, path::PathBuf, rc::Rc};

fn main() {
    let genbinding_pkg_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let pkg_path = genbinding_pkg_path.parent().unwrap();
    env::set_current_dir(pkg_path).unwrap();

    #[derive(Debug)]
    struct ItemNameCollector(AssertUnwindSafe<Rc<RefCell<Vec<String>>>>);

    impl bindgen::callbacks::ParseCallbacks for ItemNameCollector {
        fn item_name(&self, original_item_name: &str) -> Option<String> {
            self.0 .0.borrow_mut().push(original_item_name.to_owned());
            None
        }
    }

    // Find all items we want to exclude
    let excluded_items = Rc::new(RefCell::new(Vec::new()));
    bindgen::Builder::default()
        .header("wrapper_negative.h")
        .parse_callbacks(Box::new(ItemNameCollector(AssertUnwindSafe(Rc::clone(
            &excluded_items,
        )))))
        .generate()
        .expect("Unable to determine the exclusion list");

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
        .blocklist_item(excluded_items.borrow().join("|"))
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
