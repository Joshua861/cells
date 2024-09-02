use std::fmt::Display;

/// A "tinystr" can store up to 12 characters as a u64, this lets us not use `&str`, `String`, `Vec<u8>`,
/// `[u8]`, `Vec<char>`, `[char]`, etc.
///
/// Basically we can store text on the stack, having to use `.clone()` or lifetimes.
///
/// >WARNING: It can only store lowercase alphabetic characters, and space (this is why it's so
/// >compact).
///
/// Construct it using the tiny_str! macro:
///
/// ## Examples
///
/// ```
/// tiny_str!("hello");
///
/// assert_eq!(tiny_str!("hello").to_string(), "hello");
/// assert_eq!(tiny_str!("hi").0, );
/// ```
pub struct TinyStr(pub u64);

pub const CHARS: [char; 27] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', ' ',
];

impl TinyStr {
    /// Convert the tinystr back into a string for display.
    ///
    ///```
    /// assert_eq!(tiny_str!("hello").to_string(), "hello");
    ///```
    pub fn as_string(&self) -> String {
        let num = self.0;

        let mut result = String::new();
        let mut found_non_zero = false;

        for i in (0..12).rev() {
            let index = ((num >> (i * 5)) & 0x1F) as usize;

            if index > 0 {
                found_non_zero = true;
                result.push(CHARS[index]);
            } else if found_non_zero {
                break; // Stop adding characters once we've seen a non-zero value and now see a zero
            }
        }

        result.chars().rev().collect()
    }
}

impl Display for TinyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[macro_export]
/// Converts a string into a TinyStr.
///
/// ## Examples
///
/// ```
/// tiny_str!("hello");
///
/// assert_eq!(tiny_str!("hello").to_string(), "hello");
/// assert_eq!(tiny_str!("hi").0, );
/// ```
///
/// This is a macro for performance reasons, all this logic will be done at compile time, not runtime.
macro_rules! tiny_str {
    ($s:expr) => {{
        let string: String = $s.into();
        let chars = $crate::utils::tinystr::CHARS;
        assert!(string.len() <= 12, "tiny_str can only hold 12 characters.");

        let mut num: u64 = 0;

        string.chars().enumerate().for_each(|(i, b)| {
            assert!(
                chars.iter().any(|&c| c == b),
                "tiny_str can only contain lowercase alphabetic characters & space."
            );
            num += (chars.iter().position(|&c| c == b).unwrap() as u64) << (i * 5);
        });

        TinyStr(num)
    }};
}
