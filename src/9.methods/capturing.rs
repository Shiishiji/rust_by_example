use std::mem;

fn main() {
    let color = String::from("green");
    let print = || println!("`color`: {}", color); // color is borrowed here
    print();
    let _reborrow = &color; // can be borrowed because closure holds only immutable reference
    print();
    let _color_moved = color;

    let mut count = 0;
    let mut inc = || { // mutable borrow occurs here
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    // let _borrow_count = &count; // cannot borrow because closure holds mutable reference
    inc();
    let _count_borrowed = &count;
    let _count_mut_borrowed = &mut count;

    let movable = Box::new(1);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume(); // Cannot be called because movable was moved in previous invocation


    let haystack = vec![1, 2, 3];

    // Using move before vertical pipes forces closure to take ownership of captured variables
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // This cannot be done because haystack was moved
    // for x in haystack {
    //     println!("{}", x);
    // }
}
