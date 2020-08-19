cargo install node-cli --git https://github.com/paritytech/substrate.git --tag v2.0.0-rc4 --force
cargo install cargo-contract --vers 0.6.1 --force
cargo contract new erc20

cargo +nightly contract build
cargo +nightly contract generate-metadata
 cargo +nightly test
