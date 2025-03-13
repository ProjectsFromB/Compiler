use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

#[derive(Debug)]
enum Token {
    Identifier(String),
    Constant(String),
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

fn lexer(file_path: &str) -> Result<(), String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(format!("Error: Could not open file '{}'.", file_path));
        }
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return Err(format!("Error: Could not read file '{}'.", file_path));
    }

    let mut tokens = Vec::new();
    let mut chars = contents.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' => {
                chars.next();
            } // Ignore whitespace
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
                tokens.push(Token::Constant(num));
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

                // Debugging: Print each identifier being checked
                eprintln!("DEBUG: Found identifier '{}'", ident);

                // Check for invalid identifiers
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
                        // Single-line comment, consume until newline
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    } else if next == '*' {
                        // Multi-line comment, consume until `*/`
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
    let args: Vec<String> = env::args().collect();

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
                    process::exit(1);  // Ensure failure is indicated by exit code 1
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
                eprintln!("Use --lex, --parse, --codegen, or -s.");
                process::exit(1);
            }
        },
        None => {}
    }
}

