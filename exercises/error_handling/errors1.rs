// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.



pub fn generate_nametag_text(name: String) -> Result<String, String> {      //Result<T, E>, 仅返回一个: T or E
    if name.is_empty() {
        // Empty names aren't allowed.

        // 当 name 为空时，操作失败，返回 Err 变体并携带错误信息
        Err("`name` was empty; it must be nonempty.".to_string())

    } else {
        // 当 name 不为空时，操作成功，返回 Ok 变体并携带生成的姓名标签文本
        Ok(format!("Hi! My name is {}", name))  //format! 宏会生成一个 String 类型的值,所以不需要.to_string()进行 &str 类型到String类型的转换
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()),
            Ok("Hi! My name is Beyoncé".to_string())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".to_string()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".to_string())
        );// &str 类型（字符串字面量）需要 .to_string() 来转换为 String 类型作为 generate_nametag_text的name参数
    }
}
