// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]            // 1. Rustfmt跳过格式化
macro_rules! my_macro {     // 2. 定义多模式宏
    () => {                 // 3. 无参模式
        println!("Check out my macro!");
    };
    ($val:expr) => {        // 4. 带表达式参数模式
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();            // 5. 调用无参模式
    my_macro!(7777);        // 6. 调用带参模式
}
