use std::env;

fn main() {
    // let mut clang_path;

    // if let Some(t) = env::var_os("LIBCLANG_PATH") {
    //     clang_path = t.to_str().unwrap().to_owned();
    //     clang_path.push_str("\\clang++.exe");
    // } else {
    //     eprintln!("LIBCLANG_PATH environnement variable must be specified.");
    //     return;
    // }

    println!(
        "cargo:rustc-link-search={}",
        "C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\10.0.18362.0\\um\\x64"
    );

    // println!("cargo:rustc-link-arg={}", "/MT");

    println!("cargo:rustc-link-lib={}", "OpenGL32");
    println!("cargo:rustc-link-lib={}", "Uuid");
    println!("cargo:rustc-link-lib={}", "User32");
    println!("cargo:rustc-link-lib={}", "Gdi32");

    let include_path = std::path::PathBuf::from("SpoutGL"); // include path

    let path = format!("src/bridge.rs");
    let mut b = autocxx_build::Builder::new(path, &[&include_path])
    .build().unwrap();

    b.file("SpoutGL/Spout.cpp")
        .file("SpoutGL/SpoutCopy.cpp")
        .file("SpoutGL/SpoutDirectX.cpp")
        .file("SpoutGL/SpoutDX.cpp")
        .file("SpoutGL/SpoutFrameCount.cpp")
        .file("SpoutGL/SpoutGL.cpp")
        .file("SpoutGL/SpoutGLextensions.cpp")
        .file("SpoutGL/SpoutReceiver.cpp")
        .file("SpoutGL/SpoutSender.cpp")
        .file("SpoutGL/SpoutSenderNames.cpp")
        .file("SpoutGL/SpoutSharedMemory.cpp")
        .file("SpoutGL/SpoutUtils.cpp");

    b.compile("spout");
}
