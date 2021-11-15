alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'

cd functions/get_key
rust-musl-builder cargo build --release
cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./bootstrap && zip lambda.zip bootstrap && rm bootstrap

################################################################################

cd ../set_key
rust-musl-builder cargo build --release
cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./bootstrap && zip lambda.zip bootstrap && rm bootstrap