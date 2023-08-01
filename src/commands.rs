use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum Commands {
    UpdateHost,
    TakePortInput,
    SendServiceListRequest,
    SendFunctionListRequest,
    TakeBodyInput,
    SendRequest,
    EndOfRequestSelection,
    Exit
}
impl Commands {
    pub fn get_command_message(&self) -> Option<String> {
        match self {
            Commands::Exit                    => None,
            Commands::UpdateHost              => Some(String::from("Type Host or `Enter` for \"localhost\"")),
            Commands::TakePortInput           => Some(String::from("Type Port or `Enter` for \"9090\"")),
            Commands::SendServiceListRequest  => Some(String::from("Select service to proceed")),
            Commands::SendFunctionListRequest => Some(String::from("Select function to proceed")),
            Commands::TakeBodyInput           => Some(String::from("Type request body\nType 3 new lines in order to finish(`Enter` 3 times)\nex) {\"name\": \"Johnny\"}")),
            Commands::SendRequest             => Some(String::from("Sent request")),
            Commands::EndOfRequestSelection   => Some(String::from("Press `Enter` if want to repeat the same request.\nOtherwise select which step number
    1. Set Host
    2. Set Port
    3. Set Service
    4. Set Function
    5. Set body
    6. Repeat(or Enter)
    -------------------
    7. to exit(or 'exit')\n"
            )),
        }
    }
    pub fn print_command_message(&self) -> () {
        match Commands::get_command_message(self) {
            Some(message) => println!("{}", message),
            None => ()
        }
    }

    pub fn set(&mut self, command: Commands) {
        *self = command;
    }

    pub fn set_next_step(&mut self) {
        *self = match self {
            Commands::UpdateHost              => Commands::TakePortInput,
            Commands::TakePortInput           => Commands::SendServiceListRequest,
            Commands::SendServiceListRequest  => Commands::SendFunctionListRequest,
            Commands::SendFunctionListRequest => Commands::TakeBodyInput,
            Commands::TakeBodyInput           => Commands::SendRequest,
            Commands::SendRequest             => Commands::EndOfRequestSelection,
            Commands::EndOfRequestSelection   => Commands::Exit,
            _ => unreachable!()
        }
    }
}
impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}