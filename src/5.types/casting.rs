#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // error[E0308]: mismatched types
    //  --> src/main.rs:5:23
    //   |
    // 5 |     let integer: u8 = decimal;
    //   |                  --   ^^^^^^^ expected `u8`, found `f32`
    //   |                  |
    //   |                  expected due to this
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    // error[E0604]: only `u8` can be cast as `char`, not `f32`
    //   --> src/main.rs:17:21
    //    |
    // 17 |     let character = decimal as char;
    //    |                     ^^^^^^^^^^^^^^^ invalid cast
    //    |
    // help: try `char::from_u32` instead (via a `u32`)
    //   --> src/main.rs:17:21
    //    |
    // 17 |     let character = decimal as char;
    //    |                     ^^^^^^^^^^^^^^^
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);
    println!("1000 as u8 is: {}", 1000 as u8);

    println!("300.0 as u8 is: {}", 300.0_f32 as u8);
    println!("-100 as u8 is: {}", -100.0_f32 as u8);
    println!("nan as u8 is: {}", f32::NAN as u8);

}
