use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Product {
    #[validate(length(min = 1, max = 64))]
    name: String,
    // #[validate(length)]
    category: String, // TODO: change to enum maybe?
}
