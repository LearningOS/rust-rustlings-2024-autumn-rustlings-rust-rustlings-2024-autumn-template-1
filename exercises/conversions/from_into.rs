// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug, Default, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // Step 1: Check if the provided string is empty
        if s.is_empty() {
            return Person::default();
        }

        // Step 2: Split the string on commas and collect parts
        let parts: Vec<&str> = s.split(',').collect();

        // Step 3: Extract the name
        let name = parts.get(0).unwrap_or(&"").trim().to_string();

        // Step 4: If the name is empty, return the default Person
        if name.is_empty() {
            return Person::default();
        }

        // Step 5: Check if there is a valid age provided
        if parts.len() < 2 || parts[1].trim().is_empty() {
            return Person::default(); // If no age is provided or it's empty, return default
        }

        // Attempt to parse the age
        match parts[1].trim().parse::<usize>() {
            Ok(age) => Person { name, age }, // Successfully parsed age
            Err(_) => Person::default(), // If parsing fails, return default
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_comma() {
        let input = "Mike,32,";
        let expected = Person { name: "Mike".to_string(), age: 32 };
        let result: Person = input.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let input = "Mike,32,man";
        let expected = Person { name: "Mike".to_string(), age: 32 };
        let result: Person = input.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let expected = Person::default();
        let result: Person = input.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_name() {
        let input = ",32";
        let expected = Person::default();
        let result: Person = input.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_age() {
        let input = "Mike,";
        let expected = Person::default();
        let result: Person = input.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_invalid_age() {
        let input = "Mike,twenty";
        let expected = Person::default();
        let result: Person = input.into();
        assert_eq!(result, expected);
    }
}

fn main() {
    // 示例使用
    let person: Person = "Alice,25".into();
    println!("{:?}", person); // 输出: Person { name: "Alice", age: 25 }
}
