// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();      //这里使用的 &str 是对字符串数据的借用，它并不拥有数据
    shopping_list.push("milk");
}

// 需要 Vec 拥有字符串数据，可使用 String 类型
// fn main() {
//     let mut shopping_list: Vec<String> = Vec::new();
//     shopping_list.push("milk".to_string());
//     println!("Shopping list: {:?}", shopping_list);
// }