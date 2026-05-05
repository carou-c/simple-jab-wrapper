use std::env;
use std::path::PathBuf;

const JAVA_HOME: &str = "/usr/lib/jvm/java-8-openjdk";
const MINGW_SYSROOT: &str = "/usr/i686-w64-mingw32";

fn main() {
    // 1. Compile the C code
    cc::Build::new()
        .cpp(true)
        .prefer_clang_cl_over_msvc(true)
        .warnings(false)
        .include(format!("{}/include", JAVA_HOME))
        .include(format!("{}/include/linux", JAVA_HOME))
        .include("native") // for headers
        .file("native/AccessBridgeCalls.c")
        .file("native/AccessBridgeDebug.cpp")
        .compile("accessbridge"); // produces libaccessbridge.a

    println!("cargo:rustc-link-search=native={}/lib", MINGW_SYSROOT);
    println!("cargo:rustc-link-lib=static=accessbridge");
    println!("cargo:rustc-link-lib=static=stdc++");

    // 2. Tell cargo to rerun if headers, C src, or build.rs change
    println!("cargo:rerun-if-changed=native/AccessBridgeDebug.h");
    println!("cargo:rerun-if-changed=native/AccessBridgePackages.h");
    println!("cargo:rerun-if-changed=native/AccessBridgeCallbacks.h");
    println!("cargo:rerun-if-changed=native/AccessBridgeCalls.h");
    println!("cargo:rerun-if-changed=native/wrapper.h");
    println!("cargo:rerun-if-changed=native/AccessBridgeDebug.cpp");
    println!("cargo:rerun-if-changed=native/AccessBridgeCalls.c");
    println!("cargo:rerun-if-changed=build.rs");

    // let target = std::env::var("TARGET").unwrap();
    // println!("{}", target);
    // unsafe {
    //     std::env::set_var("LIBCLANG_PATH", "/usr/i686-w64-mingw32/bin");
    // }

    // 3. Generate bindings
    let builder = bindgen::Builder::default()
        .header("native/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-Inative")
        .clang_arg(format!("-I{}/include", JAVA_HOME))
        .clang_arg(format!("-I{}/include/linux", JAVA_HOME))
        .clang_arg("-Wno-everything")
        .blocklist_type("_LONGDOUBLE")
        .allowlist_file("native/.*\\.h");
        // .clang_arg(format!("--target={}", target))
        // .clang_arg("--sysroot=/usr/i686-w64-mingw32")
        // .clang_arg("-I/usr/i686-w64-mingw32/include")
        // .clang_arg("-L/usr/i686-w64-mingw32/lib");

    let bindings = builder.generate().expect("Unable to generate bindings");

    // 4. Write bindings to OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
