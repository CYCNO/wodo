use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// settings structure of json
#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    current_branch: String,
    available_branches: Vec<String>,
}

// it takes info from settings.json
pub fn current_branch() -> String {
    let branches_info: Vec<Settings> = branch_info();
    branches_info[0].current_branch.to_string()
}

// to locate the settings file
fn settings_file() -> PathBuf {
    let mut path = config_dir().expect("Could not find config directory");
    path.push("wodo");
    fs::create_dir_all(&path).expect("Could not create config directory");

    path.push("settings.json");

    // this will only run for the first time
    if !path.exists() {
        let contents = vec![Settings {
            current_branch: "main".into(),
            available_branches: vec!["main".into()],
        }];
        let json = serde_json::to_string_pretty(&contents).expect("Could not serialize settings");
        fs::write(&path, json).expect("Could not write settings file");
    }

    path
}

// fetch data from settings.json
fn branch_info() -> Vec<Settings> {
    let json_data = fs::read_to_string(settings_file()).unwrap_or_else(|_| "[]".to_string());
    let branches_info: Vec<Settings> =
        serde_json::from_str(&json_data).unwrap_or_else(|_| Vec::new());
    branches_info
}

// it list all the branch
pub fn show_branch() -> Result<(), Box<dyn Error>> {
    let branches_info: Vec<Settings> = branch_info();

    println!("      ------------------------------------------------");
    println!("     | {:<2} | {:<42} |", "No", "Branch");
    println!("      ------------------------------------------------");
    let mut i: u32 = 1;
    for branch in branches_info[0].available_branches.clone() {
        if branch == branches_info[0].current_branch {
            println!(
                "     | {:<2} | {:<42} |",
                i,
                format!("{} (current)", branch)
            );
        } else {
            println!("     | {:<2} | {:<42} |", i, branch);
        };
        i += 1;
    }

    println!("      ------------------------------------------------");

    Ok(())
}

// create a branch
pub fn create_branch(message: String) -> Result<(), Box<dyn Error>> {
    let mut path = config_dir().expect("Could not find config directory");
    path.push("wodo");
    fs::create_dir_all(&path).expect("Could not create config directory");
    path.push(format!("{}.json", message));

    let mut branches_info = branch_info();
    if branches_info[0].available_branches.contains(&message) {
        println!("Branch Already Exist.");
    } else {
        branches_info[0].available_branches.push(message.clone());

        let updated_json = serde_json::to_string_pretty(&branches_info)?;
        fs::write(settings_file(), updated_json)?;
        let _ = fs::write(&path, "[]");

        println!("Branch {} Have been created", message);
    };

    Ok(())
}

//switch to any branch
pub fn switch_branch(num: usize) -> Result<(), Box<dyn Error>> {
    let mut branches_info = branch_info();

    branches_info[0].current_branch = branches_info[0].available_branches[num - 1].clone();

    let updated_json = serde_json::to_string_pretty(&branches_info)?;
    fs::write(settings_file(), updated_json)?;
    println!(
        "Branch Have Been Switched to {}",
        branches_info[0].current_branch
    );
    Ok(())
}

// delete branch
pub fn delete_branch(num: usize) -> Result<(), Box<dyn Error>> {
    let mut branches_info = branch_info();
    let branch = branches_info[0].available_branches[num - 1].clone();

    if branch == branches_info[0].current_branch {
        println!("Current Branch Cannot be deleted, First switch to any other branch");
    } else {
        let mut path = config_dir().expect("Could not find config directory");
        path.push("wodo");
        fs::create_dir_all(&path).expect("Could not create config directory");
        path.push(format!("{}.json", branch));
        if path.exists() {
            let _ = fs::remove_file(&path);
            branches_info[0].available_branches.remove(num - 1);
            let updated_json = serde_json::to_string_pretty(&branches_info)?;
            fs::write(settings_file(), updated_json)?;
            println!("Branch {branch} have been deleted");
        } else {
            println!("Branch Does Not Exist");
        };
    }
    Ok(())
}
