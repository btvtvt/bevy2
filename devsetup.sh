# x11
# sudo apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0

# Wayland
# sudo apt-get install -y libwayland-dev libxkbcommon-dev

sudo apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev \
    libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev libssl-dev

# https://bevy.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional
sudo apt-get -y install lld clang

rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo install wasm-bindgen-cli
