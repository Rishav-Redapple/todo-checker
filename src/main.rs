use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Please provide a path to a file or directory");
        return;
    }

    let path = Path::new(&args[0]);

    if !path.exists() {
        println!("The path is not a directory or does not exist");
        return;
    }

    if path.is_file() {
        read_file(path);
    } else if path.is_dir() {
        todo!("read a directory recursively")
    } else {
        println!("Could not read this dir or file");
        return;
    }
}

fn read_file(path: &Path) {
    let data = fs::read(path).expect("Cannot read file");
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    check_todos(data, filename);
}

fn check_todos(data: Vec<u8>, filename: String) {
    let mut todos: Vec<String> = vec![];
    match String::from_utf8(data) {
        Ok(s) => {
            for (line_no, line) in s.lines().enumerate() {
                if line.contains("TODO:") {
                    let index = line.find("TODO:").unwrap();
                    let todo = format!(
                        "line {}: {}",
                        line_no + 1,
                        &line[index + &"TODO:".len()..].trim_start()
                    );
                    todos.push(todo)
                }
            }
        }
        Err(e) => {
            println!("Oops some error while reading a file");
            println!("{}", e.to_string());
            return;
        }
    };
    if todos.is_empty() {
        println!("No todos left. Great work!");
    } else {
        println!("{} todos left in file \"{}\":\n", todos.len(), filename);
        for todo in todos {
            println!("{}", todo);
        }
    }
}
