use std::collections::HashMap;

fn should_skip_guest_build() -> bool {
    match std::env::var("SKIP_GUEST_BUILD")
        .as_ref()
        .map(|arg0: &String| String::as_str(arg0))
    {
        Ok("1") | Ok("true") | Ok("risc0") => true,
        Ok("0") | Ok("false") | Ok(_) | Err(_) => false,
    }
}

fn main() {
    println!("cargo::rerun-if-env-changed=SKIP_GUEST_BUILD");
    println!("cargo::rerun-if-env-changed=OUT_DIR");
    if should_skip_guest_build() {
        println!("Skipping guest build for CI run");
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let out_dir = std::path::Path::new(&out_dir);
        let methods_path = out_dir.join("methods.rs");

        let elf = r#"
            pub const ROLLUP_PATH: &str = "";
            pub const MOCK_DA_PATH: &str = "";
        "#;

        std::fs::write(methods_path, elf).expect("Failed to write mock rollup elf");
    } else {
        let guest_pkg_to_options = get_guest_options();
        risc0_build::embed_method_metadata_with_options(guest_pkg_to_options);
    }
}

fn get_guest_options() -> HashMap<&'static str, risc0_build::GuestOptions> {
    HashMap::new()
}
