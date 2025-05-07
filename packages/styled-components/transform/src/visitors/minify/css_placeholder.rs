//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/main/src/css/placeholderUtils.js

use once_cell::sync::Lazy;
use regex::Regex;

pub static PLACEHOLDER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"__PLACEHOLDER_(\d+)__").unwrap());

pub fn make_placeholder(index: usize) -> String {
    format!("__PLACEHOLDER_{index}__")
}

pub fn split_by_placeholders(input: &str) -> Vec<&str> {
    PLACEHOLDER_REGEX.split(input).collect()
}
