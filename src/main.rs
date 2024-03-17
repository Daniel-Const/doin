use std::env;
use std::path::Path;
use std::fs::{File, read_to_string};
use std::process;

mod todo;

fn new(todo_list: &mut todo::TodoList, text: String) {
    todo_list.add_todo(text);
    let _ = todo_list.save_to_file();
    todo_list.print_list();
}

fn rm(todo_list: &mut todo::TodoList, id: u16) {
    todo_list.delete_todo(id);
    let _ = todo_list.save_to_file();
    todo_list.print_list();

}

fn read_doin_file() -> todo::TodoList {
    let file = read_to_string(".doin");

    let text = match file {
        Ok(text) => text,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    return todo::TodoList::from_text(text);
}

fn help() {
    println!("USAGE: doin [command]");
}

fn init() -> std::io::Result<()> {
    if !Path::new(".doin").exists() {
        let _file = File::create(".doin");
    }
    Ok(())
}

fn main() {
    let _ = init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        process::exit(1);
        // TODO: Start interactive shell here
    }
    let command = &args[1];
    let mut todo_items = read_doin_file();
    match command.as_str() {
        "ls" => todo_items.print_list(),
        "add" => new(&mut todo_items, args[2].to_string()),
        "rm" => rm(&mut todo_items, args[2].parse::<u16>().unwrap()),
        _ => println!("Invalid command"),
    }


    // dbg!(args);

}
