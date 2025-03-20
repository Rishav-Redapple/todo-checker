use std::{
    env,
    fs::{self},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let f = &args[0];

    match fs::read(args[0].to_string()) {
        Ok(read_dir) => read_file(read_dir, f.to_string()),
        Err(_) => println!("Could not read file `{}`. Does this file exist?", &f),
    }
}

fn read_file(data: Vec<u8>, filename: String) {
    let mut todos: Vec<String> = vec![];
    match String::from_utf8(data) {
        Ok(s) => {
            for (line_no, line) in s.lines().enumerate() {
                if line.contains("TODO:") {
                    let todo_index = line.find("TODO:");
                    match todo_index {
                        Some(index) => {
                            let str = format!("line {}: {}", line_no + 1, &line[index + 6..]);
                            todos.push(str)
                        }
                        None => println!("{}", &line.trim_start()),
                    }
                }
            }
        }
        Err(e) => {
            println!("Oops some error while reading a file");
            println!("{}", e.to_string());
        }
    };
    if todos.is_empty() {
        println!("No todos left. Great work!");
    } else {
        println!("{} todos left in file `{}`", todos.len(), filename);
        for todo in todos {
            println!("{}", todo);
        }
    }
}
