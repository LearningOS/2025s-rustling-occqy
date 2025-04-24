// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();          // 借助 chars 方法把输入字符串转换为字符迭代器
    match c.next() {                    // 运用 next 方法获取首字符
        None => String::new(),
        Some(first) => {
            // 若首字符存在，使用 to_uppercase 方法将其转换为大写，再用 collect 方法将其收集到一个字符串中
            let mut result = first.to_uppercase().collect::<String>();
            // 利用 extend 方法把剩余字符添加到结果字符串中
            result.extend(c);
            result
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 采用 iter 方法对字符串切片创建迭代器
    // 运用 map 方法对每个字符串应用 capitalize_first 函数
    // 利用 collect 方法将结果收集到一个向量中
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // 利用 collect 方法将结果收集到一个字符串中
    words.iter().map(|word| capitalize_first(word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
