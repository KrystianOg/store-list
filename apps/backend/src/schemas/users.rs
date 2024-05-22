use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct Signup {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}
