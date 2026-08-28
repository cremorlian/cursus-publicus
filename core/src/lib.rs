#![no_std]

#[cfg(test)]
extern crate std;

pub fn opening_text() -> &'static str {
    "Hello World"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_text_is_hello_world() {
        assert_eq!(opening_text(), "Hello World");
    }
}
