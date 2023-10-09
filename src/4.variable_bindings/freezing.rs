
fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // error[E0384]: cannot assign twice to immutable variable `_mutable_integer`
        //  --> src/main.rs:8:9
        //   |
        // 6 |         let _mutable_integer = _mutable_integer;
        //   |             ----------------
        //   |             |
        //   |             first assignment to `_mutable_integer`
        //   |             help: consider making this binding mutable: `mut _mutable_integer`
        // 7 |
        // 8 |         _mutable_integer = 50;
        //   |         ^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
