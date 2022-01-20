#[cfg(test)]
mod greet_should {
    use crate::greet;

    #[test]
    fn return_hello_world_with_name_arg_empty_string() {
        let _expected = "Hello, world!";

        let _actual = greet("".to_string());
    }
}

pub fn greet(name: String) -> String {
    name
}
