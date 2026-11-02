//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/main/src/css/placeholderUtils.js

use std::ops::Range;

use once_cell::sync::Lazy;
use regex::Regex;

static PLACEHOLDER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"__PLACEHOLDER_(\d+)__").unwrap());

const PREFIX: &str = "__PLACEHOLDER_";

pub fn make_placeholder(index: usize) -> String {
    format!("__PLACEHOLDER_{index}__")
}

pub fn placeholders(input: &str) -> impl Iterator<Item = (Range<usize>, &str)> {
    let mut cursor = 0;
    std::iter::from_fn(move || {
        while let Some(start) = input[cursor..].find(PREFIX) {
            let start = cursor + start;
            let digits = start + PREFIX.len();
            let len = input[digits..]
                .bytes()
                .take_while(u8::is_ascii_digit)
                .count();
            let end = digits + len;
            if input.as_bytes().get(end).is_some_and(|b| !b.is_ascii()) {
                // Generated indices are ASCII. Keep the old Unicode \d behavior
                // for literal markers without compiling a regex on the usual path.
                let found = PLACEHOLDER_REGEX.find(&input[start..])?;
                let range = start + found.start()..start + found.end();
                cursor = range.end;
                let index = &input[range.start + PREFIX.len()..range.end - 2];
                return Some((range, index));
            }
            if len > 0 && input[end..].starts_with("__") {
                cursor = end + 2;
                return Some((start..cursor, &input[digits..end]));
            }
            cursor = digits;
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str) {
        let expected: Vec<_> = PLACEHOLDER_REGEX
            .captures_iter(input)
            .map(|capture| {
                (
                    capture.get(0).unwrap().range(),
                    capture.get(1).unwrap().as_str(),
                )
            })
            .collect();
        assert_eq!(
            placeholders(input).collect::<Vec<_>>(),
            expected,
            "{input:?}"
        );
    }

    #[test]
    fn scanner_matches_regex() {
        for index in [
            "",
            "0",
            "123",
            "01",
            "-1",
            "1x",
            "١",
            "1١",
            "１２",
            "é",
            "²",
            "界",
            "99999999999999999999999999999999",
        ] {
            for suffix in ["", "_", "__", "___", "__PLACEHOLDER_2__"] {
                let marker = format!("{PREFIX}{index}{suffix}");
                check(&marker);
                check(&format!("é{marker}界{PREFIX}42__{marker}"));
            }
        }
        for input in [
            "",
            "plain CSS",
            "__PLACEHOLDER___PLACEHOLDER_2__",
            "__PLACEHOLDER_0____PLACEHOLDER_1__",
            "__PLACEHOLDER_é__PLACEHOLDER_3__",
        ] {
            check(input);
        }

        let pieces = [PREFIX, "_", "__", "0", "12", "١", "é", " ", "/*", "*/"];
        let mut seed = 1u64;
        for _ in 0..5000 {
            let mut input = String::new();
            for _ in 0..20 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                input.push_str(pieces[(seed >> 32) as usize % pieces.len()]);
            }
            check(&input);
        }
    }
}
