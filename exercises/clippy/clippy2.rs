// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);

    if let Some(x) = option  {   
        //This checks if the option is Some(x),类型匹配正确后再进行运算
        res += x;
    }
    println!("{}", res);
}
