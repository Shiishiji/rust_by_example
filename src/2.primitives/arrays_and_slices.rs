use std::fmt;
use std::fmt::{Formatter, write};
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("Number of elements in the array: {}", xs.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            None => println!("Unknown index."),
            Some(xval) => println!("{}: {}", i, xval),
        }
    }
}
