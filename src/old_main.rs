use std::io;
use std::process::Command;
use std::process;

fn main() {
    println!("Enter the path to a C source file:");
    
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line.");
    
    let path = path.trim();

    println!("You entered: {}", path);
    
    let new_name=path.strip_suffix(".c").unwrap_or(path);


    let status = Command::new("touch")
        .arg(new_name)
        .status()
        .expect("Failed to execute touch");

    if !status.success(){
        eprintln!("Failed to create file: {}", new_name);
        process::exit(1);
    }else{
        println!("Created file: {}", new_name);
        process::exit(0);
    }

    //process::exit(0);    
}
    
