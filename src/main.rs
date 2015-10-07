extern crate toml;

#[cfg(not(test))]
fn main() {
    let home_dir_bin = get_ship_dir();
    let target_name = build_target_name();

    let args = vec!["test".to_owned()];
    let test_exit_status = run_command("cargo".to_owned(), args);
    if !test_exit_status.success() {
        println!("Stopping ship due to tests.");
        exit_with_code(test_exit_status.code());
    }

    let build_args = vec!["build".to_owned(), "--release".to_owned()];
    let build_exit_status = run_command("cargo".to_owned(), build_args);
    if !build_exit_status.success() {
        println!("Stopping ship due to compilation error.");
        exit_with_code(build_exit_status.code());
    }

    let cp_args = vec![target_name, home_dir_bin];
    println!("copying {} to {}", cp_args[0], cp_args[1]);
    run_command("cp".to_owned(), cp_args);
}

#[cfg(not(test))]
fn build_target_name() -> String {
    let value = read_cargo_config();
    let binary_name = value.lookup("bin.0.name");
    let release_name = value.lookup("release.name");
    let default_name = value.lookup("package.name");
    "./target/release/".to_owned() +
    binary_name.or(release_name)
               .or(default_name)
               .expect("unable to determine target name")
               .as_str()
               .unwrap()
}

#[cfg(not(test))]
fn get_ship_dir() -> String {
    use std::env;
    env::home_dir().expect("unable to determine publish directory").to_str().unwrap().to_owned() +
    "/.bin/"
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
    use std::process;
    process::exit(code.unwrap_or(0));
}

#[cfg(not(test))]
fn run_command(command: String, args: Vec<String>) -> std::process::ExitStatus {
    use std::process::Command;

    let mut command = Command::new(command);
    command.args(&args);
    let mut child = command.spawn().unwrap_or_else(|e| panic!("{}", e));
    child.wait().unwrap_or_else(|e| panic!("{}", e))
}
