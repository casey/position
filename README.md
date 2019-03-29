# position

[![crates.io](https://img.shields.io/crates/v/position.svg)](https://crates.io/crates/position) [![docs](https://docs.rs/position/badge.svg)](http://docs.rs/position)

`position` provides a `Position` struct, representing a source code position,
as well as a convenient `here!()` macro for creating a position corresponding
to the location where `here!()` was invoked:

```rust
use position::{here, Position};
let p: Position = here!();
assert_eq!(p.file(), "src/lib.rs");
assert_eq!(p.line(), 5);
assert_eq!(p.column(), 19);
assert_eq!(p.module_path(), "rust_out");
assert_eq!(p.to_string(), "src/lib.rs:5:19");
```

If `position` is compiled with the `location` feature, `Position` implements
`oi::Location`, so it can be used with `oi::ErrAt::err_at`:

```rust
# #[cfg(feature = "location")]
# {
use std::{io, fs::File};
use oi::ErrAt;
use position::{here, Position};

let result: oi::Result<File, io::Error, Position> =
  File::open("foo.txt").err_at(here!());

assert_eq!(
  result.unwrap_err().to_string(),
  "src/lib.rs:11:32: No such file or directory (os error 2)",
);
# }
```
