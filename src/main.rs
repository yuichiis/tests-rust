extern "C" {
    fn cblas_daxpy(N: i32, alpha: f64, X: *const f64, incX: i32, Y: *mut f64, incY: i32);
}

fn main() {
    let n = 5;
    let alpha = 4.0;
    let mut y: Vec<f64> = vec![1.0; n as usize];
    let x: Vec<f64> = vec![2.0; n as usize];

    unsafe {
        cblas_daxpy(n, alpha, x.as_ptr(), 1, y.as_mut_ptr(), 1);
    }

    println!("Updated Y: {:?}", y);
}
