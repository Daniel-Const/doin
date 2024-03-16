use std::env;
use std::path::Path;
use std::fs::{File, read_to_string};
use std::process;

struct Todo {
    id: u16,
    text: String,
}

fn new() {
    // Create a new TODO item
}

fn rm() {
    // Delete a TODO item
}

fn save_todo_list() {
    // Overwrite .doin file with the edited todo list items
}



fn read_doin_file() -> Vec<Todo> {
    let file = read_to_string(".doin");

    let text = match file {
        Ok(text) => text,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let mut todo_vec: Vec<Todo> = vec![];
    for line in text.split('\n').into_iter() {
        let parts: Vec<&str> = line.split('.').collect();
        if parts.len() < 2 {
            continue
        }
        todo_vec.push(Todo {
            id: parts[0].parse::<u16>().unwrap(),
            text: parts[1].trim().to_string()
        });
    }

    return todo_vec;
}

fn init() -> std::io::Result<()> {
    if !Path::new(".doin").exists() {
        let _file = File::create(".doin");
    }
    Ok(())
}

fn main() {
    let _ = init();
    let _args: Vec<String> = env::args().collect();
    // dbg!(args);
    let todo_items = read_doin_file();
    println!("~ Todo items ~");
    for item in todo_items {
        println!("{0}: {1}", item.id, item.text);
    }
}
