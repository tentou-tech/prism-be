check:
  @echo "Running cargo fmt..."
  cargo fmt --all -- --check
  @echo "Running cargo clippy..."
  cargo clippy --all --all-targets -- -D warnings

build:
  @echo "Building the project..."
  cargo build --release

run:
  @echo "Running the project..."
  RUST_BACKTRACE=full RUST_LOG="debug" cargo run

unit-test:
  @echo "Running unit tests..."
  RUST_BACKTRACE=full cargo test --release -- --nocapture
