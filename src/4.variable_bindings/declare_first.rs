
fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // error[E0381]: used binding `another_binding` is possibly-uninitialized
    //   --> src/main.rs:14:37
    //    |
    // 12 |     let another_binding;
    //    |         --------------- binding declared here but left uninitialized
    // 13 |
    // 14 |     println!("another binding: {}", another_binding);
    //    |                                     ^^^^^^^^^^^^^^^ `another_binding` used here but it is possibly-uninitialized
    //    |
    //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
