use std::fs::File;
use std::io::{self, Read, Write, BufRead};
use std::path::Path;

fn main() {
    print!("Enter the path to the JPG file: ");
    io::stdout().flush().unwrap();
    let jpg_path = read_input_path();

    if !Path::new(&jpg_path).exists() {
        eprintln!("Error: JPG file does not exist.");
        return;
    }

    print!("Enter the path to the TXT file containing the seed phrase: ");
    io::stdout().flush().unwrap();
    let txt_path = read_input_path();

    if !Path::new(&txt_path).exists() {
        eprintln!("Error: TXT file does not exist.");

        return;
    }

    let mut jpg_file = match File::open(&jpg_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: Failed to open JPG file.");

            return;
        }
    };

    let mut jpg_buffer = Vec::new();
    if let Err(_) = jpg_file.read_to_end(&mut jpg_buffer) {
        eprintln!("Error: Failed to read JPG file.");

        return;
    }

    let mut txt_file = match File::open(&txt_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: Failed to open TXT file.");

            return;
        }
    };

    let mut seed_phrase = String::new();
    if let Err(_) = txt_file.read_to_string(&mut seed_phrase) {
        eprintln!("Error: Failed to read TXT file.");

        return;
    }

    let marker = "\n---SEEDPHRASE---\n";

    jpg_buffer.extend_from_slice(marker.as_bytes());
    jpg_buffer.extend_from_slice(seed_phrase.as_bytes());

    // Save the new polyglot file
    let mut output = match File::create("polyglot.jpg") {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: Failed to create output file.");

            return;
        }
    };
    if let Err(_) = output.write_all(&jpg_buffer) {
        eprintln!("Error: Failed to write output file.");

        return;
    }

    println!("polyglot.jpg created successfully!");
}

fn read_input_path() -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    
    stdin.lock().read_line(&mut line).expect("Failed to read input");
    line.trim().to_string()
}
