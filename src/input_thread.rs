mod test_input{
    use super::{fs, input_step};
    #[test]
    fn input_step_check_when_file_exist() {//Как корректно тестировать функции, работающие с выводом из stdin???
        fs::File::create("check_when_file_exist.txt").expect("System error, No fn bug!");
        input_step("check_when_file_exist.txt");
        fs::remove_file("check_when_file_exist.txt").expect("System error, No fn bug!");
    }
    #[test]
    fn  input_step_check_when_file_no_exist() {//Как корректно тестировать функции, работающие с выводом из stdin???
        fs::remove_file("check_when_file_No_exist.txt").ok();//Если файла нет - значит удалять уже нечего 
        input_step("check_when_file_No_exist.txt");
    }
}
use std::{time, fs};
pub fn input_step(path : &str)
{
    if fs::File::open(path).is_ok() {
        std::thread::sleep(time::Duration::from_millis(100));
        match fs::read_to_string(path)
        {
            Ok(msg) => println!("{}", msg),
            _ =>  return,
        }
    }
    //ошибка может произойти в связи с удлением файла пользователем, не требует обработки, особых действий
    fs::remove_file(path).ok();
    //Дает время второму клиенту произвести запись в файл
    std::thread::sleep(time::Duration::from_millis(500));
}