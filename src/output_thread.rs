use crate::output_thread::io::stdin;
use std::{ path::Path, io::{BufRead, Write}, io, fs};
use chrono;
pub fn output_step(path: &String)
{
    let mut msg = String::new();
    if stdin().lock().read_line(&mut msg).is_err() {return;}
    
    loop
    {
        match fs::File::create(Path::new(path))
        {
            Ok(mut f) => {
                match write!(f, "{} [{}]", msg, chrono::Local::now())
                {
                    Ok(_var) => break,
                    Err(err) => { println!("{}", err); continue;  },
                };
            },
            Err(err) => {  println!("{}", err); continue; },
        };
    }
}