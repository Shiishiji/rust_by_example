
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // error[E0384]: cannot assign twice to immutable variable `_immutable_binding`
    //   --> src/main.rs:12:5
    //    |
    // 3  |     let _immutable_binding = 1;
    //    |         ------------------
    //    |         |
    //    |         first assignment to `_immutable_binding`
    //    |         help: consider making this binding mutable: `mut _immutable_binding`
    // ...
    // 12 |     _immutable_binding += 1;
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
    // _immutable_binding += 1;
}
