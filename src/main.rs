use clap::{Arg, Command};
use remember_me::RememberMe;

fn main() {
    let rememberme = RememberMe::new().expect("Couldn't create the todo instance");

    let matches = Command::new("RememberMe")
        .version("1.1")
        .author("Hendrick Sumeck <hsumeck@gmail.com>")
        .arg(Arg::new("command").short('c').long("command"))
        .get_matches();

    if let Some(command) = matches.get_one::<String>("command") {
        match command.as_str() {
            "asdf" | "conda" | "docker" | "dotnet" | "general_linux" | "geral" | "git" | "kubectl" | "linux" | "wsl" => rememberme.read_and_print_file(command),
            "help" | "--help" | "-h" | _ => rememberme.help(),
        }
    } else {
        rememberme.help();
    }
}
