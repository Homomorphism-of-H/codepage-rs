assure: check build-all
    cargo test --no-default-features
    cargo test --no-default-features --features std
    cargo test --no-default-features --features bytemuck
    cargo test --all-features
    cargo doc --all-features

check:
    cargo clippy

build-all:
    cargo build --no-default-features
    cargo build --no-default-features --features std
    cargo build --no-default-features --features bytemuck
    cargo build --all-features

build-all-release:
    cargo build --no-default-features --release
    cargo build --no-default-features --features std --release
    cargo build --no-default-features --features bytemuck --release
    cargo build --all-features --release