const PROFILE: &'static str = "RelWithDebInfo";

fn main() {
    // let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // let dst = cmake::Config::new("vgmstream")
    //     .define("BUILD_CLI", "OFF")
    //     .define("BUILD_FB2K", "OFF")
    //     .define("BUILD_WINAMP", "OFF")
    //     .define("BUILD_XMPLAY", "OFF")
    //     .define("BUILD_STATIC", "ON")
    //     .define("USE_MPEG", "OFF")
    //     .define("USE_VORBIS", "ON")
    //     .define("USE_G719", "OFF")
    //     .define("USE_FFMPEG", "OFF")
    //     .define("USE_ATRAC9", "OFF")
    //     .define("USE_CELT", "OFF")
    //     .define("USE_SPEEX", "OFF")
    //     .define("USE_G7221", "OFF")
    //     .define("USE_G719", "OFF")
    //     .build_target("libvgmstream")
    //     .profile(PROFILE)
    //     // // cohae: Kind of disappointing that the cmake crate doesnt handle this on it's own
    //     // .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", &out_dir)
    //     .build();

    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     dst.join("build/src/").join(PROFILE).display()
    // );

    let project_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}",
        project_path.join("libs").display()
    );

    println!("cargo:rustc-link-lib=static=libvgmstream");

    println!("cargo:rustc-link-lib=static=ogg");
    println!("cargo:rustc-link-lib=static=vorbis");
    println!("cargo:rustc-link-lib=static=vorbisfile");
}
