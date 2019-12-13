use std::io::{self, BufReader};
use std::fs::File;

pub fn read_input(file: &str) -> io::Result<BufReader<File>> {
    // the "?" macro is short and for return Err 
    let file_in = File::open(file)?;
    // last argument is return value, since we are returning it in a Result wrap it in Ok
    Ok(BufReader::new(file_in)) 
}