mod note;
use note::Note;

use std::io::{stdin, stdout, Write};

// strip newlines
fn strip(s: &mut String) -> () {
    if  let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
}

// create note from user input on stdin
fn make_note() -> Note {
    let mut name = String::new();
    let mut contents = String::new();
    
    // get name
    print!("Name: ");
    let _ = stdout().flush();
    stdin().read_line(&mut name).expect("Invalid note name");
    strip(&mut name); // strip newlines from name

    // get contents
    println!("Contents: ");
    let __ = stdout().flush();
    stdin().read_line(&mut contents).expect("Invalid note contents");

    // populate note
    let mut user_note: Note = Note::new();
    user_note.set_name(name);
    user_note.set_data(contents);

    return user_note;
}

fn main() -> () {
    let usernote: Note = make_note();
    println!("");
    usernote.print();
}

