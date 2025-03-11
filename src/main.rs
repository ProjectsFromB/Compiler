use std::env;
use std::fs::File;
use std::process;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least the C file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} [option] <path-to-C-file>", args[0]);
        eprintln!("Options:");
        eprintln!("  --lex       Perform lexical analysis");
        eprintln!("  --parse     Perform parsing");
        eprintln!("  --codegen   Perform code generation");
        eprintln!("  -s          Generate an assembly file");
        process::exit(1);
    }

    let mut path_index = 1;
    let mut option: Option<&String> = None;

    // Check if the first argument is an option
    if args[1].starts_with('-') {
        option = Some(&args[1]);
        path_index = 2;
    }

    // Ensure a path is provided
    if path_index >= args.len() {
        eprintln!("Error: Missing path to C file.");
        process::exit(1);
    }

    let path = &args[path_index];

    // Ensure input has a .c extension
    if !path.ends_with(".c") {
        eprintln!("Error: The file must have a .c extension.");
        process::exit(1);
    }

    // Remove the ".c" extension properly
    let new_name = match path.strip_suffix(".c") {
        Some(name) => name.to_string(),
        None => {
            eprintln!("Error: Could not strip '.c' from filename.");
            process::exit(1);
        }
    };

    // If no option is given, create a file without ".c"
    if option.is_none() {
        match File::create(&new_name) {
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

    // Handle different options
    match option {
        Some(opt) => match opt.as_str() {
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
                eprintln!("Error: Unknown option '{}'", opt);
                eprintln!("Use --lex, --parse, --codegen, or -s.");
                process::exit(1);
            }
        },
        None => {} // Already handled above.
    }
}

