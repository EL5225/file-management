use std::io::{self, Write};

pub fn read_entry() -> Result<String, String> {
    let mut input = String::new();

    let read_res = io::stdin().read_line(&mut input);

    let content = match read_res {
        Ok(_) => input.trim().to_string(),
        Err(err) => {
            return Err(err.to_string());
        }
    };

    Ok(content)
}

pub fn stdout_flush() -> Result<(), String> {
    match io::stdout().flush() {
        Err(err) => Err(err.to_string()),
        Ok(()) => Ok(()),
    }
}
