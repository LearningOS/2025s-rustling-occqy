// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.



// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day < 22 {
        Some(5)             //如果 time_of_day 小于 22，返回 Some(5)，表示还有 5 块冰淇淋
    } else if time_of_day == 22 {
        Some(0)             //如果 time_of_day 等于 22，返回 Some(0)，表示冰淇淋被吃完了。
    } else {
        None                //否则（time_of_day 大于 22），返回 None，表示没有冰淇淋了。
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), None);
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap_or(0), 5);  //使用 unwrap_or 方法从 Option 中取出值，如果 Option 是 None，则返回默认值 0。这样就可以正确地与 u16 类型的 5 进行比较了
    }
}
