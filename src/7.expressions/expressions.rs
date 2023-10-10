
fn main() {
    let x = 2u8;
    let y: i32 = {
        let x_squared: i32 = (x * x) as i32;
        let x_cube = x_squared * (x as i32);

        x_cube + x_squared + (x as i32)
    };

    println!("x is {}", x);
    println!("y is {}", y);
}
