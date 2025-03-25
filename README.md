# Xenon CodeGen

## The Xenon Code Generator & AST Representation

This package functions similar to Rust's codegen crate, allowing an in-code representation of Xenon code in the form of Nodes. Every node impliments fmt::Display, and the to_string() function generates correct code that can be passed back into the parser.

### TODOs

- Into\<T\> for variants of enum types
- Documentation
- fmt::Display for All structs and enums
