use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    // First,
    Middle,
    // Last
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QuoteStatus {
    Open,
    Close
}
impl QuoteStatus {
    pub fn toggle(&mut self) {
        match self {
            Self::Open => *self = Self::Close,
            Self::Close => *self = Self::Open,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JsonPart {
    Start,
    CurlyBracketOpen,
    CurlyBracketClose,
    QuoteOpen,
    QuoteClose,
    ListOpen,
    ListClose,
    Colon,
    Comma,
    LiteralElement(char),
    NumericElement(char),
    End,
}

impl Display for JsonPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl JsonPart {
    pub fn next_expected(part: &JsonPart) -> Vec<JsonPart> {
        use JsonPart::*;
        match part {
            Start             => vec![CurlyBracketOpen],
            CurlyBracketOpen  => vec![CurlyBracketClose, QuoteOpen],
            CurlyBracketClose => vec![CurlyBracketClose, ListClose, Comma],
            QuoteOpen         => vec![LiteralElement(' '), QuoteClose],
            QuoteClose        => vec![Colon, Comma, CurlyBracketClose, ListClose],
            ListOpen          => vec![ListClose, CurlyBracketOpen, QuoteOpen, NumericElement('0')],
            ListClose         => vec![CurlyBracketClose, ListClose, Comma],
            Colon             => vec![CurlyBracketOpen, QuoteOpen, ListOpen, NumericElement('0')],
            LiteralElement(_) => vec![LiteralElement(' '), QuoteClose],
            NumericElement(_) => vec![NumericElement('0'), CurlyBracketClose, Comma],
            Comma             => vec![QuoteOpen, CurlyBracketOpen, ListOpen, NumericElement('0')],
            End               => vec![]
        }
    }

    pub fn perhaps_missed_this(prev: &JsonPart, current: &JsonPart, _current_stack: &JsonPart) -> (Option<JsonPart>, Position) {
        use JsonPart::*;
        (match (prev, current) {
            (Start, QuoteOpen)                         => Some(CurlyBracketOpen),
            (Start, LiteralElement(_))                 => Some(QuoteOpen),
            (Start, NumericElement(_))                 => Some(QuoteOpen),
            (CurlyBracketOpen, LiteralElement(_))      => Some(QuoteOpen),
            (LiteralElement(_), Colon)                 => Some(QuoteClose),
            (Colon, LiteralElement(' '))               => None,
            (Colon, LiteralElement(_))                 => Some(QuoteOpen),
            (LiteralElement(_), CurlyBracketClose)     => Some(QuoteClose),
            (LiteralElement(_), End)                   => Some(QuoteClose),
            (NumericElement(_), End)                   => Some(CurlyBracketClose),
            (QuoteClose, End)                          => Some(CurlyBracketClose),
            (QuoteClose, QuoteOpen)                    => Some(Colon),
            (QuoteClose, CurlyBracketClose)            => Some(Colon),
            (LiteralElement(' '), LiteralElement(' ')) => None,
            _c => None
        }, Position::Middle)
    }
}

#[derive(Debug)]
pub struct JsonPartStack {
    stack: Vec<JsonPart>,
}

impl JsonPartStack {
    pub fn new() -> JsonPartStack { JsonPartStack { stack: vec![] } }

    pub fn push(&mut self, part: JsonPart) -> Result<(), String> {
        fn handle_closing_case(upper: &mut JsonPartStack, incoming_part: &JsonPart, expected: JsonPart) -> Result<(), String> {
            if let Some(p) = upper.stack.last() {
                if *p != expected {
                    Err(format!("Can't close `{:?}` since the top of stack is `{:?}`. But expected `{:?}`\n\tCurrent stack: {:?}", incoming_part, upper.stack.last(), expected, upper.stack))
                } else {
                    upper.stack.pop();
                    Ok(())
                }
            } else {
                Err(format!("Something not right here. can't close since empty"))
            }
        }

        use JsonPart::*;
        match part {
            CurlyBracketOpen | QuoteOpen | ListOpen => {
                self.stack.push(part);
                Ok(())
            }
            CurlyBracketClose => handle_closing_case(self, &part, CurlyBracketOpen),
            QuoteClose => handle_closing_case(self, &part, QuoteOpen),
            ListClose => handle_closing_case(self, &part, ListOpen),
            LiteralElement(_) | NumericElement(_) | Colon | Comma => Ok(()),
            Start => Ok(()),
            End => if !(&self.stack.is_empty()) {
                Err(format!("Supposed to be empty by now but got {:?}", &self.stack))
            } else { Ok(()) }
        }
    }


    pub fn translate_to_parts(elem: &str) -> Vec<JsonPart> {
        use JsonPart::*;
        let mut container = vec![];
        container.push(Start);
        let mut quote_flag = QuoteStatus::Close;
        let mut stack = JsonPartStack::new();
        let mut count = 0;
        for e in elem.chars() {
            match e {
                '{' => container.push(CurlyBracketOpen),
                '}' => container.push(CurlyBracketClose),
                ' ' | '\n' => {
                    match quote_flag {
                        QuoteStatus::Close => (), // ignore blank
                        QuoteStatus::Open => container.push(LiteralElement(e)),
                    }
                }
                '=' => {
                    match quote_flag {
                        QuoteStatus::Close => container.push(LiteralElement(':')), // replacing equal with colon
                        QuoteStatus::Open => container.push(LiteralElement(e)),
                    }
                }
                '"' | '\'' => {
                    match quote_flag {
                        QuoteStatus::Close => {
                            container.push(QuoteOpen);
                            quote_flag.toggle()
                        }
                        QuoteStatus::Open => {
                            container.push(QuoteClose);
                            quote_flag.toggle()
                        }
                    }
                }
                '[' => container.push(ListOpen),
                ']' => container.push(ListClose),
                ':' => container.push(Colon),
                ',' => container.push(Comma),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    match quote_flag {
                        QuoteStatus::Close => container.push(NumericElement(e)),
                        QuoteStatus::Open => container.push(LiteralElement(e)),
                    }
                }
                c => container.push(LiteralElement(c))
            }
            if container.len() != count {
                match container.last() {
                    Some(p) => {
                        match stack.push(p.clone()) {
                            Ok(_) => (),
                            Err(e) => eprintln!("{}\n", e)
                        };

                        count += container.len();
                    }
                    None => ()
                }
            }
        }
        container.push(End);
        container
    }

    pub fn translate_back(parts: &Vec<JsonPart>) -> String {
        use JsonPart::*;
        let mut vec: Vec<char> = vec![];
        for p in parts {
            let character = match p {
                CurlyBracketOpen => '{',
                CurlyBracketClose => '}',
                QuoteOpen | QuoteClose => '"',
                ListOpen => '[',
                ListClose => ']',
                Colon => ':',
                Comma => ',',
                LiteralElement(char) => *char,
                NumericElement(char) => *char,
                End | Start => continue
            };
            vec.push(character)
        }
        String::from_iter(vec)
    }
}

impl Display for JsonPartStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ff: String = self.stack.iter().map(|s| s.to_string()).collect();
        write!(f, "{}", ff)
    }
}