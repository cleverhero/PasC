use std::fmt;
use GeneratorPack::section::*;
use GeneratorPack::command::*;

pub struct Generator {
    pub data_section: Section,
    pub text_section: Section,

    pub last_format: i32,
}

impl Generator {
    pub fn new() -> Generator {
        let data_section = Section::new(".data".to_string());
        let text_section = Section::new(".text".to_string());

        Generator {
            data_section,
            text_section,
            last_format: 0,
        }
    }

    pub fn push_to_data_section(&mut self, command: Command) {
        self.data_section.push(command);
    }

    pub fn push_to_text_section(&mut self, command: Command) {
        self.text_section.push(command);
    }

    pub fn next_format(&mut self) -> String {
        self.last_format += 1;
        "fmt_".to_string() + &(self.last_format - 1).to_string()
    }
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "extern _printf \n{} \n{}",
            self.data_section, self.text_section
        )
    }
}
