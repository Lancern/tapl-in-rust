# tapl-in-rust

Implementation of the topics discussed in the book _Types and Programming
Languages_ (TaPL) in the Rust programming language.

## Topics

- [ ] Part I: Untyped systems
  - [x] Chapter 3: Untyped arithmetic expressions
  - [ ] Chapter 5 & 6: Untyped lambda-calculus, nameless representation of terms

> More topics are coming.

## Build and Run

You need a valid Rust toolchain to build this project. You can install a Rust
toolchain with [rustup](https://rustup.rs/).

### Chapter 3 - untyped arithmetic expressions

To run the implementation of untyped arithmetic expressions (TaPL chapter 3),
execute the following command:

```bash
cargo run --bin tapl-cli --features cli --features unty_arith -- \
    --topic unty-arith <input>
```

`<input>` should be the path to the input file containing source code of untyped
arithmetic expressions.

## License

This project is open sourced under [MIT License](./LICENSE).
