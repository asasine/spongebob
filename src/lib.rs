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

/// Randomize the case of each letter of each word, passing non-alphabetic characters through.
///
/// # Examples
/// ```
/// use spongebob::randomize;
/// println!("{}", randomize("Hello, world!")); // possibly "hELLo WoRld!"
/// ```
pub fn randomize(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphabetic() {
                if rand::random::<bool>() {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Inserts spaces between character of the input.
///
/// # Examples
/// ```
/// use spongebob::spaceify;
/// assert_eq!(spaceify("Hello, world!"), "H e l l o ,   w o r l d ! ");
/// ```
pub fn spaceify(text: &str) -> String {
    text.chars().map(|c| format!("{} ", c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod alternate {
        use super::*;

        #[test]
        fn empty() {
            assert_eq!(alternate(""), "");
        }

        #[test]
        fn alternate_no_alphabetic() {
            assert_eq!(alternate("123"), "123");
            assert_eq!(alternate(" "), " ");
            assert_eq!(alternate("\t"), "\t");
            assert_eq!(alternate(" \t \r\n"), " \t \r\n");
        }

        #[test]
        fn single_word() {
            assert_eq!(alternate("a"), "a");
            assert_eq!(alternate("A"), "A");
            assert_eq!(alternate("aa"), "aA");
            assert_eq!(alternate("Aa"), "Aa");
            assert_eq!(alternate("AA"), "Aa");
            assert_eq!(alternate("aaa"), "aAa");
            assert_eq!(alternate("Aaa"), "AaA");
            assert_eq!(alternate("AAA"), "AaA");
        }

        #[test]
        fn multiple_words() {
            assert_eq!(alternate("a a"), "a A");
            assert_eq!(alternate("A a"), "A a");
            assert_eq!(alternate("A A"), "A a");
            assert_eq!(alternate("a a a"), "a A a");
            assert_eq!(alternate("A a a"), "A a A");
            assert_eq!(alternate("A A A"), "A a A");
            assert_eq!(alternate("a aa aaa aaaa"), "a Aa AaA aAaA");
            assert_eq!(alternate("A aa aaa aaaa"), "A aA aAa AaAa");
        }
    }

    mod randomize {
        use super::*;

        #[test]
        fn empty() {
            assert_eq!(randomize(""), "");
        }

        #[test]
        fn randomize_no_alphabetic() {
            assert_eq!(randomize("123"), "123");
            assert_eq!(randomize(" "), " ");
            assert_eq!(randomize("\t"), "\t");
            assert_eq!(randomize(" \t \r\n"), " \t \r\n");
        }

        #[test]
        fn single_word() {
            // can't test the exact output but can test that it's the same length and contains the same characters
            assert_eq!(randomize("a").to_lowercase(), "a");
            assert_eq!(randomize("A").to_lowercase(), "a");
            assert_eq!(randomize("aa").to_lowercase(), "aa");
            assert_eq!(randomize("Aa").to_lowercase(), "aa");
            assert_eq!(randomize("AA").to_lowercase(), "aa");
            assert_eq!(randomize("aaa").to_lowercase(), "aaa");
            assert_eq!(randomize("Aaa").to_lowercase(), "aaa");
            assert_eq!(randomize("AAA").to_lowercase(), "aaa");
        }

        #[test]
        fn multiple_words() {
            // can't test the exact output but can test that it's the same length and contains the same characters
            assert_eq!(randomize("a a").to_lowercase(), "a a");
            assert_eq!(randomize("A a").to_lowercase(), "a a");
            assert_eq!(randomize("A A").to_lowercase(), "a a");
            assert_eq!(randomize("a a a").to_lowercase(), "a a a");
            assert_eq!(randomize("A a a").to_lowercase(), "a a a");
            assert_eq!(randomize("A A A").to_lowercase(), "a a a");
            assert_eq!(randomize("a aa aaa aaaa").to_lowercase(), "a aa aaa aaaa");
            assert_eq!(randomize("A aa aaa aaaa").to_lowercase(), "a aa aaa aaaa");
        }
    }

    mod spaceify {
        use super::*;

        #[test]
        fn empty() {
            assert_eq!(spaceify(""), "");
        }

        #[test]
        fn single_word() {
            assert_eq!(spaceify("a"), "a ");
            assert_eq!(spaceify("aa"), "a a ");
            assert_eq!(spaceify("aaa"), "a a a ");
        }

        #[test]
        fn multiple_words() {
            assert_eq!(spaceify("a a"), "a   a ");
            assert_eq!(spaceify("a aa"), "a   a a ");
            assert_eq!(spaceify("a aaa"), "a   a a a ");
            assert_eq!(spaceify("a a a"), "a   a   a ");
            assert_eq!(spaceify("a aa a"), "a   a a   a ");
            assert_eq!(spaceify("a aa aa"), "a   a a   a a ");
        }

        #[test]
        fn non_alphabetic() {
            assert_eq!(spaceify("123"), "1 2 3 ");
            assert_eq!(spaceify(" "), "  ");
            assert_eq!(spaceify("\t"), "\t ");
            assert_eq!(spaceify(" \t \r\n"), "  \t   \r \n ");
        }
    }
}
