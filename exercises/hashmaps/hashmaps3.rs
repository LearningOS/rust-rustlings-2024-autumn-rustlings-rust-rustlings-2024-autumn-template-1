// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // 初始化一个新的 HashMap
    let mut basket = HashMap::new();

    // 两根香蕉已经放入篮子中
    basket.insert(String::from("banana"), 2);
    
    // 在这里添加更多水果
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("orange"), 1);
    basket.insert(String::from("grape"), 5);
    basket.insert(String::from("kiwi"), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}



/*use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
let mut basket = // TODO: declare your hash map here.

// Two bananas are already given for you :)
basket.insert(String::from("banana"), 2);

// TODO: Put more fruits in your basket here.

basket
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn at_least_three_types_of_fruits() {
    let basket = fruit_basket();
    assert!(basket.len() >= 3);
}

#[test]
fn at_least_five_fruits() {
    let basket = fruit_basket();
    assert!(basket.values().sum::<u32>() >= 5);
}
}error[E0425]: cannot find value basket in this scope
--> exercises/hashmaps/hashmaps1.rs:22:5
|
22 |     basket.insert(String::from("banana"), 2);
|     ^^^^^^ not found in this scope

error: aborting due to 1 previous error
怎么修改
代码报错如下*/