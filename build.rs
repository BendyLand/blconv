fn main() {
    cc::Build::new()
        .file("src/video_converter.c")
        .include("src")
        .compile("video_converter");

    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=swscale");  // <-- ADD THIS
}

