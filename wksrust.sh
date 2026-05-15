# Install Rust & Tools into github workspace, just for development & testing, not for production use
# 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
source "$HOME/.cargo/.bashrc"
source "$HOME/.cargo/.zshrc"
cargo install just
rustup component add clippy