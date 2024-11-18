mod mods;
use mods::{file_actions, file_manager, file_utility};

fn run_program() -> Result<(), String> {
    loop {
        println!("\nAvailable commands:");
        println!("1. Print files");
        println!("2. Create files");
        println!("3. Read file");
        println!("4. Delete file");
        println!("5. Exit program\n");

        print!("Enter command: ");
        let _ = file_utility::stdout_flush()?;

        let user_input = match file_utility::read_entry() {
            Err(err) => {
                println!("Unable to continue program. {}", err);
                continue;
            }
            Ok(content) => content,
        };

        let command_res = match file_actions::validate_command(&user_input) {
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
            Ok(command) => command,
        };

        match command_res {
            file_actions::Commands::PrintFiles => file_manager::print_files()?,
            file_actions::Commands::CreateFiles => file_manager::create_files()?,
            file_actions::Commands::ReadFile => file_manager::read_file()?,
            file_actions::Commands::DeleteFile => file_manager::delete_file()?,
            file_actions::Commands::ExitProgram => {
                println!("Exiting program...");
                return Ok(());
            }
        }
    }
}

fn main() {
    match run_program() {
        Err(err) => panic!("ERROR {:?}", err),
        Ok(()) => {}
    }
}
