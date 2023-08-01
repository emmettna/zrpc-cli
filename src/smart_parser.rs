use std::fmt::{Display, Formatter};
use serde_json::Value;
use crate::json_domain;

use json_domain::*;
use log::*;


#[derive(Debug)]
pub struct SmartParser {
    pub origin: String,
}

impl SmartParser {
    pub fn new(s: &str) -> SmartParser {
        SmartParser { origin: String::from(s) }
    }

    fn try_auto_correction(parts: Vec<JsonPart>, tries: usize) -> Result<Value, String> {
        debug!("Trying auto correction #take {}", tries);
        debug!("Parts: {:?}", &parts);
        let updated_parts = parts.iter().fold(vec![], |mut acc, x| {
            if let Some(last) = acc.last() {
                if !JsonPart::next_expected(last).contains(x) {
                    if let (Some(middle), Position::Middle) =
                        JsonPart::perhaps_missed_this(last, x, &JsonPart::End) {
                        acc.push(middle);
                    }
                }
            }
            acc.push(*x);
            acc
        });

        let maybe_json_string = JsonPartStack::translate_back(&updated_parts);
        match serde_json::from_str::<Value>(maybe_json_string.as_str()) {
            Ok(j) => Ok(j),
            Err(e) => if tries > 0 {
                Self::try_auto_correction(updated_parts, tries - 1)
            } else {
                Err(format!("{}", e.to_string()))
            }
        }
    }

    pub fn parse(&self) -> Result<Value, String> {
        let input = self.origin.as_str();
        let serde_parsed = serde_json::from_str::<Value>(input);
        match serde_parsed {
            Ok(json) => Ok(json),
            Err(_) => {
                Self::try_auto_correction(
                    JsonPartStack::translate_to_parts(self.origin.as_str()),
                    3,
                )
            }
        }
    }
}

impl Display for SmartParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_quote() {
        let parser = SmartParser::new("{name:john}");
        let parsed = parser.parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn missing_opening_curly_bracket() {
        let parsed = SmartParser::new("\"name\": \"john\"}").parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn missing_closing_curly_bracket() {
        let parsed = SmartParser::new("{\"name\": \"john\"").parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn missing_curly_bracket() {
        let parsed = SmartParser::new("\"name\": \"john\"").parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn missing_colon() {
        let parsed = SmartParser::new("{\"name\" \"john\"}").parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn equal_instead_of_colon() {
        let parsed = SmartParser::new("{\"name\" = \"john\"}").parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }

    // Unsupported
    #[test]
    #[ignore]
    fn missing_opening_square_bracket() {
        let parsed = SmartParser::new("{\"name\" : \"john\"]}").parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }

    // Unsupported
    #[test]
    #[ignore]
    fn missing_closing_square_bracket() {
        let parsed = SmartParser::new("{\"name\" : [\"john\"}").parse();
        let expected: Result<Value, String> = Ok(serde_json::from_str("{\"name\": \"john\"}").unwrap());
        assert_eq!(parsed, expected);
    }
}