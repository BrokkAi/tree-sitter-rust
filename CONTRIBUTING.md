# Contributing to tree-sitter-rust

This repository is Brokk's independently maintained fork of
[`tree-sitter/tree-sitter-rust`](https://github.com/tree-sitter/tree-sitter-rust).
Contributions here should target Brokk's fork and its code-intelligence use
cases. Changes intended for the general upstream grammar may be better proposed
upstream first.

## Development

Install the JavaScript dependencies, regenerate the parser after grammar
changes, and run the corpus and binding tests:

```bash
npm install
npx tree-sitter generate
npx tree-sitter test
cargo test --locked
```

Commit generated parser artifacts in `src/` alongside changes to `grammar.js`.
