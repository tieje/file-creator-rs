use std::io::{Result, stdin};

static FIRST_STRING: &str = "Would you like to create a file or folder?\nfile: press Enter\nfolder: type \"f\", then Enter";


fn main() -> Result<()>{
    println!("{}", FIRST_STRING);
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    let user_input: u32 = buffer.chars().nth(0).unwrap() as u32;
    // TODO: split into separate functions and create tests for each function
    println!("{}", user_input);
    // match buffer {

    // }
    Ok(())
}
