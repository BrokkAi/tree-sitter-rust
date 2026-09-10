# Brokk's Rust Grammar for Tree-sitter

[![CI][ci]](https://github.com/BrokkAi/tree-sitter-rust/actions/workflows/ci.yml)
[![crates][crates]](https://crates.io/crates/brokk-tree-sitter-rust)
[![docs.rs][docs]](https://docs.rs/brokk-tree-sitter-rust)

This is the **Brokk-owned and independently maintained fork** of
[`tree-sitter/tree-sitter-rust`](https://github.com/tree-sitter/tree-sitter-rust),
a Rust grammar for [Tree-sitter](https://tree-sitter.github.io/tree-sitter/).
Brokk maintains this fork for its code-intelligence tooling and publishes the
Rust package as
[`brokk-tree-sitter-rust`](https://crates.io/crates/brokk-tree-sitter-rust).
It intentionally diverges where Brokk needs language support that is not yet
available upstream. Unless you specifically need Brokk's changes, you may
prefer the upstream project.

## Installation

Add the Brokk-maintained Rust crate to your project:

```sh
cargo add brokk-tree-sitter-rust@=0.24.3
```

Or add it directly to `Cargo.toml`:

```toml
[dependencies]
brokk-tree-sitter-rust = "=0.24.3"
```

The npm and Python bindings retain their upstream-compatible package names but
are not published by this fork. The Brokk-namespaced Rust crate prefixes its
native symbols, so it can coexist with the upstream `tree-sitter-rust` crate in
one executable.

## Features

- **Speed** — When initially parsing a file, `tree-sitter-rust` takes around two to three times
  as long as rustc's hand-written parser.

  ```sh
  $ wc -l examples/ast.rs
    2157 examples/ast.rs

  $ rustc -Z unpretty=ast-tree -Z time-passes examples/ast.rs | head -n0
    time:   0.002; rss:   55MB ->   60MB (   +5MB)  parse_crate

  $ tree-sitter parse examples/ast.rs --quiet --time
    examples/ast.rs    6.48 ms        9908 bytes/ms
  ```

  But if you _edit_ the file after parsing it, tree-sitter can generally _update_
  the previous existing syntax tree to reflect your edit in less than a millisecond,
  thanks to its incremental parsing system.

## References

- [The Rust Reference](https://doc.rust-lang.org/reference/) — While Rust does
  not have a specification, the reference tries to describe its working in detail.
  It tends to be out of date.
- [Keywords](https://doc.rust-lang.org/stable/book/appendix-01-keywords.html) and
  [Operators and Symbols](https://doc.rust-lang.org/stable/book/appendix-02-operators.html).

[ci]: https://img.shields.io/github/actions/workflow/status/BrokkAi/tree-sitter-rust/ci.yml?logo=github&label=CI
[crates]: https://img.shields.io/crates/v/brokk-tree-sitter-rust?logo=rust
[docs]: https://img.shields.io/docsrs/brokk-tree-sitter-rust?logo=docs.rs
