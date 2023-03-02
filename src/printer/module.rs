use super::class::Class;
use super::enum_::Enum;
use super::export::Export;
use super::import::Import;
use protobuf::plugin::code_generator_response::File;

pub struct Module {
    filename: String,
    content: String,
    indentation: u32,
}

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

impl Module {
    pub fn new<S: Into<String>>(filename: S) -> Self {
        Self {
            filename: filename.into(),
            content: String::new(),
            indentation: 0,
        }
    }

    pub fn export<'a>(&'a mut self, names: &'a [&'a str]) -> Export<'a> {
        Export::new(self, names)
    }

    pub fn import<'a>(&'a mut self, names: &'a [&'a str]) -> Import<'a> {
        Import::new(self, names)
    }

    pub fn class(&mut self, name: impl Into<String>) -> Class {
        Class::new(self, name)
    }

    pub fn enum_(&mut self, name: impl Into<String>) -> Enum {
        Enum::new(self, name)
    }

    pub(super) fn println(&mut self, str: &str) {
        self.print(str);
        self.blank();
    }

    pub(super) fn print_indentation(&mut self) {
        for _ in 0..self.indentation {
            self.content.push(' ');
        }
    }

    pub(super) fn print(&mut self, str: &str) {
        self.content.push_str(str);
    }

    pub fn blank(&mut self) {
        self.content.push_str(LINE_ENDING);
    }

    pub(super) fn indent(&mut self) {
        self.indentation += 2;
    }

    pub(super) fn unindent(&mut self) {
        self.indentation -= 2;
    }
}

impl From<Module> for File {
    fn from(value: Module) -> Self {
        let mut file = File::new();
        file.set_name(value.filename);
        file.set_content(value.content);
        file
    }
}
