use crate::note_struct::Note;
use std::fs;
use std::error::Error;
use std::path::Path;
use colored::*; // Import colored crate


const PATH: &str = "./notes/Notes.json";

fn ensure_notes_dir() -> std::io::Result<()> {
    let dir = Path::new(PATH).parent().unwrap();
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}


pub fn list_notes() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(PATH).unwrap_or_else(|_| "[]".to_string());
    let notes: Vec<Note> = serde_json::from_str(&data)?;

    if notes.is_empty() {
        println!("{}", "No notes found.".red().bold());
        return Ok(());
    }

    println!("{}", "Notes:".green().bold());
    for note in notes {
        // You can color the note name however you like
        println!("- {}", note.name.blue().bold());
    }

    Ok(())
}

pub fn load_note(title: &str) -> Result<(String, String), Box<dyn Error>> {
    let data = fs::read_to_string(PATH).unwrap_or_else(|_| "[]".to_string());
    let notes: Vec<Note> = serde_json::from_str(&data)?;

    if let Some(note) = notes.into_iter().find(|n| n.name == title) {
        Ok((note.text, note.date))
    } else {
        Err("Note not found".into())
    }
}

pub fn mod_note(title: &str, new_text: &str) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(PATH).unwrap_or_else(|_| "[]".to_string());
    let mut notes: Vec<Note> = serde_json::from_str(&data)?;

    if let Some(note) = notes.iter_mut().find(|n| n.name == title) {
        note.text = new_text.to_string();
    } else {
        return Err("Note not found".into());
    }

    let updated = serde_json::to_string_pretty(&notes)?;
    fs::write(PATH, updated)?;
    Ok(())
}

pub fn append_note(title: &str, add_text: &str) -> Result<(), Box<dyn Error>> {
    let (old_text, _) = load_note(title)?;
    let new_text = format!("{}{}", old_text, add_text);
    mod_note(title, &new_text)?;
    Ok(())
}

pub fn create_note(name: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load existing notes
    let data = fs::read_to_string(PATH).unwrap_or_else(|_| "[]".to_string());
    let mut notes: Vec<Note> = serde_json::from_str(&data)?;

    // Check for duplicate name
    if notes.iter().any(|note| note.name == name) {
        println!("{}", format!("A note with the name '{}' already exists!", name).red().bold());
        return Ok(());
    }

    // Create and add the new note
    let note = Note::new(name.to_string(), text.to_string());
    notes.push(note);

    // Save back to JSON
    fs::write(PATH, serde_json::to_string_pretty(&notes)?)?;

    println!("{}", format!("Note '{}' created successfully!", name).green().bold());
    Ok(())
}


pub fn rename_note(old_name: &str, new_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(PATH).unwrap_or_else(|_| "[]".to_string());
    let mut notes: Vec<Note> = serde_json::from_str(&data)?;

    // Check if a note with the new_name already exists
    if notes.iter().any(|note| note.name == new_name) {
        println!("{}", format!("Cannot rename: a note with the name '{}' already exists!", new_name).red().bold());
        return Ok(());
    }

    // Find the note to rename
    let mut found = false;
    for note in notes.iter_mut() {
        if note.name == old_name {
            note.name = new_name.to_string();
            found = true;
            break;
        }
    }

    if !found {
        println!("{}", format!("Note '{}' not found!", old_name).red().bold());
        return Ok(());
    }

    // Save updated notes back to JSON
    fs::write(PATH, serde_json::to_string_pretty(&notes)?)?;
    println!("{}", format!("Renamed note: {} -> {}", old_name, new_name).green().bold());
    Ok(())
}

pub fn delete_note(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    ensure_notes_dir()?;
    let data = std::fs::read_to_string(PATH).unwrap_or_else(|_| "[]".to_string());
    let mut notes: Vec<Note> = serde_json::from_str(&data)?;

    notes.retain(|n| n.name != title);

    let updated = serde_json::to_string_pretty(&notes)?;
    std::fs::write(PATH, updated)?;
    Ok(())
}
