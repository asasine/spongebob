#![deny(missing_docs)]
//! Functions to manipulate text in the style of the SpongeBob mocking meme.
//!
//! # Examples
//! ```
//! use spongebob::alternate;
//! assert_eq!(alternate("Hello, world!"), "HeLlO, wOrLd!");
//! ```

/// Alternate the case of each letter of each word, skipping non-alphabetic characters.
///
/// Alternation starts with the capitalization of the first letter of the first word, and continues across words.
///
/// # Examples
/// ```
/// use spongebob::alternate;
/// assert_eq!(alternate("Hello, world!"), "HeLlO, wOrLd!");
/// ```
pub fn alternate(text: &str) -> String {
    let capitalize_first = text
        .chars()
        .find(|c| c.is_alphabetic())
        .map(|c| c.is_uppercase())
        .unwrap_or(false);

    // alternate each alphabetic character's case, passing non-alphabetic characters through without changing the
    // alternation state
    text.chars()
        .scan(capitalize_first, |capitalize, c| {
            if c.is_alphabetic() {
                let result = if *capitalize {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                };
                *capitalize = !*capitalize;
                Some(result)
            } else {
                Some(c.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn alternate_empty() {
        assert_eq!(super::alternate(""), "");
    }

    #[test]
    fn altenate_no_alphabetic() {
        assert_eq!(super::alternate("123"), "123");
        assert_eq!(super::alternate(" "), " ");
        assert_eq!(super::alternate("\t"), "\t");
        assert_eq!(super::alternate(" \t \r\n"), " \t \r\n");
    }

    #[test]
    fn alternate_single_word() {
        assert_eq!(super::alternate("a"), "a");
        assert_eq!(super::alternate("A"), "A");
        assert_eq!(super::alternate("aa"), "aA");
        assert_eq!(super::alternate("Aa"), "Aa");
        assert_eq!(super::alternate("AA"), "Aa");
        assert_eq!(super::alternate("aaa"), "aAa");
        assert_eq!(super::alternate("Aaa"), "AaA");
        assert_eq!(super::alternate("AAA"), "AaA");
    }

    #[test]
    fn alternate_multiple_words() {
        assert_eq!(super::alternate("a a"), "a A");
        assert_eq!(super::alternate("A a"), "A a");
        assert_eq!(super::alternate("A A"), "A a");
        assert_eq!(super::alternate("a a a"), "a A a");
        assert_eq!(super::alternate("A a a"), "A a A");
        assert_eq!(super::alternate("A A A"), "A a A");
        assert_eq!(super::alternate("a aa aaa aaaa"), "a Aa AaA aAaA");
        assert_eq!(super::alternate("A aa aaa aaaa"), "A aA aAa AaAa");
    }
}
