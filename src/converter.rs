//! # converter
//!
//! The module which converts the shell script to windows batch script.
//!

#[cfg(test)]
#[path = "./converter_test.rs"]
mod converter_test;

use regex::Regex;

fn replace_flags(
    arguments: &str,
    flags_mappings: Vec<(&str, &str)>,
) -> String {
    let mut windows_arguments = arguments.clone().to_string();

    if flags_mappings.len() > 0 {
        for flags in flags_mappings {
            let (shell_flag, windows_flag) = flags;

            windows_arguments = match Regex::new(shell_flag) {
                Ok(shell_regex) => {
                    let str_value = &shell_regex.replace_all(&windows_arguments, windows_flag);
                    str_value.to_string()
                }
                Err(_) => windows_arguments,
            };
        }
    }

    windows_arguments
}

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

        let (mut windows_command, flags_mappings) = match shell_command {
            "cp" => ("xcopy".to_string(), vec![("-[rR]", "/E")]),
            "mv" => ("move".to_string(), vec![]),
            "ls" => ("dir".to_string(), vec![]),
            "rm" => ("del".to_string(), vec![("-[rR]*[fF][rR]*", "/Q"), ("-[rR]+ ", " ")]),
            "mkdir" => ("mkdir".to_string(), vec![("-[pP]", "")]),
            "clear" => ("cls".to_string(), vec![]),
            "grep" => ("find".to_string(), vec![]),
            "pwd" => ("chdir".to_string(), vec![]),
            "export" => ("set".to_string(), vec![]),
            _ => (shell_command.to_string(), vec![]),
        };

        // replace flags
        let windows_arguments = replace_flags(arguments, flags_mappings);

        windows_command.push_str(&windows_arguments);

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
