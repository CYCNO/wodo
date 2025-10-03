// =============================================
// Author: CYCNO
// For more info about code see contribution.md
// =============================================

use std::env::args;
mod actions;
mod branch;
use actions::{delete, done, help, save_todo, show_todo};
use branch::{create_branch, delete_branch, show_branch, switch_branch};

// normal(todos) functionalities
enum Actions {
    Show(),
    Add(String),
    Done(usize),
    Delete(usize),
    Help(),
}
//branches functionalities
enum Branches {
    Create(String),
    Switch(usize),
    Show(),
    Delete(usize),
}

//actions
fn action_here(action: Actions) {
    match action {
        Actions::Show() => show_todo().expect("Did not able to get todos."),
        Actions::Add(msg) => save_todo(msg).expect("Did Not Saved Todo"),
        Actions::Done(num) => done(num).expect("Did not able to check it"),
        Actions::Delete(num) => delete(num).expect("Did not able to delete"),
        Actions::Help() => help(),
    }
}

//branches
fn branches(branch: Branches) {
    match branch {
        Branches::Create(message) => create_branch(message).expect("Not able to create branch"),
        Branches::Switch(num) => switch_branch(num).expect("Unable to switch Branch"),
        Branches::Show() => show_branch().expect("Not able to Show Branch"),
        Branches::Delete(num) => delete_branch(num).expect("Not able to delete Branch"),
    }
}

fn main() {
    // this take all the args and store in list(vec)
    let args: Vec<String> = args().collect();
    let command = &args[1];

    //logic of command
    match command.as_str() {
        "show" => {
            action_here(Actions::Show());
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Missing arguments for `add`. Usage: wodo add <message>");
            } else {
                let message = args[2..].join(" ");
                action_here(Actions::Add(message));
            }
        }
        "done" => {
            if args.len() < 3 {
                eprintln!("Missing arguments for `done`. Usage: wodo done <task_id>");
            } else {
                match args[2].parse::<usize>() {
                    Ok(num) => action_here(Actions::Done(num)),
                    Err(_) => eprintln!("Invalid task id: {}", args[2]),
                }
            }
        }
        "delete" => {
            if args.len() < 3 {
                eprintln!("Missing arguments for `delete`. Usage: wodo delete <task_id>");
            } else {
                match args[2].parse::<usize>() {
                    Ok(num) => action_here(Actions::Delete(num)),
                    Err(_) => eprintln!("Invalid task id: {}", args[2]),
                }
            }
        }
        "branch" => {
            if args.len() < 3 {
                eprintln!("Missing arguments for `branch`. Usage: wodo branch <subcommand> [args]");
            } else {
                match args[2].as_str() {
                    "show" => branches(Branches::Show()),
                    "create" => {
                        if args.len() < 4 {
                            eprintln!("Missing branch name. Usage: wodo branch create <name>");
                        } else {
                            let message = args[3..].join(" ");
                            branches(Branches::Create(message));
                        }
                    }
                    "switch" => {
                        if args.len() < 4 {
                            eprintln!("Missing branch id. Usage: wodo branch switch <id>");
                        } else {
                            match args[3].parse::<usize>() {
                                Ok(num) => branches(Branches::Switch(num)),
                                Err(_) => eprintln!("Invalid branch id: {}", args[3]),
                            }
                        }
                    }
                    "delete" => {
                        if args.len() < 4 {
                            eprintln!("Missing branch id. Usage: wodo branch delete <id>");
                        } else {
                            match args[3].parse::<usize>() {
                                Ok(num) => branches(Branches::Delete(num)),
                                Err(_) => eprintln!("Invalid branch id: {}", args[3]),
                            }
                        }
                    }
                    _ => {
                        eprintln!(
                            "Unknown Branch Command: {}\n`wodo help` for valid commands.",
                            args[2]
                        );
                    }
                }
            }
        }
        "help" => {
            action_here(Actions::Help());
        }
        _ => {
            eprintln!(
                "Unknown Command: {}\n`wodo help` for valid commands.",
                command
            );
        }
    }
}
