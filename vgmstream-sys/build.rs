const PROFILE: &'static str = "RelWithDebInfo";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

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
        "cargo:rustc-link-search={}",
        project_path.join("libs").display()
    );

    println!("cargo:rustc-link-lib=static=libvgmstream");
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=libvorbis");
    println!("cargo:rustc-link-lib=static=libatrac9");
    println!("cargo:rustc-link-lib=static=libcelt-0061");
    println!("cargo:rustc-link-lib=static=libcelt-0110");
    println!("cargo:rustc-link-lib=static=libg719_decode");
    println!("cargo:rustc-link-lib=static=libmpg123-0");
    println!("cargo:rustc-link-lib=static=libspeex-1");

    // Copy all DLLS from lib/ to the exe directory
    let project_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let exe_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let lib_path = project_path.join("libs");
    let exe_path = exe_path.join("../../../");
    println!("Copying DLLs from {:?} to {:?}", lib_path, exe_path);

    let files = std::fs::read_dir(lib_path).unwrap();
    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        // Check if the file is a DLL
        if file_path.extension().unwrap() != "dll" {
            continue;
        }
        let file_name = file.file_name();
        let exe_file_path = exe_path.join(file_name);
        std::fs::copy(file_path, exe_file_path).unwrap();
    }

    // println!("cargo:rustc-link-lib=static=libvgmstream");
    // println!("cargo:rustc-link-lib=avcodec");
    // println!("cargo:rustc-link-lib=static=ogg");
    // println!("cargo:rustc-link-lib=static=vorbis");
    // println!("cargo:rustc-link-lib=static=vorbisfile");
}
