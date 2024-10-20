// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.



// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
    fn maybe_icecream(time_of_day: u16) -> Option<u16> {
        // 检查时间是否合法
        if time_of_day > 23 {
            return None; // 返回 None 如果时间超出 23
        }
        // 根据时间返回相应的冰淇淋数量
        if time_of_day >= 9 && time_of_day < 22 {
            Some(5) // 在 9 点到 21 点之间可以吃 5 个冰淇淋
        } else if time_of_day == 22 || time_of_day == 23 {
            Some(0) // 22 和 23 点不可以吃冰淇淋，返回 0
        } else {
            None // 其他时间（如 0-8 点）返回 None
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn check_icecream() {
            assert_eq!(maybe_icecream(9), Some(5));  // 早上9点
            assert_eq!(maybe_icecream(10), Some(5)); // 上午10点
            assert_eq!(maybe_icecream(21), Some(5)); // 晚上9点
            assert_eq!(maybe_icecream(22), Some(0)); // 晚上10点，不吃冰淇淋
            assert_eq!(maybe_icecream(23), Some(0)); // 晚上11点，不吃冰淇淋
            assert_eq!(maybe_icecream(25), None);    // 不合法时间
        }
    
        #[test]
        fn raw_value() {
            let icecreams = maybe_icecream(12);
            assert_eq!(icecreams, Some(5)); // 12 点应该返回 Some(5)
        }
    }
    