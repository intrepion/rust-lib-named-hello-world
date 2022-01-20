#[cfg(test)]
mod greet_should {
    use crate::greet;

    #[test]
    fn return_hello_world_with_name_arg_empty_string() {
        let expected = "Hello, world!".to_owned();

        let actual = greet("".to_owned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_hello_oliver_with_name_arg_oliver() {
        let expected = "Hello, Oliver!".to_owned();

        let actual = greet("Oliver".to_owned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_hello_karinna_with_name_arg_karinna() {
        let expected = "Hello, Karinna!".to_owned();

        let actual = greet("Karinna".to_owned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_hello_world_with_name_arg_whitespace_only() {
        let expected = "Hello, world!".to_owned();

        let actual = greet("    ".to_owned());

        assert_eq!(actual, expected);
    }
}

pub fn greet(name: String) -> String {
    let name = name.trim();

    if name.is_empty() {
        return "Hello, world!".to_owned();
    }

    format!("Hello, {name}!")
}
