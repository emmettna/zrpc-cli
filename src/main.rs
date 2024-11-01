use std::ops::Index;
use log::error;
use colored::*;

mod grpc_request_dsl;
mod user_input;
mod commands;
mod grpc_client;
mod util;
mod json_domain;
mod smart_parser;
mod logger;
mod config_loader;
mod text_coloring;

use grpc_request_dsl::*;
use user_input::*;
use util::*;
use smart_parser::*;
use crate::commands::Commands;

fn print_divider() -> () {
    println!("---------------------------------------------------\n")
}

fn handle_command(
    command: &mut Commands,
    service_request: &mut ServiceRequest,
    user_input: &mut UserInput,
) -> Result<(), String> {
    print_divider();
    match &command {
        Commands::UpdateHost => {
            command.print_command_message();
            emptiable_input(user_input, "localhost");
            let host = Host::from(user_input.get_last_input())?;
            service_request.update_host(host);
            Ok(command.set_next_step())
        }

        Commands::TakePortInput => {
            command.print_command_message();
            emptiable_input(user_input, "9090");
            let port = Port::from(user_input.get_last_input())?;
            service_request.update_port(port);
            Ok(command.set_next_step())
        }

        Commands::SendServiceListRequest => {
            command.print_command_message();
            let services = grpc_client::request_service_list(&service_request.host, &service_request.port);
            services.iter().enumerate().for_each(|(i, s)| println!("[{}] {}", i, s.0));
            let _ = non_empty_input(user_input)?;
            let user_selection_index = parse_usize(user_input.get_last_input(), &services.len())?;
            let selected_service = services.index(user_selection_index).clone();
            service_request.update_service(selected_service);
            Ok(command.set_next_step())
        }

        Commands::SendFunctionListRequest => {
            command.print_command_message();
            let functions = grpc_client::request_function_list_by(service_request);
            functions.iter().enumerate().for_each(|(i, s)| println!("[{}] {}", i, s.0));
            let _ = non_empty_input(user_input)?;
            let user_selection_index = parse_usize(user_input.get_last_input(), &functions.len())?;
            let selected_function = functions.index(user_selection_index).clone();
            service_request.update_function(selected_function);
            Ok(command.set_next_step())
        }

        Commands::TakeBodyInput => {
            command.print_command_message();
            let joined = multi_line_input().map(|lines| lines.join("\n"))?;
            match to_json(&joined) {
                Ok(j) => {
                    service_request.update_body(j.to_string());
                    Ok(command.set_next_step())
                }
                Err(_) => {
                    let j = SmartParser::new(joined.as_str()).parse()?;
                    let json_string = (&j).to_string();
                    println!("Invalid JSON format. Did you mean this instead?\n\n => {}", json_string.blue());
                    println!("{}{}", "\n\t1: Yes".green(), "\n\t2: No".color("red"));
                    non_empty_input(user_input)?;
                    let user_selection_index = parse_usize(user_input.get_last_input(), &(2 as usize))?;
                    if user_selection_index == 1 {
                        service_request.update_body(j.to_string());
                        command.set_next_step()
                    }
                    Ok(())
                }
            }
        }

        Commands::SendRequest => {
            command.print_command_message();
            let response = grpc_client::request(&service_request)?;
            println!("Server response:\n{}", response);
            Ok(command.set_next_step())
        }

        Commands::EndOfRequestSelection => {
            command.print_command_message();
            emptiable_input(user_input, "6");
            match user_input.get_last_input().as_str() {
                "1" => command.set(Commands::UpdateHost),
                "2" => command.set(Commands::TakePortInput),
                "3" => command.set(Commands::SendServiceListRequest),
                "4" => command.set(Commands::SendFunctionListRequest),
                "5" => command.set(Commands::TakeBodyInput),
                "6" | "" => command.set(Commands::SendRequest),
                "7" | "exit" => command.set(Commands::Exit),
                _ => println!("Invalid input. Type again"),
            }
            Ok(())
        }
        Commands::Exit => Ok(())
    }
}

fn main() {
    let config = config_loader::config();
    let _ = logger::init(config);

    let mut command: Commands = Commands::UpdateHost;
    let mut service_request = ServiceRequest::default();
    let mut user_input = UserInput::empty();
    let mut continuous_error_count: u8 = 0;

    loop {
        if command == Commands::Exit { break; } else {
            if continuous_error_count > 10 {
                error!("Exiting after failing 10 consecutive times");
                command.set(Commands::Exit)
            }
            if let Err(msg) = handle_command(&mut command, &mut service_request, &mut user_input) {
                eprintln!("Failed while handling command `{}`", msg);
                continuous_error_count += 1
            } else {
                continuous_error_count = 0
            }
        }
    }
}
