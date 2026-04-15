extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let features = [
        std::env::var("CARGO_FEATURE_ICU_SEGMENTER").is_ok(),
        std::env::var("CARGO_FEATURE_WINDOWS").is_ok(),
        std::env::var("CARGO_FEATURE_RUST_ICU_UBRK").is_ok(),
    ];

    let enabled_count = features.iter().filter(|&&x| x).count();

    if enabled_count > 1 {
        panic!(
            "Error: Features 'icu_segmenter', 'windows', and 'rust_icu_ubrk' are mutually exclusive. Choose only one.\n"
        );
    } else if enabled_count == 0 && std::env::var("CARGO_FEATURE_WINDOWS_ICU").is_err() {
        panic!("Error: At least one feature must be enabled.");
    }

    let bindings = bindgen::Builder::default()
        // I just imported it straight from where it was on my computer.
        // If we were building a real module we might not want to do this!
        // Also if you're trying this on macOS this header file may not be
        // in this particular location
        .header("emacs-module.h")
        // We want to specify our own emacs_module_init function, so we won't
        // generate one in Rust automatically
        .blocklist_function("emacs_module_init")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Generate the bindings
        .generate()
        // Explode if something goes wrong
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
