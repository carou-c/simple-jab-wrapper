use std::env;
use std::path::PathBuf;

const JAVA_HOME: &str = "/usr/lib/jvm/java-8-openjdk";

fn main() {
    // 1. Compile the C code
    cc::Build::new()
        .prefer_clang_cl_over_msvc(true)
        .file("native/AccessBridgeCalls.c")
        .include(format!("{}/include", JAVA_HOME))
        .include(format!("{}/include/linux", JAVA_HOME))
        .include("native") // for headers
        .compile("accessbridgecalls"); // produces libaccessbridgecalls.a

    // 2. Tell cargo to rerun if headers change
    println!("cargo:rerun-if-changed=native/AccessBridgeDebug.h");
    println!("cargo:rerun-if-changed=native/AccessBridgePackages.h");
    println!("cargo:rerun-if-changed=native/AccessBridgeCallbacks.h");
    println!("cargo:rerun-if-changed=native/AccessBridgeCalls.h");
    println!("cargo:rerun-if-changed=native/wrapper.h");
    println!("cargo:rerun-if-changed=native/AccessBridgeCalls.c");

    // 3. Generate bindings
    let bindings = bindgen::Builder::default()
        .header("native/wrapper.h")
        // Optional but recommended:
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // If your headers include others:
        .clang_arg("-Inative")
        .clang_arg(format!("-I{}/include", JAVA_HOME))
        .clang_arg(format!("-I{}/include/linux", JAVA_HOME))
        .generate()
        .expect("Unable to generate bindings");

    // 4. Write bindings to OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
