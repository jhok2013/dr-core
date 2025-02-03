//! Module for secrets handling
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;

/// Container that masks its value from Debug and Display attempts
#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Mask<T> {
    val: T,
}

impl From<&str> for Mask<String> {
    fn from(value: &str) -> Self {
        Mask::new(String::from(value))
    }
}

impl From<String> for Mask<String> {
    fn from(value: String) -> Self {
        Mask::new(value.clone())
    }
}

impl<T> Mask<T> {
    pub fn new(val: T) -> Self {
        Mask { val }
    }

    pub fn unwrap(self) -> T {
        self.val
    }
}

impl<T> Display for Mask<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mask(*****)")
    }
}

impl<T> Debug for Mask<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mask(*****)")
    }
}

/// Enum for the type of available secrets
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Secret<T> {
    Token(Mask<T>),
    Pwd(Mask<T>),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mask() {
        let m = Mask::new("hello");
        assert_eq!(m.unwrap(), "hello");
    }

    #[test]
    fn test_display_and_debug() {
        let m = Mask::new("hello");
        let disp = m.to_string();
        assert_eq!(disp, String::from("Mask(*****)"));
    }

    #[test]
    fn test_enum() {
        let m = Mask::new("hello");
        let pwd = Secret::Pwd(m);
        if let Secret::Pwd(mask) = pwd {
            assert_eq!(mask.unwrap(), "hello");
        }
    }
}
