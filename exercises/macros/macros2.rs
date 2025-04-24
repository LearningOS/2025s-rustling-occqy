// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

// 需要将宏的定义移动到 main 函数之前
// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
// }

