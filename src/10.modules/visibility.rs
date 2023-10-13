
mod my_mod {
    fn private_fn() {
        println!("I'm private");
    }

    pub fn public_fn() {
        nested_mod::public_in_my_mod();
        private_fn();
        println!("I'm public");
    }

    pub mod nested_mod {
        pub(super) fn public_in_my_mod() {
            println!("hello");
        }
    }
}

fn main() {
    my_mod::public_fn()
}
