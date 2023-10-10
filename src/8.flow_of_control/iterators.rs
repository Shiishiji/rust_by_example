
// The collection can be used after iterations
// But elements aren't mutable
fn this_borrows_collection() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("Wow! Hello {}!", name),
            _ => println!("Hi, {}", name),
        }
    }

    println!("names {:?}",  names);
}

fn this_borrows_collection_and_allow_changing_elements() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = "changed";
    }

    println!("names {:?}",  names);
}

fn this_consumes_collection() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() { // elements are moved within loop
        match name {
            "Ferris" => println!("Wow! Hello {}!", name),
            _ => println!("Hi, {}", name),
        }
    }

    // error[E0382]: borrow of moved value: `names`
    //    --> src/main.rs:25:29
    //     |
    // 16  |     let names = vec!["Bob", "Frank", "Ferris"];
    //     |         ----- move occurs because `names` has type `Vec<&str>`, which does not implement the `Copy` trait
    // 17  |
    // 18  |     for name in names.into_iter() {
    //     |                       ----------- `names` moved due to this method call
    // ...
    // 25  |     println!("names {:?}",  names);
    //     |                             ^^^^^ value borrowed here after move
    // println!("names {:?}",  names);
}

fn main() {
    this_borrows_collection();
    this_consumes_collection();
    this_borrows_collection_and_allow_changing_elements();
}
