## 概述

ValidationErrors 是一个具有嵌套结果的错误类型, 类似于树状结构.

Validator 三大类错误

```rust
pub enum ValidationErrorsKind {
    Struct(Box<ValidationErrors>),
    List(BTreeMap<usize, Box<ValidationErrors>>),
    Field(Vec<ValidationError>),
}
```

例如: `examples/merge_validation_errors.rs`

```sh
Validation failed!
Field: user_profile_update, Kind: Struct(ValidationErrors({"username": Field([ValidationError { code: "length", message: None, params: {"value": String("us"), "min": Number(3)} }])}))
Field: email, Kind: Field([ValidationError { code: "email", message: None, params: {"value": String("invalid-email")} }])
Field: password, Kind: Field([ValidationError { code: "length", message: None, params: {"value": String("short"), "min": Number(8)} }])
```

Field 是叶子错误节点, Struct, List 是 Field 的容器, 要处理所有的错误, 需要向下递归

`ValidationErrors::has_error(&combined_errors, "user_profile_update")`

检查在 user_profile_update 字段上是否有错误
