fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Bitcoin development.", name)
}

fn main() {
    let message = greet("World");
    println!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("Satoshi");
        assert_eq!(result, "Hello, Satoshi! Welcome to Bitcoin development.");
    }

    #[test]
    fn test_greet_empty() {
        let result = greet("");
        assert_eq!(result, "Hello, ! Welcome to Bitcoin development.");
    }
}
