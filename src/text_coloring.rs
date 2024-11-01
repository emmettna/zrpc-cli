use colored::{ColoredString, Colorize};

pub fn to_success(text: &str) -> ColoredString {
    text.green()
}

pub fn to_error(text: &str) -> ColoredString {
    text.red()
}

pub fn to_plain_msg(text: &str) -> ColoredString {
    text.blue()
}

pub fn to_warn(text: &str) -> ColoredString {
    text.yellow()
}

pub fn to_unknown(text: &str) -> ColoredString {
    text.cyan()
}

pub fn to_plain(text: &str) -> ColoredString {
    text.white()
}
