use std::mem;

fn applyOnce<F>(f: F) where
    F: FnOnce() {
    f();
}

fn applyMut<F>(mut f: F) where
    F: FnMut() {
    f();
}

fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    let greeting = "hello";
    let mut text = "bye bye".to_owned();

    let closureA = || {
        println!("{} world!", greeting); // Captured greeting by reference
    };

    let closureB = || {
        println!("{} world!", greeting); // Captured greeting by reference

        // This captures text by mut ref! FnMut is required to describe this closure
        text.push_str(".. baka!");
    };

    apply(closureA);
    applyMut(closureB);
    println!("{}", text);

    let closureC = || {
        // THis requires FnOnce
        mem::drop(text);
    };
    applyOnce(closureC);
    println!("{} world!", greeting);


    let double = |x| x * 2;
    println!("Three doubled: {}", apply_to_3(double));
}
