use crate::mods::file_utility::{self, stdout_flush};

use super::file_actions;
use std::{fs, path::Path};

fn check_dir(path: &Path) -> Result<(), String> {
    if !path.is_dir() {
        match fs::create_dir(&path) {
            Err(err) => return Err(err.to_string()),
            Ok(_) => {
                println!("Directory created.\n");
            }
        }
    }

    Ok(())
}

pub fn print_files() -> Result<(), String> {
    let path = Path::new(file_actions::FOLDER_BASEPATH);

    check_dir(path)?;

    let dir = match fs::read_dir(&path) {
        Err(err) => return Err(err.to_string()),
        Ok(dir) => dir,
    };

    let mut count = 0;

    for file in dir {
        count += 1;

        match file {
            Err(err) => return Err(err.to_string()),
            Ok(entry) => {
                println!("-> {:?}", entry.file_name());
            }
        }
    }

    if count == 0 {
        println!("No files found.");
    }

    Ok(())
}

pub fn create_files() -> Result<(), String> {
    let path = Path::new(file_actions::FOLDER_BASEPATH);

    check_dir(path)?;

    print!("Enter file name: ");
    let _ = file_utility::stdout_flush()?;

    let filenames = match file_utility::read_entry() {
        Err(err) => return Err(err),
        Ok(content) => content,
    };

    print!("Enter file content: ");
    let _ = file_utility::stdout_flush()?;

    let content = match file_utility::read_entry() {
        Err(err) => return Err(err),
        Ok(content) => content,
    };

    match fs::write(&path.join(filenames), &content) {
        Err(err) => return Err(err.to_string()),
        Ok(_) => {}
    }

    Ok(())
}

pub fn read_file() -> Result<(), String> {
    let path = Path::new(file_actions::FOLDER_BASEPATH);

    check_dir(path)?;

    print!("Enter file name: ");
    let _ = stdout_flush()?;

    let filename = match file_utility::read_entry() {
        Err(err) => return Err(err),
        Ok(content) => content,
    };

    let content = match fs::read_to_string(path.join(filename)) {
        Err(err) => {
            println!("Error: {}", err.to_string());
            return Ok(());
        }
        Ok(content) => content,
    };

    println!("file content -> {}", content);

    Ok(())
}

pub fn delete_file() -> Result<(), String> {
    let path = Path::new(file_actions::FOLDER_BASEPATH);

    check_dir(path)?;

    print!("Enter file name: ");
    let _ = stdout_flush()?;

    let filename = match file_utility::read_entry() {
        Err(err) => return Err(err),
        Ok(content) => content,
    };

    match fs::remove_file(path.join(filename)) {
        Err(err) => {
            println!("Error: {}", err.to_string());
            return Ok(());
        }
        Ok(_) => {
            println!("File deleted.");
        }
    };

    Ok(())
}
