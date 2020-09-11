use std::process::{Command, exit};
use std::env;

fn main() {
    let args = env::args().skip(1).map(|arg| {
        if arg.starts_with("\\\\wsl$\\") {
            let path = arg.strip_prefix("\\\\wsl$\\").unwrap();
            let start = match path.find("\\") {
                None | Some(0) => panic!("invalid WSL UNC path: {}", arg),
                index => index.unwrap()
            };
            return path[start..].replace("\\", "/");
        }
        return arg;
    });

    let status = Command::new("wsl.exe")
        .args(args)
        .status()
        .expect("failed to execute wsl.exe");

    match status.code() {
        Some(code) => exit(code),
        None => println!("wsl.exe process terminated by signal")
    }
}
