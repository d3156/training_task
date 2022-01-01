use std::{io::BufRead, io, fs};
pub fn output_step(s: &String) -> io::Result<()>
{
    let mut msg = String::new();
    std::io::stdin().lock().read_line(&mut msg)?;
    loop
    {
        match fs::write(&s, &msg)
        {
            Ok(_var) => break,
            Err(_err) => continue,
        };
    }
    Ok(())
}