use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

// impl From<i32> for Number {
//     fn from(value: i32) -> Self {
//         Number { value }
//     }
// }

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    // let num = Number::from(10);
    //
    // println!("My number is: {:?}", num);

    let int = 5;
    let num: Number = int.into();

    println!("My 2nd number is: {:?}", num);
}
