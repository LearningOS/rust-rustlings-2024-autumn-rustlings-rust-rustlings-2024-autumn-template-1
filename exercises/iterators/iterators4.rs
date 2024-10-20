// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



// factorial.rs

pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    // 以下是一些新的测试函数示例
    #[test]
    fn count_collection_complete() {
        // 示例实现，返回某个集合的大小
        let collection = vec![1, 2, 3, 4, 5];
        assert_eq!(collection.len(), 5);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = vec![1, 2, 3, 4, 5];
        assert_eq!(collection.iter().count(), 5);
    }

    #[test]
    fn count_collection_none() {
        let collection: Vec<i32> = Vec::new();
        assert_eq!(collection.is_empty(), true);
    }

    #[test]
    fn count_collection_some() {
        let collection = vec![1, 2, 3];
        assert_eq!(collection.get(1), Some(&2));
    }

    #[test]
    fn count_complete_equals_for() {
        let complete = vec![true, false, true];
        assert_eq!(complete.iter().filter(|&&x| x).count(), 2);
    }

    #[test]
    fn count_complete() {
        let complete = vec![true, false, true];
        assert_eq!(complete.iter().filter(|&&x| x).count(), 2);
    }

    #[test]
    fn count_none() {
        let collection: Vec<i32> = Vec::new();
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn count_some() {
        let collection = vec![1, 2, 3];
        assert_eq!(collection.len(), 3);
    }
}

fn main() {
    // 示例主函数，可以用于运行程序
    println!("Factorial of 5 is: {}", factorial(5));
}
