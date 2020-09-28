fn main() {
    // Cause compilation error when both almagation and link-system is set
    #[cfg(all(feature = "link-amalg", feature = "link-system"))]
    compile_error!(r#"You can only use either "link-amalg" or "system" feature, not both."#);

    // Cause compilation error when both almagation and system is set
    #[cfg(all(feature = "link-amalg", feature = "system"))]
    compile_error!(r#"You can only use either "link-amalg" or "system" feature, not both."#);

    // Link to the system libjanet if the system feature is included
    #[cfg(feature = "link-system")]
    println!("cargo:rustc-link-lib=janet");

    // Make cargo rerun if header changes
    println!("cargo:rerun-if-changed=csrc/janet.h");

    let whitelist_regex = "^([jJ]anet|ANET.*)|(.*jmp)";

    #[cfg(any(feature = "system", feature = "link-system"))]
    let header = std::env::var("JANET_HEADERPATH")
        .expect("JANET_HEADERPATH not found, maybe you don't have Janet installed.")
        + "/janet.h";

    #[cfg(not(any(feature = "system", feature = "link-system")))]
    let header = "./csrc/janet.h";

    let bindings = bindgen::Builder::default()
        .header(header)
        .whitelist_type(whitelist_regex)
        .whitelist_function(whitelist_regex)
        .whitelist_var(whitelist_regex)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    cc::Build::new()
        .file("csrc/janet.c")
        .include("csrc")
        .compile("janet");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
