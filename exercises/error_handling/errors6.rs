// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {         // 自定义枚举错误类型 ParsePosNonzeroError -- 有两个变体
    Creation(CreationError),        // 表示在创建 PositiveNonzeroInteger 实例时出现的错误
    ParseInt(ParseIntError),        // 表示在将字符串解析为整数时出现的错误
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)     //将 CreationError 转换为 ParsePosNonzeroError::Creation 类型
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)     //将 ParseIntError 转换为 ParsePosNonzeroError::ParseInt 类型
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;       //如果解析失败，parse 方法会返回 Err(ParseIntError)，map_err 方法将 ParseIntError 转换为 ParsePosNonzeroError::ParseInt 类型的错误，? 运算符会将这个错误返回给调用者
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation) //调用 PositiveNonzeroInteger::new(x) 方法尝试创建 PositiveNonzeroInteger 实例。
    //如果创建失败，new 方法会返回 Err(CreationError)，map_err 方法将 CreationError 转换为 ParsePosNonzeroError::Creation 类型的错误。如果创建成功，返回 Ok(PositiveNonzeroInteger)。
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);     //是一个元组结构体，用于存储正的非零整数

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // 测试输入字符串无法解析为整数的情况，期望返回 Err(ParsePosNonzeroError::ParseInt)
    fn test_parse_error() {     
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    // 测试输入为负数的情况，期望返回 Err(ParsePosNonzeroError::Creation(CreationError::Negative))
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    // 测试输入为零的情况，期望返回 Err(ParsePosNonzeroError::Creation(CreationError::Zero))
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    // 测试输入为正的非零整数的情况，期望返回 Ok(PositiveNonzeroInteger)
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
