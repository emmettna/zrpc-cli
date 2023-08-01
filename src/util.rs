use std::io::BufRead;
use serde_json::{json, Value};
use crate::user_input;
use user_input::*;

pub fn parse_usize(str: String, max_bound: &usize) -> Result<usize, String> {
    let selection = str.parse::<usize>().map_err(|e|format!("parsing error {:?}", e))?;
    if (selection + 1) > *max_bound {
        Err(String::from(format!("Invalid selection {}", selection)))
    } else { Ok(selection) }
}

pub fn to_json(string_json: &String) -> Result<Value, String>{
    let empty: Value = json!("{}");
    if string_json.is_empty() { Ok(empty) } else {
        serde_json::from_str(string_json.as_str()).map_err(|e|e.to_string())
    }
}

pub fn multi_line_input() -> Result<Vec<String>, String> {
    let mut vec = vec![];
    let mut lines = std::io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let s = line.map_err(|e|format!("Failed to take multiline user input: {:?}", e))?;
        vec.push(s);
        let condition = if vec.len() < 1 { false } else {
            let string = &vec[vec.len() - 1..];
            string.join("").is_empty()
        };
        if true == condition { break }
    };
    Ok(vec)
}

pub fn non_empty_input(user_input: &mut UserInput) -> Result<(), String> {
    let mut temp_string = String::new();
    std::io::stdin().read_line(&mut temp_string).unwrap();
    temp_string = String::from(temp_string.to_string().trim()); // trim
    if temp_string.is_empty() {
        Err(String::from("Input can't be empty"))
    } else {
        user_input.push(temp_string);
        Ok(())
    }
}

pub fn emptiable_input(user_input: &mut UserInput, default: &str) -> () {
    let mut temp_string = String::new();
    std::io::stdin().read_line(&mut temp_string).unwrap();
    temp_string = String::from(temp_string.to_string().trim()); // trim
    if temp_string.is_empty() { user_input.push(String::from(default))} else { user_input.push(temp_string) }
}