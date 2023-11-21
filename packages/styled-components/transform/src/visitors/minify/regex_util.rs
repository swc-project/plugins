use regex::Regex;

/// Split string by regex, keeping the delimiters.
pub fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for m in r.find_iter(text) {
        result.push(&text[last..m.start()]);
        result.push(m.as_str());
        last = m.start() + m.len();
    }
    result.push(&text[last..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_keep() {
        assert_eq!(
            split_keep(&Regex::new("[ ,.]+").unwrap(), "this... is a, test"),
            vec!["this", "... ", "is", " ", "a", ", ", "test"]
        );

        // Produces empty string when there are consecutive delimiters.
        assert_eq!(
            split_keep(&Regex::new("[.,]").unwrap(), ",.ab,."),
            vec!["", ",", "", ".", "ab", ",", "", ".", ""]
        );
    }
}
