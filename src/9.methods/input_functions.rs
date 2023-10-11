
fn call<F: Fn()>(f: F) {
    f();
}

fn hello() {
    println!("Hello!");
}

fn main() {
    let closure = || println!("Hello!");

    call(hello);
    call(closure);
}