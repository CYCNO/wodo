use crate::branch::current_branch;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// How todos will look like
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    message: String,
    done: bool,
}

// it will see what is the current branch and will take its file
fn get_todo_file_path() -> PathBuf {
    let mut path = config_dir().expect("Could not find config directory");
    path.push("wodo");
    fs::create_dir_all(&path).expect("Could not create config directory");
    let current = current_branch();
    path.push(format!("{}.json", current));
    path
}

// it is the main priting of todos
pub fn show_todo() -> Result<(), Box<dyn Error>> {
    let file_path = get_todo_file_path();
    let json_data = fs::read_to_string(&file_path).unwrap_or_else(|_| "[]".to_string());
    let todos: Vec<Todo> = serde_json::from_str(&json_data)?;
    let current = current_branch();
    let message = format!("Message ({})", current);

    println!("      ------------------------------------------------------");
    println!("     | {:<2} | {:<42} | {:<3} |", "No", message, "Box");
    println!("      ------------------------------------------------------");
    let mut i: u32 = 1;
    for todo in todos {
        let check_box = if todo.done { "[x]" } else { "[ ]" };
        println!(
            "     | {:<2} | {:<42} | {:<3} |",
            &i, todo.message, check_box
        );
        i += 1;
    }
    println!("      ------------------------------------------------------");

    Ok(())
}

// it save todos to the branch
pub fn save_todo(data: String) -> Result<(), Box<dyn Error>> {
    let file_path = get_todo_file_path();
    let json_data = fs::read_to_string(&file_path).unwrap_or_else(|_| "[]".to_string());

    let mut todos: Vec<Todo> = serde_json::from_str(&json_data)?;

    let new_todo = Todo {
        message: data,
        done: false,
    };

    todos.push(new_todo);

    let updated_json = serde_json::to_string_pretty(&todos)?;
    fs::write(&file_path, updated_json)?;

    println!("todo added.");

    Ok(())
}

// it to check the todo when you are done with the work
pub fn done(n: usize) -> Result<(), Box<dyn Error>> {
    let file_path = get_todo_file_path();
    let json_data = fs::read_to_string(&file_path).unwrap_or_else(|_| "[]".to_string());
    let mut todos: Vec<Todo> = serde_json::from_str(&json_data)?;

    if let Some(todo) = todos.get_mut(n - 1) {
        todo.done = true;
    } else {
        println!("Invalid Number: {}", n);
        return Ok(());
    }

    let updated_json = serde_json::to_string_pretty(&todos)?;
    fs::write(&file_path, updated_json)?;

    println!("todo checked.");

    Ok(())
}

// delete the todo
pub fn delete(n: usize) -> Result<(), Box<dyn Error>> {
    let file_path = get_todo_file_path();
    let json_data = fs::read_to_string(&file_path).unwrap_or_else(|_| "[]".to_string());
    let mut todos: Vec<Todo> = serde_json::from_str(&json_data)?;

    if todos.get(n - 1).is_some() {
        todos.remove(n - 1);
    } else {
        println!("Invalid Number: {}", n);
        return Ok(());
    }

    let updated_json = serde_json::to_string_pretty(&todos)?;
    fs::write(&file_path, updated_json)?;

    println!("todo deleted.");

    Ok(())
}

//help command
pub fn help() {
    println!("     Wodo CLI - Commands:");
    println!("       add <message>     ➜ Add a new todo item");
    println!("       show              ➜ Show all todos");
    println!("       done <number>     ➜ Mark todo item as done");
    println!("       delete <number>   ➜ Delete a todo item");
    println!("       help              ➜ Show this help message");
    println!("                   ---                          ");
    println!("     Branch Related Commands: ");
    println!("       branch create <name>      ➜ Add a new branch");
    println!("       branch show               ➜ Show all the branches");
    println!("       branch switch <number>    ➜ Switch to other branches");
    println!("       branch delete <number>    ➜ Delete a branch");
}
