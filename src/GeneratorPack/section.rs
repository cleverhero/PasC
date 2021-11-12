use GeneratorPack::command::*;
use std::fmt;

pub struct Section {
    name: String,
    commands: Vec<Command>,
}

impl Section {
    pub fn new(name: String) -> Section {
        Section {
            name,
            commands: vec![],
        }
    }

    pub fn push(&mut self, command: Command) {
        self.commands.push(command);
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = "".to_string();
        for command in &self.commands {
            text += &command.as_str();
            text += "\n";
        }
        write!(f, "section {} \n {}", self.name, text)
    }
}
