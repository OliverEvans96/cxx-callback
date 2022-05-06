use std::error;

type UnitResult = Result<(), Box<dyn error::Error>>;

fn build_cxx() -> UnitResult {
    println!("cargo:rerun-if-changed=include/callback.hpp");
    println!("cargo:rerun-if-changed=src/callback.cpp");

    cxx_build::bridge("src/callback.rs")
        .file("src/callback.cpp")
        .flag_if_supported("-std=c++17")
        .opt_level(3)
        .compile("posenet_vr_eigen");

    Ok(())
}

fn main() -> UnitResult {
    build_cxx()?;

    Ok(())
}
