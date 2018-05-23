# `light-cli`

A lightweight and heapless command line interface / command passing tool. Probably more useful for machine to machine communication.

## [Documentation](https://rudihorn.github.io/light-cli/light_cli/)

## [Example]

The following definition allows for the commands:
* `HELLO Name=<Name>`: Set the name to `<Name>`
* `EHLO`: Return the name

```
lightcli!(cl_in, cl_out, cmd, key, val, [
  "HELLO" => [
    "Name" => name = String::from(val)
  ] => { writeln!(cl_out, "Name set").unwrap(); };
  "EHLO" => [
  ] => { writeln!(cl_out, "EHLO Name={}", name.as_str()).unwrap(); }
]);
```

A typical serial communication could look like:

```
>> EHLO
<< EHLO Name=
>> HELLO Name=Johnson
<< Name set
>> EHLO
<< EHLO Name=Johnson
```

It is recommended to use this in conjunction with the program [`rlwrap`](https://linux.die.net/man/1/rlwrap).

[Complete Example](https://github.com/rudihorn/light-cli/tree/master/examples/)

## What works

- Read key value style commands in the form:
  `COMMAND KEY=VALUE`
- UTF-8 encoding.
- Specify the heapless string length.
- Partial command evaluation as data is received through the serial connection.

## TODO

- [X] Writing to output
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

