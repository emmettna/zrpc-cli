use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct UserInput {
    pub inputs:  Vec<String>,
    pub last_input: String
}
impl UserInput {
    pub fn empty() -> UserInput { UserInput { inputs: vec!(), last_input: String::new() } }

    pub fn push(&mut self, s: String) {
        self.inputs.push(s.clone());
        self.last_input = s;
    }

    pub fn get_last_input(&self) -> String {
        self.last_input.clone()
    }
}
impl Display for UserInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.last_input, self.inputs.join(", "))
    }
}
