mod json_support;
mod note_struct;
mod command_manager;

use std::env;
use crate::command_manager::manage_commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: <command> [parameter1] [parameter2]");
        return;
    }

    let command_name = &args[1];

    // param_1 is the first argument if it exists, otherwise empty string
    let param_1 = if args.len() > 2 { &args[2] } else { "" };

    // param_2 is the second argument if it exists, otherwise empty string
    let param_2 = if args.len() > 3 { &args[3] } else { "" };

    manage_commands(command_name, param_1, param_2);
}
