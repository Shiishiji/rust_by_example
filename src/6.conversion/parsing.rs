use std::str::FromStr;

#[derive(Debug)]
struct UserRegistrationFormDto {
    username: String,
}

impl FromStr for UserRegistrationFormDto {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let username = s.to_owned();
        if username.len() > 5 {
            Ok(UserRegistrationFormDto { username })
        } else {
            Err(())
        }
    }
}

fn main() {
    let user_form_str = "Matthew";

    let form = user_form_str.parse::<UserRegistrationFormDto>().unwrap();
    println!("Received form: {:?}", form);
}
