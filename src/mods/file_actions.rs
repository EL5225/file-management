pub const FOLDER_BASEPATH: &str = "./files";

pub enum Commands {
    PrintFiles,
    CreateFiles,
    ReadFile,
    DeleteFile,
    ExitProgram,
}

pub fn validate_command(cmd: &str) -> Result<Commands, &'static str> {
    match cmd {
        "1" => Ok(Commands::PrintFiles),
        "2" => Ok(Commands::CreateFiles),
        "3" => Ok(Commands::ReadFile),
        "4" => Ok(Commands::DeleteFile),
        "5" => Ok(Commands::ExitProgram),
        _ => Err("unrecognized command"),
    }
}
