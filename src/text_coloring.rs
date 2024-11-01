use colored::{ColoredString, Colorize};

pub fn toSuccess(text: &str) -> ColoredString {
    text.green()
}

pub fn toError(text: &str) -> ColoredString {
    text.red()
}

pub fn toPlainMessage(text: &str) -> ColoredString {
    text.blue()
}

pub fn toWarn(text: &str) -> ColoredString {
    text.yellow()
}

pub fn toCyan(text: &str) -> ColoredString {
    text.cyan()
}
