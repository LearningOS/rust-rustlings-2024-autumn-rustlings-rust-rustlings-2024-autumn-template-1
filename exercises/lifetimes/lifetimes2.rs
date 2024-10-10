// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



fn longest (x: String, y: String) -> String {
    if &x.len() > &y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        
        let string2 = String::from("xyz");
        result = longest(string1, string2);
    }
    println!("The longest string is {}", result);
}
