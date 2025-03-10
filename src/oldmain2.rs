use std::env;
use std::fs::File;
use std::process;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the user provided exactly one argument (excluding program name)
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-C-file>", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    // Ensure input has a .c extension
    if !path.ends_with(".c") {
        eprintln!("Error: The file must have a .c extension.");
        process::exit(1);
    }

    // Remove the ".c" extension properly
    let new_name = match path.strip_suffix(".c") {
        Some(name) => name, // Successfully stripped ".c"
        None => {
            eprintln!("Error: Could not strip '.c' from filename.");
            process::exit(1);
        }
    };

    // Attempt to create the file
    match File::create(new_name) {
        Ok(_) => {
            println!("Created file: {}", new_name);
            process::exit(0);
        }
        Err(_) => {
            eprintln!("Error: Failed to create file '{}'", new_name);
            process::exit(1);
        }
    }
}

