# VectRust

VectRust is a Rust library for [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html). It's a simple library built to learn Rust and to understand how allocators work in Rust.

## Development

To play with the library, you should check the `Makefile` to see all available commands.

```bash
make help
```

Also, to check any memory-related issues, you can use [`valgrind`](https://valgrind.org/).

```bash
valgrind --leak-check=full ./target/release/vectrust
```
