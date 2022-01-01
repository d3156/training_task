use crate::output_thread::io::stdin;
use std::{ path::Path, io::{BufRead, Write}, io, fs};
use chrono;
#[cfg(test)]
mod test{
    use super::{fs, output_step};
    #[test]
    fn output_step_check_when_file_exist() {//Как корректно тестировать функции, работающие с пользовательским вводом из stdin???
        fs::File::create("check_when_file_exist.txt").expect("System error, No fn bug!");
        output_step("check_when_file_exist.txt");
        fs::remove_file("check_when_file_exist.txt").expect("System error, No fn bug!");
    }
    #[test]
    fn output_step_check_when_file_no_exist() {//Как корректно тестировать функции, работающие с пользовательским вводом из stdin???
        fs::remove_file("check_when_file_No_exist.txt").ok();//Если файла нет - значит удалять уже нечего 
        output_step("check_when_file_No_exist.txt");
    }
}
pub fn output_step(path: &str)
{
    let mut msg = String::new();
    if stdin().lock().read_line(&mut msg).is_err() {return;}
    loop
    {
        match fs::File::create(Path::new(path))
        {
            Ok(mut f) => {
                match write!(f, "{} [{}]", msg.trim(), chrono::Local::now())
                {
                    Ok(_var) => break,
                    Err(err) => { println!("{}", err); continue;  },
                };
            },
            Err(err) => {  println!("{}", err); continue; },
        };
    }
}