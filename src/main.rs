use std::env;
use remember_me::{RememberMe};


fn main() {
    let rememberme = RememberMe::new().expect("Couldn't create the todo instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "asdf" => rememberme.read_and_print_file(command),
            "conda" => rememberme.read_and_print_file(command),
            "docker" => rememberme.read_and_print_file(command),
            "dotnet" => rememberme.read_and_print_file(command),
            "general_linux" => rememberme.read_and_print_file(command),
            "geral" => rememberme.read_and_print_file(command),
            "git" => rememberme.read_and_print_file(command),
            "kubectl" => rememberme.read_and_print_file(command),
            "linux" => rememberme.read_and_print_file(command),
            "wsl" => rememberme.read_and_print_file(command),
            "help" | "--help" | "-h" | _ => rememberme.help()
        }
    } else {
        rememberme.help();
    }
}
