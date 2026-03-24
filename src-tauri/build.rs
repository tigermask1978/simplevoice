fn main() {
    // 告诉 whisper-rs 编译 whisper.cpp 时开启优化
    // println!("cargo:rustc-env=WHISPER_NO_AVX=0");
    // println!("cargo:rustc-env=WHISPER_NO_AVX2=0");
    // println!("cargo:rustc-env=WHISPER_NO_FMA=0");
    // println!("cargo:rustc-env=WHISPER_NO_F16C=0");

    // wk
    // 强制开启 ggml 的 AVX/AVX2/OpenMP 支持
    std::env::set_var("GGML_AVX", "ON");
    std::env::set_var("GGML_AVX2", "ON");
    std::env::set_var("GGML_FMA", "ON");
    std::env::set_var("GGML_OPENMP", "ON");
    // 追加 MSVC release 优化标志
    // std::env::set_var("CXXFLAGS", "/O2 /Ob2 /DNDEBUG");
    // std::env::set_var("CFLAGS", "/O2 /Ob2 /DNDEBUG");
    // 通过 CMAKE_TOOLCHAIN_FILE 之外的方式注入优化标志
    // std::env::set_var(
    //     "CMAKE_CXX_FLAGS_RELEASE",
    //     "/MD /O2 /Ob2 /DNDEBUG /utf-8 -nologo -Brepro -W0",
    // );
    // std::env::set_var(
    //     "CMAKE_C_FLAGS_RELEASE",
    //     "/MD /O2 /Ob2 /DNDEBUG -nologo -Brepro -W0",
    // );

    // 强制 CMake 使用 RelWithDebInfo 配置（有 /O2）
    std::env::set_var("CMAKE_BUILD_TYPE", "RelWithDebInfo");
    // 
    tauri_build::build()
}
