

fn main() {
    let closure_square_i32 = |i: i32| -> i32 { i * i };
    let closure_square_anything = |i| i*i;
    let closure_no_params = || 4*64_i32;

    println!("Square i32: {0}*{0}={1}", 1, closure_square_i32(1));
    println!("Square any: {0}*{0}={1:?}", 0.6421_f64, closure_square_anything(0.6421_f64));
    println!("Some weird no param closure: {}", closure_no_params());
}
