default:
    just --list

build:
    cargo build

build-release:
    cargo build --release    

fmt:
    cargo fmt -- --check

clippy: 
    cargo clippy --all-targets --all-features -- -D warnings

