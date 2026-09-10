# Rust Grammar for Tree-sitter

This crate provides Brokk's independently maintained package of the
[`tree-sitter/tree-sitter-rust`](https://github.com/tree-sitter/tree-sitter-rust)
grammar. It prefixes all native symbols so it can coexist with the upstream
Rust grammar crate in one executable.

To use this crate, add it to the `[dependencies]` section of your `Cargo.toml`:

```toml
tree-sitter = "0.25"
brokk-tree-sitter-rust = "=0.24.2"
```

Typically, use the `LANGUAGE` constant with a tree-sitter `Parser`:

```rust
let code = r#"
pub trait LendingIterator<'a> = Iterator<Item = &'a str>;
"#;
let mut parser = tree_sitter::Parser::new();
parser
    .set_language(&brokk_tree_sitter_rust::LANGUAGE.into())
    .expect("Error loading Rust grammar");
let parsed = parser.parse(code, None).unwrap();
assert!(!parsed.root_node().has_error());
```
