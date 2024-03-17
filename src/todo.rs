use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct TodoList {
    todo_map: HashMap<u16, String>,
    largest_id: u16,
    filename: String,
}

impl TodoList {
    pub fn from_text(text: String) -> TodoList {
        let mut todo_map = HashMap::new();
        let mut largest_id = 0;
        for line in text.split('\n').into_iter() {
            let parts: Vec<&str> = line.split('.').collect();
            if parts.len() < 2 {
                continue;
            }
            let id = parts[0].parse::<u16>().unwrap();
            if id > largest_id {
                largest_id = id;
            }
            todo_map.insert(
                parts[0].parse::<u16>().unwrap(),
                parts[1].trim().to_string()
            );
        }

        return TodoList {todo_map, largest_id, filename: ".doin".to_string()}
    }

    pub fn to_string(&self) -> String {
        let mut value = String::new();
        for (id, text) in &self.todo_map {
            value += &String::from(format!("{id}.{text}\n"));
        }
        return value.to_string();
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new().write(true).truncate(true).open(self.filename.to_string())?;
        let _ = file.write(self.to_string().as_bytes());
        Ok(())
    }

    pub fn delete_todo(&mut self, id: u16) {
         self.todo_map.remove(&id);
    }

    pub fn add_todo(&mut self, text: String) {
        self.todo_map.insert(
            self.largest_id + 1,
            text
        );
        self.largest_id += 1;
    }

    pub fn print_list(&self) {
        println!("~ TODO ~");
        for (id, text) in &self.todo_map {
            println!("{0}: {1}", id, text);
        }
    }
}
