use std::path::Path;
use is_executable::is_executable;
pub fn find_executable_files(command: &str) -> bool{

        let paths = std::env::var("PATH").unwrap_or_default();
        let path_dirs : Vec<&str> = paths.split(':').collect();

        for dir in path_dirs{
            let full_path = format!("{}/{}", dir, command);
            if Path::new(&full_path).exists() && Path::new(&full_path).is_file(){

                if is_executable(&full_path){
                    println!("{} is {}",command, full_path);
                    return true
                }
            }
        }

        return false

}