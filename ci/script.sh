set -euxo pipefail

main() {
    cargo check --target $TARGET

    if [ "$TARGET" = "thumbv7m-none-eabi" ]; then
        cargo check --example stm32 --target $TARGET
    fi

    if [ "$TARGET" = "x86_64-unknown-linux-gnu" ]; then
        # the --tests is required to ignore the examples
        # which will not compile under x86
        cargo test --target $TARGET --tests
    fi
}

main
