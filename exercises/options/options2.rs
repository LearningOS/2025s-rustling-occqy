// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {       //Some(word)解构optional_target，之后if 进行判断是否为同类型Some()
            assert_eq!(word, target);           //if 条件为true所执行的语句
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;         //类型为u16
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {           //(range + 1)先进行运算，再进行循环
            optional_integers.push(Some(i));//压入类型为<Option<i8>的数据
        }

        let mut cursor = range;     //类型为u16

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {       //optional_integers.pop() 方法会从 optional_integers 向量的末尾移除并返回一个元素
            assert_eq!(integer as u32, cursor as u32);  // 从10开始比较
            cursor -= 1;        //然后cursor接着-1
        }

        assert_eq!(cursor, 0);
    }
}
