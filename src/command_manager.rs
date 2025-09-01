use colored::*;
use crate::json_support;
use std::fs;
use std::io::{self, Write};
use crate::json_support::rename_note;
use crate::note_struct::Note;

pub fn manage_commands(command_name: &str, param_1: &str, param_2: &str) {
    match command_name {
        "help" => {
            println!("{}", "Available commands:".bright_yellow().bold());
            println!("  {}   - Show this help menu", "help".green());
            println!("  {}   - List all notes", "ls".green());
            println!("  {}   <title>   - Show a note", "cat".green());
            println!("  {}   <title>   - Create a new note", "touch".green());
            println!("  {}   <title>   - Modify a note", "mod".green());
            println!("  {}   <title>   - Append text to a note", "append".green());
            println!("  {}   <old> <new>   - Rename a note", "rename".green());
            println!("  {}   <title>   - Delete a note", "rm".green());
        }

        "ls" => {
            json_support::list_notes().unwrap();
        }

        "touch" => {
            if param_1.is_empty() {
                println!("{}", "Invalid argument. Usage: touch <title>".red());
                return;
            }

            println!("{} {}", "Creating new note:".yellow(), param_1.cyan().bold());
            let mut lines: Vec<String> = Vec::new();

            loop {
                print!("{}", "New line (-1 to finish): ".blue());
                io::stdout().flush().unwrap();

                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed to read line");
                let text = text.trim().to_string();

                if text == "-1" { break; }
                lines.push(text);
            }

            let final_text = lines.join("\n");

            json_support::create_note(param_1, &final_text);
        }

        "mod" => {
            if param_1.is_empty() {
                println!("{}", "Invalid argument. Usage: mod <title>".red());
                return;
            }

            println!("{} {}", "Modifying note:".yellow(), param_1.cyan().bold());
            let mut lines: Vec<String> = Vec::new();

            loop {
                print!("{}", "New line (-1 to finish): ".blue());
                io::stdout().flush().unwrap();

                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed to read line");
                let text = text.trim().to_string();

                if text == "-1" { break; }
                lines.push(text);
            }

            let final_text = lines.join("\n");

            match json_support::mod_note(param_1, &final_text) {
                Ok(_) => println!("{} {}", "Note modified:".green(), param_1.cyan()),
                Err(e) => println!("{} {}", "Error:".red(), e),
            }
        }

        "cat" => {
            if param_1.is_empty() {
                println!("{}", "Invalid argument. Usage: cat <title>".red());
                return;
            }

            match json_support::load_note(param_1) {
                Ok((text, date)) => {
                    println!("{} {}", "Note:".bright_yellow().bold(), param_1.cyan().bold());
                    println!("{} {}", "Date:".yellow(), date.dimmed());
                    println!("\n{}", text);
                }
                Err(_) => println!("{} {}", "Note not found:".red(), param_1.cyan()),
            }
        }

        "append" => {
            if param_1.is_empty() {
                println!("{}", "Invalid argument. Usage: append <title>".red());
                return;
            }

            println!("{} {}", "Append to note:".yellow(), param_1.cyan().bold());

            let mut lines: Vec<String> = Vec::new();
            loop {
                print!("{}", "New line (-1 to finish): ".blue());
                io::stdout().flush().unwrap();

                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed to read line");
                let text = text.trim().to_string();

                if text == "-1" { break; }
                lines.push(text);
            }


            let final_text = lines.join("\n");

            if let Err(e) = json_support::append_note(param_1, &final_text) {
                println!("{} {}", "Error:".red(), e);
            } else {
                println!("{} {}", "Note updated:".green(), param_1.cyan());
            }
        }

        "rename" => {
            if param_1.is_empty() {
                println!("{}", "Invalid argument. Usage: append <title>".red());
                return;
            } else if param_2.is_empty() {
                println!("{}", "Invalid argument. Usage: append <title>".red());
                return;
            }


            let old = param_1;
            let new = param_2;

            json_support::rename_note(old, new);
        }

        "rm" => {
            if param_1.is_empty() {
                println!("{}", "Invalid argument. Usage: delete <title>".red());
                return;
            }

            match json_support::delete_note(param_1) {
                Ok(_) => println!("{} {}", "Deleted note:".green(), param_1.cyan()),
                Err(e) => println!("{} {}", "Error:".red(), e),
            }
        }

        _ => {
            println!("{}", "Invalid command. Type `help` for usage.".red());
        }
    }
}
