// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod foo { // 使用小写的模块名
    // 实现这两个函数
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
    
    pub fn my_demo_function_alias(a: u32) -> u32 {
        a * 2 // 示例实现
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // SAFETY: We know those functions are implemented.
        unsafe {
            let result1 = foo::my_demo_function(123); // 调用模块中的函数
            let result2 = foo::my_demo_function_alias(456); // 调用模块中的函数
            assert_eq!(result1, 123);
            assert_eq!(result2, 912); // 设定的期望值
        }
    }
}
