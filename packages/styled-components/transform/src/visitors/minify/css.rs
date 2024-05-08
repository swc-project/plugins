//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/4e2eb388d9c90f2921c306c760657d059d01a518/src/minify/index.js

use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;
use swc_atoms::Atom;

use super::{
    css_placeholder::{make_placeholder, split_by_placeholders, PLACEHOLDER_REGEX},
    regex_util::split_keep,
};

fn inject_unique_placeholders(str_arr: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut result = String::new();

    for (i, s) in str_arr.into_iter().enumerate() {
        if i > 0 {
            result.push_str(&make_placeholder(i - 1));
        }
        result.push_str(s.as_ref());
    }

    result
}

static LINEBREAK_REGEX_RAW: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:\\r|\\n|\r|\n)\s*").unwrap());
static MULTILINE_COMMENT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)/\*[^!].*?\*/").unwrap());
static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s*[;:{},]\s*").unwrap());

/// Counts occurrences of a character inside string
fn count_occurrences(s: impl AsRef<str>, c: char) -> usize {
    s.as_ref().split(c).count() - 1
}

/// Joins substrings until predicate returns true
fn reduce_substr(
    substrs: impl IntoIterator<Item = impl AsRef<str>>,
    join: impl AsRef<str>,
    predicate: impl Fn(&str) -> bool,
) -> String {
    let mut res = "".to_string();

    for (i, substr) in substrs.into_iter().enumerate() {
        if i == 0 {
            res.push_str(substr.as_ref());
            continue;
        }
        if predicate(&res) {
            break;
        }
        res.push_str(join.as_ref());
        res.push_str(substr.as_ref());
    }

    res
}

/// Joins at comment starts when it's inside a string or parentheses
/// effectively removing line comments
fn strip_line_comment(line: impl AsRef<str>) -> String {
    reduce_substr(line.as_ref().split("//"), "//", |s| {
        !s.ends_with(':') // NOTE: This is another guard against urls, if they're not inside strings or parantheses.
            && count_occurrences(s, '\'') % 2 == 0
            && count_occurrences(s, '"') % 2 == 0
            && count_occurrences(s, '(') == count_occurrences(s, ')')
    })
}

fn compress_symbols(code: impl AsRef<str>) -> String {
    split_keep(&SYMBOL_REGEX, code.as_ref())
        .into_iter()
        .enumerate()
        .fold("".to_string(), |s, (index, fragment)| {
            // Even-indices are non-symbol fragments
            if index % 2 == 0 {
                return s + fragment;
            }

            // Only manipulate symbols outside of strings
            if count_occurrences(&s, '\'') % 2 != 0 || count_occurrences(&s, '"') % 2 != 0 {
                return s + fragment;
            }

            // Preserve whitespace preceding colon, to avoid joining selectors.
            if !fragment.starts_with(':') && fragment.trim_start().starts_with(':') {
                return s + " " + fragment.trim();
            }

            s + fragment.trim()
        })
}

/// Detects lines that are exclusively line comments
fn is_line_comment(s: impl AsRef<str>) -> bool {
    s.as_ref().trim_start().starts_with("//")
}

/// Minifies a string of CSS code
fn minify(code: impl AsRef<str>, linebreak_regex: &Regex) -> String {
    // Remove multiline comments
    let code = MULTILINE_COMMENT_REGEX.replace_all(code.as_ref(), "\n");

    let code = linebreak_regex
        .split(&code) // Split at newlines
        .filter(|line| !line.is_empty() && !is_line_comment(line)) // Removes lines containing only line comments
        .map(strip_line_comment) // Remove line comments inside text
        .collect::<Vec<_>>()
        .join(" "); // Rejoin all lines

    compress_symbols(code)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinifyResult {
    pub values: Vec<Atom>,

    /// Indices of expressions that are not eliminated (i.e. not in comments).
    pub retained_expression_indices: HashSet<usize>,
}

/// Minifies template literal quasis
fn minify_values(
    values: impl IntoIterator<Item = impl AsRef<str>>,
    linebreak_regex: &Regex,
) -> MinifyResult {
    let code = inject_unique_placeholders(values);
    let minified_code = minify(code, linebreak_regex);

    let minified_values = split_by_placeholders(&minified_code)
        .into_iter()
        .map(Atom::from)
        .collect();

    let retained_expression_indices: HashSet<usize> = PLACEHOLDER_REGEX
        .captures_iter(&minified_code)
        .map(|captures| captures[1].parse().unwrap())
        .collect();

    MinifyResult {
        values: minified_values,
        retained_expression_indices,
    }
}

pub fn minify_raw_values(values: impl IntoIterator<Item = Atom>) -> MinifyResult {
    minify_values(values, &LINEBREAK_REGEX_RAW)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_unique_placeholders() {
        assert_eq!(
            inject_unique_placeholders(vec!["a", "b", "c"]),
            "a__PLACEHOLDER_0__b__PLACEHOLDER_1__c"
        );
    }

    #[test]
    fn test_count_occurrences() {
        assert_eq!(count_occurrences("abbbcc", 'a'), 1);
        assert_eq!(count_occurrences("abbbcc", 'b'), 3);
        assert_eq!(count_occurrences("abbbcc", 'c'), 2);
        assert_eq!(count_occurrences("abbbcc", 'd'), 0);
    }

    #[test]
    fn test_strip_line_comment() {
        // splits a line by potential comment starts and joins until one is an actual
        // comment
        assert_eq!(strip_line_comment("abc def//ghi//jkl"), "abc def");

        // ignores comment markers that are inside strings
        assert_eq!(
            strip_line_comment(r#"abc def"//"ghi'//'jkl//the end"#),
            r#"abc def"//"ghi'//'jkl"#
        );

        // ignores comment markers that are inside parantheses
        assert_eq!(
            strip_line_comment(r#"bla (//) bla//the end"#),
            r#"bla (//) bla"#
        );

        // ignores even unescaped URLs
        assert_eq!(
            strip_line_comment(r#"https://test.com// comment//"#),
            r#"https://test.com"#
        );
    }

    #[test]
    fn test_compress_symbols() {
        // removes spaces around symbols
        // The whitespace preceding the colon is removed here as part of the
        // trailing whitespace on the semi-colon. Contrast to the "preserves"
        // test below.
        assert_eq!(compress_symbols(";  :  {  }  ,  ;  "), ";:{},;");

        // ignores symbols inside strings
        assert_eq!(compress_symbols(r#";   " : " ' : ' ;"#), r#";" : " ' : ';"#);

        // preserves whitespace preceding colons
        assert_eq!(
            compress_symbols(r#"& :last-child { color: blue; }"#),
            r#"& :last-child{color:blue;}"#
        );
    }

    #[test]
    fn test_minify() {
        fn test(description: &str, code: &str, expected: &str) {
            // test minify()
            assert_eq!(
                minify(code, &LINEBREAK_REGEX_RAW),
                expected,
                "{}: minify",
                description
            );

            // test minify_values()
            assert_eq!(
                minify_values(vec![code], &LINEBREAK_REGEX_RAW),
                MinifyResult {
                    values: vec![expected.into()],
                    retained_expression_indices: HashSet::new(),
                },
                "{}: minify_css_quasis",
                description
            );
        }

        test(
            "Removes multi-line comments",
            "this is a/* ignore me please */test",
            "this is a test",
        );

        test(
            "Joins all lines of code",
            "this\nis\na/* ignore me \n please */\ntest",
            "this is a test",
        );

        test(
            "Removes line comments filling an entire line",
            "line one\n// remove this comment\nline two",
            "line one line two",
        );

        test(
            "Removes line comments at the end of lines of code",
            "valid line with // a comment\nout comments",
            "valid line with  out comments",
        );

        test(
            "Preserves multi-line comments starting with /*!",
            "this is a /*! dont ignore me please */ test/* but you can ignore me */",
            "this is a /*! dont ignore me please */ test",
        );

        test(
            "works with raw escape codes",
            "this\\nis\\na/* ignore me \\n please */\\ntest",
            "this is a test",
        );
    }

    #[test]
    fn test_minify_values() {
        // Returns the indices of retained placeholders (expressions)
        assert_eq!(
            minify_values(
                vec!["this is some\ninput with ", " and // ignored ", ""],
                &LINEBREAK_REGEX_RAW
            ),
            MinifyResult {
                values: vec!["this is some input with ".into(), " and ".into()],
                retained_expression_indices: vec![0].into_iter().collect(),
            }
        );
    }
}
