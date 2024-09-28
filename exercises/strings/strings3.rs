// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut i = 0;
    let mut j = input.len() - 1;
    let mut k = 0;

    let mut start = i;
    let mut end = j;

    while true {
        match input.char_indices().nth(i) {
            Some((idx, chars)) if chars == ' ' => i += 1,
            _ => start = i,
        }

        match input.char_indices().nth(j) {
            Some((idx, chars)) if chars == ' ' => j -= 1,
            _ => end = j,
        }

        let mut k2 = j - i;
        if k2 <= 0 || k == k2 {
            break;
        } else {
            k = k2;
        }
    }

    input[start..end + 1].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut i = 0;
    let mut r = String::new();

    for ch in input.chars() {
        if i == 0 {
            if ch == 'c' {
                i = 1;
            } else {
                r += ch.to_string().as_str();
                i = 0;
            }
            continue;
        }

        if i == 1 {
            if ch == 'a' {
                i = 2;
            } else {
                r += "c";
                r += ch.to_string().as_str();
                i = 0;
            }
            continue;
        }

        if i == 2 {
            if ch == 'r' {
                i = 3;
            } else {
                r += "ca";
                r += ch.to_string().as_str();
                i = 0;
            }
            continue;
        }

        if i == 3 {
            if ch == 's' {
                r += "balloons";
                i = 0;
            } else {
                r += "car";
                r += ch.to_string().as_str();
                i = 0;
            }
            continue;
        }
    }

    r
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
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
