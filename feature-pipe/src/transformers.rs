use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const NULL: [f64; 2] = [0., 0.];

pub fn string_to_f64(input: &str) -> f64 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish() as f64
}

pub fn long_to_f64(input: i64) -> f64 {
    input as f64
}

pub fn make_nullable(input: f64) -> [f64; 2] {
    [1., input]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_hashing() {
        let result = string_to_f64(&"Hello World".to_string());
        assert_eq!(result, 7.982898449168957e18);
    }
}
