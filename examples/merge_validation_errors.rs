use validator::Validate;

#[derive(Debug, Validate)]
struct UserRegistration {
    #[validate(email)]
    email: String,

    #[validate(length(min = 8))]
    password: String,
}

#[derive(Debug, Validate)]
struct UserProfileUpdate {
    #[validate(length(min = 3))]
    username: String,
}

fn main() {
    let registration_data = UserRegistration {
        email: "invalid-email".to_string(),
        password: "short".to_string(),
    };

    let profile_update_data = UserProfileUpdate {
        username: "us".to_string(),
    };

    // 合并两个步骤的错误
    let combined_errors = validator::ValidationErrors::merge(
        registration_data.validate(),   // 父节点
        "user_profile_update",          // 子节点字段名
        profile_update_data.validate(), // 子节点
    );

    let validation_errors = combined_errors.as_ref().unwrap_err();
    // 处理合并的错误
    if !validation_errors.is_empty() {
        println!("Validation successful!");
    } else {
        println!("Validation failed!");

        for (field, err_kind) in combined_errors.unwrap_err().errors() {
            println!("Field: {}, Kind: {:?}", field, err_kind);
            println!();
        }
    }
}
