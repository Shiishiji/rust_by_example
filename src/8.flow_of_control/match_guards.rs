#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

type T = Temperature;

fn main() {
    let t = T::Celsius(35);

    match t {
        T::Celsius(t) if t > 30 => println!("it's damn hot -> {}C", t),
        T::Celsius(t) => println!("temp is -> {}C", t),
        T::Fahrenheit(t) => println!("temp is -> {} Fahrenheit", t),
    }
}
