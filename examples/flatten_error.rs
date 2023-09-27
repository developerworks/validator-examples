use std::collections::HashMap;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

// fn flatten_errors(errors: &ValidationErrors) -> HashMap<String, ValidationError> {
//     let mut flattened_errors = HashMap::new();

//     for (field, error_kind) in errors.errors() {
//         match error_kind {
//             ValidationErrorsKind::Field(errors) => {
//                 let field_name = field.to_string();
//                 for error in errors {
//                     flattened_errors.insert(field_name.clone(), error.clone());
//                 }
//             }
//             ValidationErrorsKind::Struct(errors) => {
//                 let nested_errors = flatten_errors(errors);
//                 for (nested_field, nested_error) in nested_errors {
//                     let field_name = format!("{}/{}", field, nested_field);
//                     flattened_errors.insert(field_name, nested_error);
//                 }
//             }
//             ValidationErrorsKind::List(errors_map) => {
//                 for (idx, errors) in errors_map {
//                     let nested_errors = flatten_errors(errors);
//                     for (nested_field, nested_error) in nested_errors {
//                         let field_name = format!("{}/{}[{}]", field, nested_field, idx);
//                         flattened_errors.insert(field_name, nested_error);
//                     }
//                 }
//             }
//         }
//     }

//     flattened_errors
// }

fn flatten_errors(errors: &ValidationErrors, parent_field: &str) -> HashMap<String, ValidationError> {
    let mut flattened_errors = HashMap::new();

    for (field, error_kind) in errors.errors() {
        let full_field = if parent_field.is_empty() {
            field.to_string()
        } else {
            format!("{}/{}", parent_field, field)
        };

        match error_kind {
            ValidationErrorsKind::Field(errors) => {
                for error in errors {
                    flattened_errors.insert(full_field.clone(), error.clone());
                }
            }
            ValidationErrorsKind::Struct(errors) => {
                let nested_errors = flatten_errors(errors, &full_field);
                flattened_errors.extend(nested_errors);
            }
            ValidationErrorsKind::List(errors_map) => {
                for (idx, errors) in errors_map {
                    let nested_errors = flatten_errors(errors, &format!("{}[{}]", full_field, idx));
                    flattened_errors.extend(nested_errors);
                }
            }
        }
    }

    flattened_errors
}

fn main() {
    // 示例用法
    let mut errors = ValidationErrors::new();
    errors.add("email", ValidationError::new("Invalid email"));
    errors.add("password", ValidationError::new("Password too short"));
    errors.add("profile/username", ValidationError::new("Username too short"));
    errors.add("profile/addresses[0]/street", ValidationError::new("Invalid street"));
    errors.add("profile/addresses[1]/street", ValidationError::new("Invalid street"));

    let flattened_errors = flatten_errors(&errors, "");

    for (field, error) in flattened_errors {
        println!("Field: {:?}, Error: {:?}", field, error.message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::ValidationErrors;

    #[test]
    fn test_flatten_errors() {
        let mut errors = ValidationErrors::new();
        errors.add("email", ValidationError::new("Invalid email"));
        errors.add("password", ValidationError::new("Password too short"));
        errors.add("profile/username", ValidationError::new("Username too short"));
        errors.add("profile/addresses[0]/street", ValidationError::new("Invalid street"));
        errors.add("profile/addresses[1]/street", ValidationError::new("Invalid street"));

        let flattened_errors = flatten_errors(&errors, "");

        assert_eq!(flattened_errors.len(), 5);
        assert_eq!(flattened_errors.get("email").unwrap().code, "Invalid email");
        assert_eq!(flattened_errors.get("password").unwrap().code, "Password too short");
        assert_eq!(
            flattened_errors.get("profile/username").unwrap().code,
            "Username too short"
        );
        assert_eq!(
            flattened_errors.get("profile/addresses[0]/street").unwrap().code,
            "Invalid street"
        );
        assert_eq!(
            flattened_errors.get("profile/addresses[1]/street").unwrap().code,
            "Invalid street"
        );
    }
}
