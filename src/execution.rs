use std::path::Path;
use is_executable::is_executable;
use std::io::Write;
use std::process::Command;
use anyhow::{Result, Context};
use std::fs::OpenOptions;

enum QuoteStateMachine{
    Normal,
    InSingleQuote,
    InDoubleQuote,
}

#[derive(Debug, Clone)]
pub struct CommandResult{
    pub clean_args: Vec<String>,
    pub stdout_redirect: Vec<(String, bool)>, // (file_path, append)
    pub stderr_redirect: Vec<(String, bool)>, // (file_path, append)
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

pub fn parse_str(args: Vec<String>) -> CommandResult{
      let operations = vec![
        (">", false, 1),
        ("1>", false, 1),
        (">>", true, 1),
        ("1>>", true, 1),
        ("2>>", true, 2),
        ("2>", false, 2),
    ];
    let mut clean_args =  Vec::new();
    let mut stdout_redirect: Vec<(String, bool)> = Vec::new(); // (file_path, append)
    let mut stderr_redirect: Vec<(String, bool)> = Vec::new(); // (file_path, append)
    let mut i = 0;

    while i < args.len(){
        let mut matches = false;
        let current_arg = &args[i];
        for (op, append, fd) in &operations{
            if args[i] == *op{
                if let Some(file_path) = args.get(i+1){
                    let entry = (file_path.to_string(), *append);
                    if *fd ==1{
                        stdout_redirect.push(entry);
                    } else{
                        stderr_redirect.push(entry);
                    }
                    i +=2;
                    matches = true;
                    break;
                }
            }
        }
        if !matches{
            clean_args.push(current_arg.to_string());
            i +=1;
        }
    }
    CommandResult{
        clean_args,
        stdout_redirect,
        stderr_redirect,
    }
}

pub fn handler_output(output: Vec<u8>, redirects:&mut Vec<(String, bool)>, is_stdout: bool) -> Result<()>{
    if let Some((last_path, last_append)) = redirects.pop(){
        for (path, append) in redirects{
            if let Some(parent) = Path::new(path).parent() {
                std::fs::create_dir_all(parent)?;
            }
            OpenOptions::new().write(true).create(true).append(*append).truncate(!*append).open(path)?;
        }
        if let Some(parent) = Path::new(&last_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new().write(true).create(true).append(last_append).truncate(!last_append).open(&last_path)?;
        file.write_all(&output)?;
    }else{
        if is_stdout{
            std::io::stdout().write_all(&output)?;
            std::io::stdout().flush()?;
        }else{
            std::io::stderr().write_all(&output)?;
            std::io::stderr().flush()?;
        }
    }
    Ok(())
}

pub fn execute_cmd(cmd: &str, commands: CommandResult)-> Result<()>{
    let CommandResult { clean_args, mut stdout_redirect, mut stderr_redirect } = commands;
    let output = Command::new(cmd)
        .args(&clean_args[1..]) //skip index 0, which is already cmd
        .output()?;
    handler_output(output.stdout, &mut stdout_redirect, true)?;
    handler_output(output.stderr, &mut stderr_redirect, false)?;
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