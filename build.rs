fn main() {
    #[cfg(target_os = "windows")]
    {
        // println!("cargo:rustc-link-search=native=C:\\OpenBLAS\\OpenBLAS-0.3.26-x64\\bin");
        println!("cargo:rustc-link-lib=libopenblas");
    }

    #[cfg(target_os = "linux")]
    {
        // println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
        println!("cargo:rustc-link-lib=dylib=libopenblas");
    }
}