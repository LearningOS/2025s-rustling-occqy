// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



struct Wrapper<T> {     // 泛型结构体 Wrapper<T>
    value: T,           //字段必须声明为泛型类型
}

// 1.impl<T> 是泛型实现块的起始标记，它声明了一个泛型参数 T
// 表明这个 impl 块中的所有方法都是泛型方法，它们可以处理不同类型的 T；
// 2.Wrapper<T> 明确指出这个 impl 块是针对泛型结构体 Wrapper 的实现
// 可以处理任意类型 T 的 Wrapper 实例；
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
