use crate::prelude::*;

#[test]
fn tiny_strs() {
    tiny_str!("hi");
    tiny_str!("hello world");
    assert_ne!(tiny_str!("hi").0, tiny_str!("hello world").0);
    assert_eq!(tiny_str!("hi").to_string(), "hi");
}
