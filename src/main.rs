
mod input_thread;
mod output_thread;
use std::fs::File;
use std::io::{ErrorKind, BufRead};
fn main() {
    println!("Please type input file path:");
    let mut inp_file = String::new();
    std::io::stdin().lock().read_line(&mut inp_file).unwrap();
    println!("Please type output file path:");
    let mut out_file = String::new();
    std::io::stdin().lock().read_line(&mut out_file).unwrap();
    std::thread::spawn(move || {
        loop{
            input_thread::input_step(&inp_file.trim());
        }
    });
    std::thread::spawn(move || {
        loop{
            output_thread::output_step(&out_file.trim());
        }
    });
    loop
    {
        match File::open("X.txt")
        {
            Ok(_file) => break,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => continue,
                _ => break,
            }
        }
    }
    println!("Thank's for use our program");
}
