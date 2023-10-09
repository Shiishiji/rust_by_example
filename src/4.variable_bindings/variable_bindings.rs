
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;
    // Not prefixed unused variables generate warnings
    //   --> src/main.rs:15:9
    //    |
    // 15 |     let noisy_unused_variable = 2u32;
    //    |         ^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_noisy_unused_variable`
    //    |
    //    = note: `#[warn(unused_variables)]` on by default
    // let noisy_unused_variable = 2u32;
}
