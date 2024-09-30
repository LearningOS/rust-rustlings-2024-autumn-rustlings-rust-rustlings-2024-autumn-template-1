// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // 使用 `trim()` 函数去除字符串两端的空白字符，并将其转换为 `String` 类型
   input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    //在 Rust 中，字符串（String 类型）是可增长的，但默认情况下，所有变量都是不可变的。为了能够修改或改变一个变量的值，你需要将其声明为可变的（使用 mut 关键字）。
   let mut result=input.to_string();
   result.push_str(" world!");
   result
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
   input.replace("cars","balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
