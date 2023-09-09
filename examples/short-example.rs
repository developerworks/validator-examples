use serde::Deserialize;

// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(phone)]
    phone: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let signup_data = SignupData {
        mail: "john@doe.com".to_string(),
        phone: "+1234567890".to_string(),
        // phone: "+8618680868047".to_string(),
        site: "https://example.org".to_string(),
        first_name: "John".to_string(),
        age: 20,
    };

    match signup_data.validate() {
        Ok(_) => {
            println!("Validated!");
        },
        Err(e) => {
            println!("{:?}", e);
        }
    };

    Ok(())
}
