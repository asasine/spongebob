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

#[cfg(test)]
mod tests {
    mod alternate
    {
        #[test]
        fn empty() {
            assert_eq!(super::super::alternate(""), "");
        }

        #[test]
        fn altenate_no_alphabetic() {
            assert_eq!(super::super::alternate("123"), "123");
            assert_eq!(super::super::alternate(" "), " ");
            assert_eq!(super::super::alternate("\t"), "\t");
            assert_eq!(super::super::alternate(" \t \r\n"), " \t \r\n");
        }

        #[test]
        fn single_word() {
            assert_eq!(super::super::alternate("a"), "a");
            assert_eq!(super::super::alternate("A"), "A");
            assert_eq!(super::super::alternate("aa"), "aA");
            assert_eq!(super::super::alternate("Aa"), "Aa");
            assert_eq!(super::super::alternate("AA"), "Aa");
            assert_eq!(super::super::alternate("aaa"), "aAa");
            assert_eq!(super::super::alternate("Aaa"), "AaA");
            assert_eq!(super::super::alternate("AAA"), "AaA");
        }

        #[test]
        fn multiple_words() {
            assert_eq!(super::super::alternate("a a"), "a A");
            assert_eq!(super::super::alternate("A a"), "A a");
            assert_eq!(super::super::alternate("A A"), "A a");
            assert_eq!(super::super::alternate("a a a"), "a A a");
            assert_eq!(super::super::alternate("A a a"), "A a A");
            assert_eq!(super::super::alternate("A A A"), "A a A");
            assert_eq!(super::super::alternate("a aa aaa aaaa"), "a Aa AaA aAaA");
            assert_eq!(super::super::alternate("A aa aaa aaaa"), "A aA aAa AaAa");
        }
    }

    mod randomize {
        #[test]
        fn empty() {
            assert_eq!(super::super::randomize(""), "");
        }

        #[test]
        fn randomize_no_alphabetic() {
            assert_eq!(super::super::randomize("123"), "123");
            assert_eq!(super::super::randomize(" "), " ");
            assert_eq!(super::super::randomize("\t"), "\t");
            assert_eq!(super::super::randomize(" \t \r\n"), " \t \r\n");
        }

        #[test]
        fn single_word() {
            // can't test the exact output but can test that it's the same length and contains the same characters
            assert_eq!(super::super::randomize("a").to_lowercase(), "a");
            assert_eq!(super::super::randomize("A").to_lowercase(), "a");
            assert_eq!(super::super::randomize("aa").to_lowercase(), "aa");
            assert_eq!(super::super::randomize("Aa").to_lowercase(), "aa");
            assert_eq!(super::super::randomize("AA").to_lowercase(), "aa");
            assert_eq!(super::super::randomize("aaa").to_lowercase(), "aaa");
            assert_eq!(super::super::randomize("Aaa").to_lowercase(), "aaa");
            assert_eq!(super::super::randomize("AAA").to_lowercase(), "aaa");
        }

        #[test]
        fn multiple_words() {
            // can't test the exact output but can test that it's the same length and contains the same characters
            assert_eq!(super::super::randomize("a a").to_lowercase(), "a a");
            assert_eq!(super::super::randomize("A a").to_lowercase(), "a a");
            assert_eq!(super::super::randomize("A A").to_lowercase(), "a a");
            assert_eq!(super::super::randomize("a a a").to_lowercase(), "a a a");
            assert_eq!(super::super::randomize("A a a").to_lowercase(), "a a a");
            assert_eq!(super::super::randomize("A A A").to_lowercase(), "a a a");
            assert_eq!(super::super::randomize("a aa aaa aaaa").to_lowercase(), "a aa aaa aaaa");
            assert_eq!(super::super::randomize("A aa aaa aaaa").to_lowercase(), "a aa aaa aaaa");
        }
    }
}
