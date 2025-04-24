// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    // 在 Rust 中，assert! 宏用于断言某个条件是否为 true。它的基本语法是 assert!(condition)，
    // 其中 condition 是一个布尔表达式。如果 condition 为 true，断言通过；
    // 如果为 false，断言失败，程序会抛出一个错误。
    fn you_can_assert() {
        assert!(true);
    }
}
