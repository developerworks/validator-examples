use std::error;

use serde::{Deserialize, Serialize};

// A trait that the Validate derive will impl
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

#[allow(unused)]
#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(phone(message = "不是一个有效的电话号码"))]
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

#[derive(Debug, Serialize)]
pub struct ApiResult {
    code: String,
    messages: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let signup_data = SignupData {
        mail: "john@doe.com".to_string(),
        phone: "+861868086804".to_string(),
        // phone: "+8618680868047".to_string(),
        site: "https://example.org".to_string(),
        first_name: "John".to_string(),
        age: 20,
    };

    match signup_data.validate() {
        Ok(_) => {
            println!("Validated!");
        }
        Err(e) => {
            let api_result = pretty_print_errors(e);
            println!("{:?}", api_result);
            // println!("{:?}", e);
        }
    };

    Ok(())
}

pub fn pretty_print_errors(e: ValidationErrors) -> ApiResult {
    let errors = e.errors();
    let mut api_result = ApiResult {
        code: "0000".to_string(),
        messages: vec![],
    };

    for (k, tp) in errors {
        println!("k: {}", k);
        println!("tp: {:?}", tp);
        // println!("Message: {}", error.message());
        match tp {
            ValidationErrorsKind::Field(errors) => {
                api_result.code = "10001".to_string();
                for validation_error in errors {
                    let msg = validation_error.message.as_ref().unwrap().to_string();
                    api_result.messages.push(msg);
                }
            }
            ValidationErrorsKind::Struct(errors) => {
                let e = *errors.clone();
                pretty_print_errors(e);
            }
            ValidationErrorsKind::List(errors) => {
                for value in errors.values() {
                    let e = *value.clone();
                    pretty_print_errors(e);
                }
            }
        }
    }

    api_result
}
