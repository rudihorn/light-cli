set -euxo pipefail

main() {
    if [ "$TARGET" = "thumbv7m-none-eabi" ]; then
	    rustup install target thumbv7m-none-eabi
    fi

    return
}

main
