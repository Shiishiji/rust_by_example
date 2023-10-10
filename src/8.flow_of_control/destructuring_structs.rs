
struct Abc {
    x: (u32, u32),
    y: u32,
}

fn main() {
    let a_struct = Abc { x: (3, 2), y: 3 };

    match a_struct {
        Abc { x: (1, ..), .. } => println!("x = (1 ..)!"),
        Abc { x: (.., 2), .. } => println!("x = (.., 2)!"),
        Abc { .. } => println!("Always match."),
    }
}
