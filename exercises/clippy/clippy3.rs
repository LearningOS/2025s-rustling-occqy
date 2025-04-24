// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 1. 移除死代码 + 安全解包（Clippy: `unwrap_in_result`）
    // if my_option.is_none() {   // 仅当存在时解包（原逻辑矛盾，假设需修复业务）
    //     my_option.unwrap();
    // }


    // 2. 修复数组逗号缺失（编译器错误 > Clippy）
    let my_arr = &[
        -1, -2, -3,     // 添加缺失的逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. 使用clear()替代resize(0, _)（Clippy: `resize_zero`）
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();        // 语义更明确，性能相同
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    
    // 4. 使用swap()替代手动交换（Clippy: `manual_swap`）
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;

    std::mem::swap(&mut value_a, &mut value_b);       // 原子操作，防止panic
    // 或更简洁：
    // (value_a, value_b) = (value_b, value_a);

    println!("value a: {}; value b: {}", value_a, value_b);
}
