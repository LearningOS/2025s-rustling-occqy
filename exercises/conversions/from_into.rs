// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
// 实现 Default 特征，以便在提供的字符串无法转换为 Person 对象时作为备用方案
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results



// 实现 From<&str> 特征，将 &str 类型的字符串转换为 Person 对象
impl From<&str> for Person {
    // fn from(s: &str) -> Person {     无法通过测试
    //     // 步骤 1：如果提供的字符串长度为 0，则返回 Person 的默认值
    //     if s.len() == 0 {
    //         return Person::default();
    //     }
    //      // 步骤 2：根据字符串中的逗号对其进行分割
    //     let parts:Vec<&str> = s.split(',').collect();
    //     // 步骤 3：从分割操作中提取第一个元素并将其用作名字
    //     let name = parts.get(0).unwrap_or(&"").to_string();
    //      // 步骤 4：如果名字为空，则返回 Person 的默认值
    //     if name.is_empty() {
    //         return Person::default();
    //     }
    //     // 步骤 5：从分割操作中提取另一个元素，并将其解析为 usize 类型作为年龄
    //     if parts.len() < 2 {
    //         return Person::default();
    //     }
    //     let age_str = parts[1];
    //     // 尝试将年龄字符串解析为 usize 类型
    //     let age = match age_str.parse::<usize>() {
    //         Ok(age) => age,
    //         Err(_) => return Person::default(),
    //     };

    //     // 若以上步骤都成功，返回一个包含结果的 Person 对象实例
    //     Person { name, age }
    // }

    // fn from(s: &str) -> Person {
    //     // Step 1: Check for empty string
    //     if s.is_empty() {
    //         return Person::default();
    //     }

    //     // Step 2: Split string by commas
    //     let parts: Vec<&str> = s.split(',').collect();

    //     // Check if exactly two parts exist
    //     if parts.len() != 2 {
    //         return Person::default();
    //     }

    //     let name = parts[0].trim();

    //     // Step 4: Validate name
    //     if name.is_empty() {
    //         return Person::default();
    //     }

    //     // Step 5: Parse age
    //     match parts[1].trim().parse::<usize>() {
    //         Ok(age) => Person {
    //             name: name.to_string(),
    //             age,
    //         },
    //         Err(_) => Person::default(),
    //     }
    // }


    fn from(s: &str) -> Person {
        // 步骤 1：如果提供的字符串长度为 0，则返回 Person 的默认值
        if s.len() == 0 {
            return Person::default();
        }
        // 步骤 2：根据字符串中的逗号对其进行分割
        let parts: Vec<&str> = s.split(',').collect();
        // 步骤 3：从分割操作中提取第一个元素并将其用作名字
        let name = parts.get(0).unwrap_or(&"").to_string();
        // 步骤 4：如果名字为空，则返回 Person 的默认值
        if name.is_empty() {
            return Person::default();
        }
        // 步骤 5：从分割操作中提取另一个元素，并将其解析为 usize 类型作为年龄
        // 确保分割后的部分只有两个（名字和年龄），多余部分则返回默认值
        if parts.len() != 2 {
            return Person::default();
        }
        let age_str = parts[1];
        // 尝试将年龄字符串解析为 usize 类型
        let age = match age_str.parse::<usize>() {
            Ok(age) => age,
            Err(_) => return Person::default(),
        };

        // 若以上步骤都成功，返回一个包含结果的 Person 对象实例
        Person { name, age }
    }
}

fn main() {
    // Use the `from` function      // 使用 `from` 函数
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    // 由于为 Person 实现了 From 特征，因此应该能够使用 Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
