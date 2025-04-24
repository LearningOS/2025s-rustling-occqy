// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


//定义命令枚举
pub enum Command {      
    Uppercase,              // 字符串转大写指令--无关联数据的枚举变体
    Trim,                   // 去除首尾空格指令--无关联数据的枚举变体
    Append(usize),          // 追加指定次数"bar"的指令（携带参数）
}

// 功能实现模块
mod my_module {         
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.into_iter() {
            // TODO: Complete the function body. You can do it!
            let processed = match command {         //processed为中间变量，代码可读性提升，所有权明确，扩展灵活性
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => format!("{}{}", string, "bar".repeat(n)),
            };
            output.push(processed);
        }
        output
    }
}

// 单元测试模块
#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;      //// 从父模块导入 transformer
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
