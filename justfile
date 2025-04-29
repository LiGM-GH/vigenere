set unstable

default:
    just --list

run:
    cargo build
    spd-say "Compilation ended"
    sleep 5
    cargo run

test:
    cargo test --workspace
