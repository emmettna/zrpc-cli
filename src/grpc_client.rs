use crate::grpc_request_dsl;

use tonic::*;
use grpc_request_dsl::*;
use std::process::{Command, Output};

fn output_handler(output: &Output) -> Result<Vec<String>, String> {
    if output.status.success() {
        let str_output = std::str::from_utf8(&output.stdout).map_err(|e| e.to_string())?;
        Ok(str_output.split("\n").filter(|p| !p.is_empty()).map(|s| String::from(s)).collect())
    } else {
        let default = String::from("None");
        let error_code = output.status.code().map_or_else(||default, |i| i.to_string());
        match std::str::from_utf8(&output.stderr) {
            Ok(msg) => Ok(vec![format!("Failed at code {} for reason: {}", error_code, msg)]),
            Err(e) => Ok(vec![format!("Failed to parse err output at code {}, reason:\n{}", error_code, e)])
        }
    }
}

fn parse_terminal_output<B>(output: &Output, f: &dyn Fn(&str) -> B) -> Result<Vec<B>, String> {
    if output.status.success() {
        // happy case
        let str_output = std::str::from_utf8(&output.stdout).map_err(|e| e.to_string())?;
        Ok(str_output.split("\n").filter(|p| !p.is_empty()).map(|s| f(s)).collect::<Vec<B>>())

    } else {
        let default = String::from("None");
        let error_code = output.status.code().map_or_else(||default, |i| i.to_string());
        match std::str::from_utf8(&output.stderr) {
            Ok(msg) => Err(format!("Failed at code {} for reason: {}",error_code, msg)),
            Err(e) => Err(format!("Failed to parse err output at code {}, reason: {}", error_code, e))
        }
    }
}

pub fn request_service_list(host: &Host, port: &Port) -> Vec<ServiceName> {
    let command = Command::new("grpcurl")
        .arg("-plaintext")
        .arg(format!("{}:{}", host.0, port.0))
        .arg("list")
        .output();

    match command {
        Ok(output) => match parse_terminal_output(&output, &|s| ServiceName(String::from(s))) {
            Ok(services) => services,
            Err(e) => {
                eprintln!("Terminal parsing error: {}", e);
                vec![]
            }
        },
        Err(e) => {
            eprintln!("Terminal parsing error: {}", e);
            vec![]
        }
    }
}
pub fn request_function_list_by(request: &ServiceRequest) -> Vec<ServiceFunction> {
    request_function_list(&request.host, &request.port, &request.service_name)
}


pub fn request_function_list(host: &Host, port: &Port, service: &ServiceName) -> Vec<ServiceFunction> {
    let command = Command::new("grpcurl")
        .arg("-plaintext")
        .arg(format!("{}:{}", host.0, port.0))
        .arg("list")
        .arg(format!("{}", service.0))
        .output();

    match command {
        Ok(output) => match parse_terminal_output(&output, &|s| ServiceFunction(String::from(s))) {
            Ok(services) => services,
            Err(e) => {
                eprintln!("Terminal parsing error: {}", e);
                vec![]
            }
        },
        Err(e) => {
            eprintln!("Terminal parsing error: {}", e);
            vec![]
        }
    }
}

pub fn request(service_request: &ServiceRequest) -> Result<String, String> {
    let s = service_request;
    let command = Command::new("grpcurl")
        .arg("-plaintext")
        .arg("-d")
        .arg(format!("{}", s.body))
        .arg(format!("{}:{}", s.host, s.port))
        .arg(format!("{}/{}", s.service_name, s.service_function))
        .output();

    let output = command.map_err(|e| e.to_string())?;
    output_handler(&output).map(|s| s.join("\n"))
}
