// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res = 42;
    let option = Some(12);
    // 最佳实践：明确的单次存在性检查
    if let Some(x) = option {
        res += x;
    }

    // 反模式：循环语义误导，但逻辑正确
    // while let Some(x) = option {  // 等价于 if let，但可读性差
    //     res += x;
    // }

    println!("{}", res);
}
