// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 在 Rust 中调用宏时，需要在宏名后面加上 ! 符号
    my_macro!();
}
