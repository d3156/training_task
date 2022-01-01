use std::{path::Path, time, fs};
pub fn input_step(path : &String)
{
    match fs::read_to_string(Path::new(path))
    {
        Ok(msg) => println!("{}", msg),
        _ => return,
    }
    std::thread::sleep(time::Duration::from_millis(500));
}