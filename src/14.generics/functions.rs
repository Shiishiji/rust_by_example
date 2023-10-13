struct A;
struct S(A);
struct SGen<T>(T);

fn just_fun(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn gen<T>(_s: SGen<T>) {}

fn main() {
    just_fun(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(22));

    // Explicit call
    gen::<i32>(SGen(22));

    // Implicit call
    gen(SGen(22));
}
