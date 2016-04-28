# cargo-ship
[![Build Status](https://travis-ci.org/chills42/cargo-ship.svg)](https://travis-ci.org/chills42/cargo-ship)
[![](http://meritbadge.herokuapp.com/cargo-ship)](https://crates.io/crates/cargo-ship)

This cargo plugin provides the `ship` command, which at this point simply
runs `cargo test`, `cargo build --release`, and then copies the target
executable to the `~/.bin/` folder.

The cargo-ship tool is a cargo plugin that provides a simple way to ship
a tested release build of a project.

For example, when I want to get the latest copy of the [xsv](https://github.com/BurntSushi/xsv) csv toolkit I can do the following:

```bash
git clone https://github.com/BurntSushi/xsv.git
cd xsv
cargo ship
```

After the tool passes tests and compiles in release mode, it is ready to use!

```bash
echo -e 'these,are,my,headers\n1,2,3,4\n5,6,7,8' | xsv table
```

## Defaults

Running this as a plugin using `cargo ship` is basically the same as doing the following:

```bash
cargo test
cargo build --release
# <target_name> to be replaced by name of project in Cargo.toml
cp ./target/release/<target_name> ~/.bin
```

## Assumptions

- The user is expected to have a recognizable home directory.
- The user should have a `~/.bin` folder that has been added to their PATH environment variable.

## Future plans

Although this is a very simplistic tool, and is really only useful for deploying built executables into a local folder
the goal is to provide a more complete deployment tool with configurable targeting.
Ideally, I'd like to see:

- configurable local deployment (use .bin or /usr/local/bin or /my_crazy_directory_i_added_to_my_path)
- s3 deployment (maybe using the awscli?)
- ssh/scp deploys
- allow other profiles besides release

