default:
    @just --list

# Match the native checks in CI.
lint:
    cargo fmt --all --check
    RUSTFLAGS="-Dwarnings" cargo clippy --locked --all-targets --no-default-features
    RUSTFLAGS="-Dwarnings" cargo clippy --locked --all-targets --all-features
    RUSTDOCFLAGS="-Dwarnings" cargo doc --locked --no-deps --no-default-features
    RUSTDOCFLAGS="-Dwarnings" cargo doc --locked --no-deps --all-features

test:
    cargo test --locked --no-fail-fast
    cargo test --locked --no-default-features --no-fail-fast
    cargo test --locked --all-features --no-fail-fast

msrv:
    cargo hack check --locked --rust-version --each-feature --all-targets

wasm:
    rustup target add wasm32-unknown-unknown wasm32-wasip1
    cargo check --locked --lib --target wasm32-unknown-unknown --no-default-features --features wasm_js
    cargo check --locked --lib --target wasm32-wasip1 --no-default-features

audit:
    cargo audit --deny warnings

workflows:
    actionlint
    zizmor --offline .github/workflows

check: lint test msrv wasm audit workflows

coverage:
    cargo llvm-cov --locked --all-features --html

coverage-open:
    cargo llvm-cov --locked --all-features --html --open

coverage-lcov:
    cargo llvm-cov --locked --all-features --lcov --output-path lcov.info

coverage-clean:
    cargo llvm-cov clean
