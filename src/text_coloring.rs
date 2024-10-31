use colored::{ColoredString, Colorize};

pub fn toGreen(text: &str) -> ColoredString {
    text.green()
}

pub fn toRed(text: &str) -> ColoredString {
    text.red()
}

pub fn toBlue(text: &str) -> ColoredString {
    text.blue()
}

pub fn toYellow(text: &str) -> ColoredString {
    text.yellow()
}

pub fn toCyan(text: &str) -> ColoredString {
    text.cyan()
}
