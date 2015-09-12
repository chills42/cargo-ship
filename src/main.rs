extern crate toml;

#[cfg(not(test))]
fn main() {
    use std::env;
    let value = read_cargo_config();
    let home_dir_bin = env::home_dir().expect("unable to determine publish directory").to_str().unwrap().to_string() + "/.bin/";

    let release_name = value.lookup("release.name");
    let default_name = value.lookup("package.name");
    let target_name = "./target/release/".to_owned() + release_name.or(default_name).expect("unable to determine target name").as_str().unwrap();

    let args = vec!["test".to_owned()];
    let exit_status = run_command("cargo".to_owned(), args);
    if !exit_status.success() {
        println!("Stopping ship due to tests.");
        exit_with_code(exit_status.code());
    }

    let args = vec!["build".to_owned(), "--release".to_owned()];
    let exit_status = run_command("cargo".to_owned(), args);

    if !exit_status.success() {
        println!("Stopping ship due to compilation error.");
        exit_with_code(exit_status.code());
    }

    let args = vec![target_name, home_dir_bin];
    println!("copying {} to {}", args[0], args[1]);
    run_command("cp".to_owned(), args);
}

#[cfg(not(test))]
fn read_cargo_config() -> toml::Value {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("Cargo.toml").unwrap();
    let mut toml = String::new();
    f.read_to_string(&mut toml).ok();

    //toml::Parser::new(&toml).parse().unwrap()
    toml.parse().unwrap()
}

#[cfg(not(test))]
fn exit_with_code(code: Option<i32>) {
    use std::process::{self};
    process::exit(code.unwrap_or(0));
}

#[cfg(not(test))]
fn run_command(command: String, args: Vec<String>) -> std::process::ExitStatus {
    use std::process::{Command};

    let mut command = Command::new(command);
    command.args(&args);
    let mut child = command.spawn().unwrap_or_else(|e| panic!("{}", e));
    child.wait().unwrap_or_else(|e| panic!("{}", e))
}
