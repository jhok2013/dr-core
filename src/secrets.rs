use std::fmt::Debug;
use std::fmt::Display;

pub struct Mask<T> {
    val: T,
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
