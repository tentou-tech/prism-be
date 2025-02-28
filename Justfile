check:
  @echo "Running cargo udeps..."
  cargo +nightly udeps --all-features --all-targets
  @echo "Running clippy..."
  cargo clippy --all --all-targets -- -D warnings

build:
  @echo "Building the project..."
  cargo build --release

try:
  @echo "Running the project..."
  RUST_BACKTRACE=full RUST_LOG="debug" cargo run

unit-test:
  @echo "Running unit tests..."
  RUST_BACKTRACE=full cargo test --release -- --nocapture
