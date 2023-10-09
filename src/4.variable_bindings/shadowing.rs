
fn main() {
    let x = 1;

    {
        println!("Before being shadowed: {}", x);

        let x = "I identify as a string now!";

        println!("After being shadowed: {}", x);
    }

    println!("Outside inner block: {}", x);
    let x = 2;
    println!("Shadowed in outer block: {}", x);
}
