
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Associated function looks like static function in php
    // has no "self" parameter
    fn new() -> Pair {
        Pair(Box::new(1), Box::new(2))
    }

    // A method with read only access
    fn print(&self) {
        // self is not mutable so it's not possible to change values
        // self.0 = Box::new(3);
        println!("I'm a pair of {} and {}", self.0, self.1);
    }

    fn reset_to_x(&mut self, x: i32) {
        self.0 = Box::new(x);
        self.1 = Box::new(x);
    }

    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}

fn main() {
    let mut p = Pair::new();

    p.print();
    p.reset_to_x(8i32);
    p.print();

    p.destroy();
}
