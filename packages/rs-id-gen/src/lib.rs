//! Generate a random id using the character a-z,0-9 with a length of 12.

/// The alphabet used to generate the id.
const ALPHABET: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Generate a random id using the character a-z,0-9 with a length of 12.
///
/// # Examples
/// ```rust
/// use id_gen::gen;
///
/// let id = gen();
/// println!("{}", id);
/// ```
pub fn gen() -> String {
    nanoid::nanoid!(12, &ALPHABET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_len() {
        let id = gen();
        assert_eq!(id.len(), 12);
    }

    #[test]
    fn test_gen_only_alphabet() {
        let id = gen();
        for c in id.chars() {
            assert!(ALPHABET.contains(&c));
        }
    }

    #[test]
    fn test_gen_no_equals() {
        let id = gen();
        assert_ne!(id, gen());
    }

    #[test]
    fn test_gen_no_equals_1000() {
        for _ in 0..1000 {
            let id = gen();
            assert_ne!(id, gen());
        }
    }
}
