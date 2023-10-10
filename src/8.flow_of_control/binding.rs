
fn age() -> u32 {
    15
}

fn main() {

    match age() {
        0 => println!("0!"),
        n @ 1 ..= 12 => println!("a: {:?}", n),
        n @ 13 ..= 19 => println!("a2: {:?}", n),
        _ => (),
    }
}
