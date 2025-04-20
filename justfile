set unstable

default:
    just --list

run:
    cargo run

test:
    cargo test --workspace
