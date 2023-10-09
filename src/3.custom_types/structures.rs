use std::fmt;
use std::fmt::{Formatter, write};
use std::mem;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn square(top_left: Point, size: f32) -> Rectangle {
    let bottom_right = Point {
        x: top_left.x + size,
        y: top_left.y + size,
    };

    Rectangle {
        top_left,
        bottom_right,
    }
}

fn main() {
    let p = Point {x: 1.0, y: 55.0};
    let s = 10.0;

    let square = square(p, s);

    println!("{:?}", &square);
    println!("Size of struct is: {} bytes", mem::size_of_val(&square));
}
