
use std::{time, io, fs};

pub fn input_step(s : &String) -> io::Result<()>
{
    let msg = fs::read_to_string(s)?;
    println!("{}", msg);
    std::thread::sleep(time::Duration::from_millis(500));
    Ok(())
}