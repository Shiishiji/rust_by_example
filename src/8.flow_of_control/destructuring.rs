
fn destructure_tuple() {
    let triple = (0, -2, 3);

    match triple {
        (0, y, z) => println!("0: 0, y: {:?}, z: {:?}", y, z),
        _ => println!("_"),
    }
}

fn destructure_array() {
    let array = [1, -2, 6];

    match array {
        [3, tail @ ..] => {
            println!("{:?}", tail);
        },
        _ => println!("None"),
    }
}

fn main() {
    // destructure_tuple();
    destructure_array();
}
