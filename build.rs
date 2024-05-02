fn main() {
    println!("cargo:rustc-link-search=native=C:\\OpenBLAS\\OpenBLAS-0.3.26-x64\\bin");
    println!("cargo:rustc-link-lib=libopenblas");
}