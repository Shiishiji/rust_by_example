use std::fmt;
use std::fmt::{Formatter, write};

#[derive(Debug)]
struct Dupa<'t> {
    name: &'t str,
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (a1, a2, b1, b2) = (&self.0, &self.1, &self.2, &self.3);

        writeln!(f, "( {} {} )", a1, a2)?;
        write!(f, "( {} {} )", b1, b2)
    }
}

fn transpose(matrix: Matrix) -> Matrix
{
    let (a1, a2, b1, b2) = (matrix.0, matrix.1, matrix.2, matrix.3);

    return Matrix(a1, b1, a2, b2);
}

fn return_5_and_2() -> (u8, u8)
{
    (5, 2)
}

fn main() {

    let text: String = format!("{number:0>2}", number = 633);

    println!("{}", text);

    let d = Dupa {name: "Antonio" };
    println!("{:#?}", d);

    let list = List(vec![1, 2, 3]);
    println!("{}", list);

    println!("{:?}", return_5_and_2());

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
