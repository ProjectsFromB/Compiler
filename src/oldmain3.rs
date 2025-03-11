use std::env;
use std::fs::File;
use std::process;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the user provides at least one argument (excluding program name)
    if args.len() < 3 {
        eprintln!("Usage: {} <option> <path-to-C-file>", args[0]);
        eprintln!("Options:");
        eprintln!("  --lex       Perform lexical analysis");
        eprintln!("  --parse     Perform parsing");
        eprintln!("  --codegen   Perform code generation");
        eprintln!("  -s          Generate an assembly file");
        process::exit(1);
    }

    let option = &args[1];
    let path = &args[2];

    // Ensure input has a .c extension
    if !path.ends_with(".c") {
        eprintln!("Error: The file must have a .c extension.");
        process::exit(1);
    }

    // Remove the ".c" extension properly
    let new_name = match path.strip_suffix(".c") {
        Some(name) => name,
        None => {
            eprintln!("Error: Could not strip '.c' from filename.");
            process::exit(1);
        }
    };

    // Handle different options
    match option.as_str() {
        "--lex" => {
            println!("Performing lexical analysis on {}", path);
            process::exit(0);
        }
        "--parse" => {
            println!("Performing parsing on {}", path);
            process::exit(0);
        }
        "--codegen" => {
            println!("Performing code generation on {}", path);
            process::exit(0);
        }
        "-s" => {
            let asm_file = format!("{}.s", new_name);
            match File::create(&asm_file) {
                Ok(_) => {
                    println!("Generated assembly file: {}", asm_file);
                    process::exit(0);
                }
                Err(_) => {
                    eprintln!("Error: Failed to create assembly file '{}'", asm_file);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Unknown option '{}'", option);
            eprintln!("Use --lex, --parse, --codegen, or -s.");
            process::exit(1);
        }
    }
}

