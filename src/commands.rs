#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltInCommands{
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

impl BuiltInCommands{
    pub fn from_str(command: &str) -> Option<BuiltInCommands> {
        match command {
            "exit" => Some(BuiltInCommands::Exit),
            "echo" => Some(BuiltInCommands::Echo),
            "type" => Some(BuiltInCommands::Type),
            "pwd" => Some(BuiltInCommands::Pwd),
            "cd" => Some(BuiltInCommands::Cd),
            _ => None,
        }
    }
}

impl From<&str> for BuiltInCommands{
    fn from(command: &str) -> Self {
        BuiltInCommands::from_str(command).expect("Invalid command")
    }
}