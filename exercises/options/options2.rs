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
        //Some(T): 表示包含一个类型为 T 的值。在这里，T 是任何类型的占位符，可以是任何类型的值。
        //if let 模式 = 表达式 {
       // 如果匹配成功，就会执行这里的代码
       if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
       while let Some( Some(integer)) = optional_integers.pop() {
    // `optional_integers.pop()` 从 `optional_integers` 向量中弹出最后一个元素。
    // 这个元素的类型是 `Option<Option<i8>>`，所以我们使用 `while let` 来进行模式匹配。
    // 如果弹出的元素是 `Some(Some(integer))`，这意味着：
    // - 外层的 `Some` 表示我们成功从向量中弹出了一个值。
    // - 内层的 `Some(integer)` 表示这个值是一个 `Option` 类型，且它也包含一个 `i8` 类型的值 `integer`。
    
    // 当匹配成功时，我们进入循环体，`integer` 就是我们要处理的 `i8` 值。
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
