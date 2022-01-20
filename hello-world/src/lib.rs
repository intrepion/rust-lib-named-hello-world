#[cfg(test)]
mod greet_should {
    use crate::greet;

    #[test]
    fn return_hello_world_with_name_arg_empty_string() {
        let expected = "Hello, world!".to_owned();

        let actual = greet("".to_owned());

        assert_eq!(actual, expected);
    }
}

pub fn greet(name: String) -> String {
    name
}
