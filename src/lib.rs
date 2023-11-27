use std::fs::File;
use std::io::{self, Read};
use std::fs;

const HELP: &str = "
commands:
  Os seguintes comandos integrados ao remember_me estão disponíveis.

  COMMAND
    remember-me @file
";

pub struct RememberMe {
    //pub remember_me: Vec<String>,
    pub remember_me_path: String,
}

impl RememberMe {
    pub fn new() -> Result<Self, String> {
        let remember_me_path: String = String::from("D:/Back/Rust/remember-me/target/release/txt/");

        Ok(Self {
            remember_me_path
        })
    }

    pub fn read_and_print_file(&self, file_name: &str) {
        let extension: &str = ".txt";
        let remember_me_path : &String = &self.remember_me_path;

        let file_path = format!("{remember_me_path}{file_name}{extension}");

        let mut file = File::open(file_path).expect("Failed to open the file");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read the file");

        println!("{}", contents);
    }

    pub fn help(&self) {
        let mut files: Vec<String> = Vec::new();
        let entries = fs::read_dir(&self.remember_me_path).expect("Failed to read the directory");

        for entry in entries {
            let entry = entry.expect("Failed to get entry");

            if entry.file_type().expect("Failed to get file type").is_file() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                let file_name_without_extension = file_name_str.trim_end_matches(".txt");

                files.push(file_name_without_extension.to_string());
            }
        }
        let result = replace_names(HELP, files);

        println!("{}", result);
    }
}

fn replace_names(help: &str, names: Vec<String>) -> String {
    let mut result = String::from(help);

    let replacement = names.join("\n    remember-me ");
    result = result.replace("@file", &format!("{}\n", replacement));

    result
}