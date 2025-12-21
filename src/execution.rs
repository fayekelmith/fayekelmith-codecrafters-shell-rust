use std::path::Path;
use is_executable::is_executable;
use std::io::Write;
use std::process::Command;
use std::fs;



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

pub fn execute_cmd(cmd: &str, args: Vec<&str>)-> Result<(), std::io::Error>{
    let output = Command::new(cmd)
        .args(args)
        .output()?;
    std::io::stdout().write(&output.stdout).unwrap();
    std::io::stderr().write(&output.stderr).unwrap();
    Ok(())
}

pub fn change_directory(command: &str) -> Result<(), std::io::Error>{
    let current_dir = std::env::current_dir()?;
    if command.trim() == "" || command.trim() == "~"{
        if let Some(home_dir)  = std::env::home_dir(){
            std::env::set_current_dir(home_dir)?;
        } else {
            std::env::set_current_dir(current_dir)?;
        }
    } else{
        let path = fs::canonicalize(command.trim())?;
        std::env::set_current_dir(path)?
    }

    Ok(())
}