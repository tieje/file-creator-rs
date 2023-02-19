use std::io::{Result, stdin};

static FIRST_STRING: &str = "\nWould you like to create a file or folder?\n\nfile: press Enter\n\nfolder: type \"f\", then Enter";


fn main() -> Result<()>{
    println!("{}", FIRST_STRING);
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    let user_input: u32 = buffer.chars().nth(0).unwrap() as u32;
    // TODO: split into separate functions and create tests for each function
    println!("{}", user_input);
    // 10 is Enter, 101 is e, 102 is f
    match user_input {
        10 => println!("Enter was entered, create a file"),
        101 => println!("e was entered, end program"),
        102 => println!("f was entered, create a folder"),
        _ => println!("Neither f nor Enter was entered")
    }   
    Ok(())
}
