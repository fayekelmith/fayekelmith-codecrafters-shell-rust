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

pub fn process_str(input: &str) -> Vec<String>{

    let mut args: Vec<String> = Vec::new();
    let mut current_token: String = String::new();
    let mut chars = input.chars().peekable();
    let mut state = QuoteStateMachine::Normal;

    while let Some(ch) = chars.next(){
        match state {
            QuoteStateMachine::Normal => {
                match ch {
                    '\'' => state = QuoteStateMachine::InSingleQuote,
                    '"' => state = QuoteStateMachine::InDoubleQuote,
                    ' ' | '\t' => {
                        if !current_token.is_empty(){
                            args.push(current_token.clone());
                            current_token.clear();
                        }
                    },
                    '\\' => {
                        if let Some(next_ch) = chars.next(){
                            current_token.push(next_ch);
                        }
                    },
                    _ => current_token.push(ch),
                }
            }, 
            QuoteStateMachine::InSingleQuote => {
                match ch {
                    '\'' => state = QuoteStateMachine::Normal,
                    _ => current_token.push(ch),
                }
            },
            QuoteStateMachine::InDoubleQuote => {
                match ch {
                    '"' => state = QuoteStateMachine::Normal,
                    '\\' => {
                        match chars.peek(){
                            Some(&c) if c == '"' || c == '\\' || c == '$' || c == '`' => {
                                chars.next();
                                current_token.push(c);
                            },
                            _ => current_token.push('\\'),
                        }
                    },
                    _ => current_token.push(ch),
                }
            },
        }
    }
    if !current_token.is_empty(){
        args.push(current_token);
    }

    args
}