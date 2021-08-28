fn main() {
    use git2::Repository;
    use std::env;
    use std::path::PathBuf;

    let _ = Repository::clone("https://github.com/libsdl-org/SDL", "SDL2");
    let _ = Repository::clone("https://github.com/libsdl-org/SDL_net", "SDL2_net");

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found"));

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_net");
    println!(
        "cargo:rustc-link-search={}",
        root.join("SDL2/build/.libs").as_path().to_string_lossy()
    );
    println!(
        "cargo:rustc-link-search={}",
        root.join("SDL2_net/.libs").as_path().to_string_lossy()
    );
    println!(
        "cargo:rustc-link-search={}",
        root.join("lib").as_path().to_string_lossy()
    );
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-ISDL2/include")
        .allowlist_function("SDLNet_.*")
        .allowlist_type("SDLNet_.*")
        .allowlist_var("SDLNet_.*")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .raw_line("//! Rust FFI to `SDL_net.h`")
        .raw_line("")
        .raw_line(r"#![allow(warnings)]")
        .generate()
        .expect("bindgen builder was invalid");

    bindings
        .write_to_file(root.join("src/bind.rs"))
        .expect("`src` directory not found");
}
