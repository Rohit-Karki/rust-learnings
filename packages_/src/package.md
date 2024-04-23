A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates

A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
