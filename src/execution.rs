use std::path::Path;
use is_executable::is_executable;
use std::io::Write;
use std::process::Command;
use anyhow::{Result, Context};

enum QuoteStateMachine{
    Normal,
    InSingleQuote,
    InDoubleQuote,
}


pub fn is_executable_cmd(command: &str) -> (bool, String) {

        let paths = std::env::var("PATH").unwrap_or_default();
        let path_dirs : Vec<&str> = paths.split(':').collect();

        for dir in path_dirs{
            let full_path = format!("{}/{}", dir, command);
            if Path::new(&full_path).exists() && Path::new(&full_path).is_file(){

                if is_executable(&full_path){
                    return (true, full_path);
                }
            }
        }

        return (false, String::new());
}

pub fn execute_cmd(cmd: &str, args: Vec<&str>)-> Result<()>{
    let output = Command::new(cmd)
        .args(args)
        .output()?;
    std::io::stdout().write(&output.stdout).unwrap();
    std::io::stderr().write(&output.stderr).unwrap();
    Ok(())
}

pub fn change_directory(command: &str) -> Result<()>{
    if command.trim() == "" || command.trim() == "~"{
        if let Some(home_dir)  = std::env::home_dir(){
            std::env::set_current_dir(home_dir).context("No such file or directory")?
        }
    } else{
        if let Err(_) = std::env::set_current_dir(command.trim()) {
            return Err(anyhow::anyhow!("{}: No such file or directory", command.trim()));
        }
    }
    Ok(())
}

pub fn process_echo_str(input: &str) -> String{
    let mut result: String = String::new();
    let mut state = QuoteStateMachine::Normal;
    let mut was_last_char_space = false;

    for ch in input.chars(){
        if ch == '\''{
            match state {
                QuoteStateMachine::Normal => state = QuoteStateMachine::InSingleQuote,
                QuoteStateMachine::InSingleQuote => state = QuoteStateMachine::Normal,
                QuoteStateMachine::InDoubleQuote => result.push(ch),
            }
        }
        else if ch == '\"'{
            match state{
                QuoteStateMachine::InDoubleQuote => state = QuoteStateMachine::Normal,
                QuoteStateMachine::InSingleQuote => result.push(ch),
                QuoteStateMachine::Normal => state = QuoteStateMachine::InDoubleQuote,
            }
        }else if ch.is_whitespace(){
            match state{
                QuoteStateMachine::Normal => {
                    if !was_last_char_space{
                        result.push(' ');
                        was_last_char_space = true;
                    }
                },
                _ => {
                    result.push(ch);
                    was_last_char_space = false;
                }
            }
        }
        else{
            result.push(ch);
            was_last_char_space = false;
        }
    }

    result
}