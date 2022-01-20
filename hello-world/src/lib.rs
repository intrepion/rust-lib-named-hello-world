#[cfg(test)]
mod greet_should {
    #[test]
    fn return_hello_world_with_name_arg_empty_string() {
        let expected = "Hello, world!";

        let actual = greet("");
    }
}
