#[cfg(test)]
mod tests {
    #[test]
    fn some_test_func() {
        let source_string = "qwee".to_string();
        let sub_string: &str = "wee";
        assert_eq!(string_analyzer(source_string), sub_string);
    }
}
