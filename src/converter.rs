//! # converter
//!
//! The module which converts the shell script to windows batch script.
//!

#[cfg(test)]
#[path = "./converter_test.rs"]
mod converter_test;

fn convert_line(line: &str) -> String {
    if line.starts_with("#") {
        let mut windows_command = String::from(line);
        windows_command.remove(0);
        windows_command.insert_str(0, "@REM ");

        windows_command
    } else {
        // assume first word is the command
        let (shell_command, arguments) = match line.find(" ") {
            None => (line, ""),
            Some(space_index) => line.split_at(space_index),
        };

        let mut windows_command = match shell_command {
            "cp" => "xcopy".to_string(),
            "mv" => "move".to_string(),
            "ls" => "dir".to_string(),
            "rm" => "del".to_string(),
            "clear" => "cls".to_string(),
            "grep" => "find".to_string(),
            "pwd" => "chdir".to_string(),
            "export" => "set".to_string(),
            _ => shell_command.to_string(),
        };

        windows_command.push_str(arguments);

        windows_command
    }
}

/// Converts the provided shell script and returns the windows batch script text.
pub fn run(script: &str) -> String {
    let lines: Vec<&str> = script.split('\n').collect();
    let mut windows_batch = vec![];

    for mut line in lines {
        line = line.trim();
        let mut line_string = line.to_string();

        // convert line
        let converted_line = if line_string.len() == 0 {
            line_string
        } else {
            convert_line(&mut line_string)
        };

        windows_batch.push(converted_line);
    }

    windows_batch.join("\n")
}
