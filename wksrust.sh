# install Rust into github woorkspace
#
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
source "$HOME/.cargo/.bashrc"
source "$HOME/.cargo/.zshrc"
cargo install just
rustup component add clippy