// exercises/tests/build.rs
fn main() {
    // 设置 tests7 需要的时间戳环境变量
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp); // 注意是 rustc-env

    // 设置 tests8 需要的特性开关（必须保留）
    println!(r#"cargo:rustc-cfg=feature="pass""#); // 使用原始字符串避免转义
}