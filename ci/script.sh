set -euxo pipefail

main() {
    cargo check --target $TARGET

    if [ "$TARGET" = "thumbv7m-none-eabi" ]; then
        cargo check --example stm32 --target $TARGET
    fi

    cargo test --target $TARGET
}

main
