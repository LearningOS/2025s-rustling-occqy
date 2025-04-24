// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    // 在 Rust 中，assert_eq! 宏用于断言两个值是否相等。它的基本语法是 assert_eq!(left, right)，
    // 其中 left 和 right 是要比较的值。如果 left 和 right 相等，断言通过；
    // 如果不相等，断言失败，程序会抛出一个错误。
    fn you_can_assert_eq() {
        assert_eq!(true, true);
    }
}
