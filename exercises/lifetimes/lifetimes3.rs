// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



// 结构体定义中的生命周期标注

// 单一生命周期参数：struct Book<'a> 要求 author 和 title 引用的数据具有相同的生命周期，
// 这在某些情况下可能会限制代码的灵活性
// struct Book<'a> {
//     author: &'a str,
//     title: &'a str,
// }

// 多个生命周期参数：struct Book<'a, 'b> 允许 author 和 title 引用的数据具有不同的生命周期，
// 提供了更大的灵活性，使得结构体可以处理更复杂的生命周期场景。
struct Book<'a, 'b> {
    author: &'a str,
    title: &'b str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
