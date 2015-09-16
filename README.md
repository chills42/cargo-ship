# cargo-ship
[![Build Status](https://travis-ci.org/chills42/cargo-ship.svg)](https://travis-ci.org/chills42/cargo-ship)

This cargo plugin provides the `ship` command, which at this point simply
runs `cargo test`, `cargo build --release`, and then copies the target
executable to the `~/.bin/` folder.

The cargo-ship tool is a cargo plugin that provides a simple way to ship
a tested release build of a project.

## Defaults

Running this as a plugin using `cargo ship` is basically the same as doing the following:

```text
cargo test
cargo build --release
cp ./target/release/<target_name> ~/.bin
```

with <target_name> being the name specified in the project's Cargo.toml file as the release name

## Assumptions

The user is expected to have a recognizable home directory.
The user should have a `~/.bin` folder that has been added to their PATH environment variable.

## Future plans

Although this is a very simplistic tool, and is really only useful for deploying built executables into a local folder
the goal is to provide a more complete deployment tool with configurable targeting.
Ideally, I'd like to see:

- configurable local deployment (use .bin or /usr/local/bin or /my_crazy_directory_i_added_to_my_path)
- s3 deployment (maybe using the awscli?)
- ssh/scp deploys
- allow other profiles besides release

