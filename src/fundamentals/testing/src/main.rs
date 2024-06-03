/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod testing {
    use crate::*;

    #[test]
    fn test_upper_clamp() {
        assert_eq!(20, clamp(22, 10, 20), "it should return 20")
    }

    #[test]
    fn test_lower_clamp() {
        assert_eq!(10, clamp(4, 10, 20), "it should return 10")
    }

    #[test]
    fn test_div() {
        assert_eq!(Some(2), div(10, 5), "it should return 2")
    }

    #[test]
    fn test_string_concat() {
        assert_eq!(
            String::from("Hello Sylvanas"),
            concat("Hello", "Sylvanas"),
            "it should return None"
        )
    }
}
