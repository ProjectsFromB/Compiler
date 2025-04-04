use std::env; // Import standard library module for handling command-line arguments
use std::fs::File; // Import module to handle file operations
use std::io::{self, Read}; // Import modules for input/output operations
use std::process; // Import module to handle process termination

// Define an enumeration for different types of tokens recognized by the lexer
#[derive(Debug)]
enum Token {
    Identifier(String), // Represents variable/function names
    Constant(String), // Represents numeric constants
    IntKeyword, // 'int' keyword
    VoidKeyword, // 'void' keyword
    ReturnKeyword, // 'return' keyword
    OpenParenthesis, // '('
    CloseParenthesis, // ')'
    OpenBrace, // '{'
    CloseBrace, // '}'
    Semicolon, // ';'
}

// Function to perform lexical analysis on a given file
fn lexer(file_path: &str) -> Result<(), String> {
    // Try to open the specified file
    let mut file = match File::open(file_path) {
        Ok(file) => file, // File opened successfully
        Err(_) => {
            return Err(format!("Error: Could not open file '{}'.", file_path)); // Return an error message
        }
    };

    let mut contents = String::new(); // Create a mutable string to store file contents
    // Read file contents into the string
    if let Err(_) = file.read_to_string(&mut contents) {
        return Err(format!("Error: Could not read file '{}'.", file_path)); // Return an error message
    }

    let mut tokens = Vec::new(); // Create a vector to store identified tokens
    let mut chars = contents.chars().peekable(); // Convert file content into an iterator over characters

    // Iterate through each character in the file
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' => {
                chars.next(); // Ignore whitespace characters
            }
            '(' => {
                tokens.push(Token::OpenParenthesis); // Recognize opening parenthesis
                chars.next(); // Consume the character
            }
            ')' => {
                tokens.push(Token::CloseParenthesis); // Recognize closing parenthesis
                chars.next(); // Consume the character
            }
            '{' => {
                tokens.push(Token::OpenBrace); // Recognize opening brace
                chars.next(); // Consume the character
            }
            '}' => {
                tokens.push(Token::CloseBrace); // Recognize closing brace
                chars.next(); // Consume the character
            }
            ';' => {
                tokens.push(Token::Semicolon); // Recognize semicolon
                chars.next(); // Consume the character
            }
            ///////////////////////
            '0'..='9' => {
                let mut num = String::new(); // Create a string to hold numeric constant
                while let Some(&d) = chars.peek() {
                    if d.is_numeric() {
                        num.push(d); // Add digit to the number string
                        chars.next(); // Consume the character
                    } else {
                        break; // Break if the character is no longer numeric
                    }
                }
                // Ensure the number is not followed by an identifier
                if let Some(&next) = chars.peek() {
                    if next.is_alphabetic() || next == '_' {
                        return Err(format!("Lexical Error: Identifiers cannot start with a number: '{}{}'", num, next));
                    }
                }
                tokens.push(Token::Constant(num)); // Store numeric constants
            }
            //////////////////////////////
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new(); // Create a string to hold identifier
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        ident.push(d); // Add character to the identifier string
                        chars.next(); // Consume the character
                    } else {
                        break; // Break if the character is no longer part of an identifier
                    }
                }

                eprintln!("DEBUG: Found identifier '{}'", ident);

                // Check if the identifier starts with a number
                if ident.chars().next().unwrap().is_numeric() {
                    return Err(format!("Lexical Error: Identifiers cannot start with a number: '{}'", ident));
                }

                // Check if the identifier is just an underscore
                if ident == "_" {
                    return Err(format!("Lexical Error: Standalone underscore '_' is not a valid identifier."));
                }

                // Check if the identifier contains invalid characters
                if ident.chars().any(|ch| !(ch.is_alphanumeric() || ch == '_')) {
                    return Err(format!("Lexical Error: Invalid identifier '{}'", ident));
                }

                // Match known keywords or treat as a generic identifier
                match ident.as_str() {
                    "int" => tokens.push(Token::IntKeyword), // Recognize 'int' keyword
                    "void" => tokens.push(Token::VoidKeyword), // Recognize 'void' keyword
                    "return" => tokens.push(Token::ReturnKeyword), // Recognize 'return' keyword
                    _ => tokens.push(Token::Identifier(ident)), // Otherwise, it's a generic identifier
                }
            }
            '/' => {
                chars.next(); // Consume the '/' character
                if let Some(&next) = chars.peek() {
                    if next == '/' {
                        while let Some(&c) = chars.peek() {
                            if c == '\n' { // End of single-line comment
                                break;
                            }
                            chars.next(); // Consume the character
                        }
                    } else if next == '*' {
                        chars.next(); // Consume '*'
                        while let Some(_) = chars.peek() {
                            if chars.next() == Some('*') && chars.peek() == Some(&'/') {
                                chars.next(); // Consume '/'
                                break; // End of multi-line comment
                            }
                        }
                    } else {
                        return Err("Lexical Error: Unhandled token '/'. Implement operator support.".to_string());
                    }
                }
            }
            _ => {
                return Err(format!("Lexical Error: Invalid character '{}'", c)); // Handle invalid characters
            }
        }
    }

    // Print the identified tokens
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

    // Ensure the user provides at least one argument (the path to the C file)
    if args.len() < 2 {
        eprintln!("Usage: {} [option] <path-to-C-file>", args[0]);
        eprintln!("Options:");
        eprintln!("  --lex       Perform lexical analysis");
        eprintln!("  --parse     Perform parsing");
        eprintln!("  --codegen   Perform code generation");
        eprintln!("  -s          Generate an assembly file");
        process::exit(1); // Exit if the arguments are invalid
    }

    let mut path_index = 1;
    let mut option: Option<&String> = None;

    // Check if the first argument is an option
    if args[1].starts_with('-') {
        option = Some(&args[1]);
        path_index = 2; // The path to the C file will be the next argument
    }

    // Ensure the path to the C file is provided
    if path_index >= args.len() {
        eprintln!("Error: Missing path to C file.");
        process::exit(1);
    }

    let path = &args[path_index];

    // Ensure the file has a .c extension
    if !path.ends_with(".c") {
        eprintln!("Error: The file must have a .c extension.");
        process::exit(1);
    }

    // Handle the different options passed in the command line
    match option {
        Some(opt) => match opt.as_str() {
            "--lex" => {
                println!("Performing lexical analysis on {}", path);
                if let Err(e) = lexer(path) {
                    eprintln!("{}", e); // Print error if lexical analysis fails
                    process::exit(1); // Exit with an error code
                }
                process::exit(0); // Exit successfully
            }
            "--parse" => {
                println!("Performing parsing on {}", path);
                process::exit(0); // Exit after parsing (not yet implemented)
            }
            "--codegen" => {
                println!("Performing code generation on {}", path);
                process::exit(0); // Exit after code generation (not yet implemented)
            }
            "-s" => {
                let new_name = path.trim_end_matches(".c"); // Trim the .c extension
                let asm_file = format!("{}.s", new_name); // Generate the assembly file name
                match File::create(&asm_file) {
                    Ok(_) => {
                        println!("Generated assembly file: {}", asm_file); // Successfully created assembly file
                        process::exit(0); // Exit successfully
                    }
                    Err(_) => {
                        eprintln!("Error: Failed to create assembly file '{}'", asm_file); // Handle failure
                        process::exit(1); // Exit with error code
                    }
                }
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", opt); // Handle unknown options
                process::exit(1); // Exit with error code
            }
        },
        None => {} // No option provided, proceed to default behavior
    }
}

