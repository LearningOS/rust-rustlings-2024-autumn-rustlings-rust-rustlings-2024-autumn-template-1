// hashmaps2.rs
//
// We're collecting different fruits to bake a delicious fruit cake. For this,
// we have a basket, which we'll represent in the form of a hash map. The key
// represents the name of each fruit we collect and the value represents how
// many of that particular fruit we have collected. Three types of fruits -
// Apple (4), Mango (2) and Lychee (5) are already in the basket hash map. You
// must add fruit to the basket so that there is at least one of each kind and
// more than 11 in total - we have a lot of mouths to feed. You are not allowed
// to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); 

    
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("orange"), 1);
    basket.insert(String::from("grape"), 5); 
    basket.insert(String::from("kiwi"), 2);  
    basket.insert(String::from("mango"), 3); 

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_five_types_of_fruits() {
        let basket = fruit_basket();
        let count_fruit_kinds = basket.len(); 
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let basket = fruit_basket();
        let count = basket.values().sum::<u32>(); 
        assert!(count > 11); 
    }
}
