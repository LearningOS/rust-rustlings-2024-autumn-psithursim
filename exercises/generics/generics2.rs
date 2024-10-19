// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

struct Wrapper {
    value: u32,
    text: String,
}

impl Wrapper {
    pub fn new(value: u32, text: String) -> Self {
        Wrapper { value, text }
    }
    pub fn get_text(&self) -> &String {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42, "hello".to_string()).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        let wrapper = Wrapper::new(0, "Foo".to_string());
        assert_eq!(wrapper.get_text(), &"Foo".to_string());
    }
}
