
fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    println!("Val: {:?}", reference);

    match *reference { val => println!("Got a value via dereferencing: {:?}", val) }

    println!("Val: {:?}", reference);

    let val = 5;

    match val {
        ref r_val => {
            println!("Reference to a val: {:?}", r_val)
        },
    }
}
