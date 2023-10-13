
mod my_mod {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents,
            }
        }
    }
}

fn main() {
    let open_box = my_mod::OpenBox { contents: "Public info" };
    println!("Open box: {}", open_box.contents);

    // cannot do it
    // let closed_box = my_mod::ClosedBox { contents: "Private info" };

    let closed_box = my_mod::ClosedBox::new("Private info");

    // cannot do it
    // println!("{}", closed_box.contents);
}
