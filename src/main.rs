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
                tokens.push(Token::OpenParenthesis);
                chars.next();
            }
            ')' => {
                tokens.push(Token::CloseParenthesis);
                chars.next();
            }
            '{' => {
                tokens.push(Token::OpenBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::CloseBrace);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_numeric() {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Constant(num)); // Store numeric constants
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        ident.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                eprintln!("DEBUG: Found identifier '{}'", ident);
                
                if ident.chars().next().unwrap().is_numeric() {
                    return Err(format!("Lexical Error: Identifiers cannot start with a number: '{}'", ident));
                }
                
                if ident == "_" {
                    return Err(format!("Lexical Error: Standalone underscore '_' is not a valid identifier."));
                }
                
                if ident.chars().any(|ch| !(ch.is_alphanumeric() || ch == '_')) {
                    return Err(format!("Lexical Error: Invalid identifier '{}'", ident));
                }
                
                match ident.as_str() {
                    "int" => tokens.push(Token::IntKeyword),
                    "void" => tokens.push(Token::VoidKeyword),
                    "return" => tokens.push(Token::ReturnKeyword),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            '/' => {
                chars.next();
                if let Some(&next) = chars.peek() {
                    if next == '/' {
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    } else if next == '*' {
                        chars.next(); // Consume '*'
                        while let Some(_) = chars.peek() {
                            if chars.next() == Some('*') && chars.peek() == Some(&'/') {
                                chars.next(); // Consume '/'
                                break;
                            }
                        }
                    } else {
                        return Err("Lexical Error: Unhandled token '/'. Implement operator support.".to_string());
                    }
                }
            }
            _ => {
                return Err(format!("Lexical Error: Invalid character '{}'", c));
            }
        }
    }

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

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

    if args[1].starts_with('-') {
        option = Some(&args[1]);
        path_index = 2;
    }

    if path_index >= args.len() {
        eprintln!("Error: Missing path to C file.");
        process::exit(1);
    }

    let path = &args[path_index];

    if !path.ends_with(".c") {
        eprintln!("Error: The file must have a .c extension.");
        process::exit(1);
    }

    match option {
        Some(opt) => match opt.as_str() {
            "--lex" => {
                println!("Performing lexical analysis on {}", path);
                if let Err(e) = lexer(path) {
                    eprintln!("{}", e);
                    process::exit(1);
                }
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
                let new_name = path.trim_end_matches(".c");
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
                process::exit(1);
            }
        },
        None => {}
    }
}

