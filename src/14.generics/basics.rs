
struct A;

struct Single(A);

struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);

    // Explicitly
    let _char: SingleGen<char> = SingleGen('a');

    // Implicitly
    let _t = SingleGen(A);
    let _i32 = SingleGen(2);
    let _char = SingleGen('a');
}
