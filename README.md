# `light-cli`

A lightweight and heapless command line interface / command passing tool. Probably more useful for machine to machine communication.

## [Documentation](https://rudihorn.github.io/light-cli/light_cli/)

## [Examples](https://github.com/rudihorn/light-cli/tree/master/examples/)

There is an example for the STM32F103 microcontroller in stm32.rs.

## What works

- Read key value style commands in the form:
  COMMAND KEY=VALUE
- UTF-8 encoding.
- Specify the heapless string length.
- Partial command evaluation as data is received through the serial connection.

## TODO

- [ ] Improve UTF-8 error detection / code.
- [ ] Any form of autocompletion / backspaces etc.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

