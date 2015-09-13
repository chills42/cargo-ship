# cargo-ship

This cargo plugin provides the `ship` command, which at this point simply
runs `cargo test`, `cargo build --release`, and then copies the target
executable to the `~/.bin/` folder.
