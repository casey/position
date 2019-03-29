//! `position` provides a `Position` struct, representing a source code position,
//! as well as a convenient `here!()` macro for creating a position corresponding
//! to the location where `here!()` was invoked:
//!
//! ```rust
//! use position::{here, Position};
//! let p: Position = here!();
//! assert_eq!(p.file(), "src/lib.rs");
//! assert_eq!(p.line(), 5);
//! assert_eq!(p.column(), 19);
//! assert_eq!(p.module_path(), "rust_out");
//! assert_eq!(p.to_string(), "src/lib.rs:5:19");
//! ```
//!
//! If `position` is compiled with the `location` feature, on by default, `Position`
//! implements `oi::Location`, so it can be used with `oi::ErrAt::err_at`:
//!
//! ```rust
//! # #[cfg(feature = "location")]
//! # {
//! use std::{io, fs::File};
//! use oi::ErrAt;
//! use position::{here, Position};
//!
//! let result: oi::Result<File, io::Error, Position> =
//!   File::open("foo.txt").err_at(here!());
//!
//! assert_eq!(
//!   result.unwrap_err().to_string(),
//!   "src/lib.rs:11:32: No such file or directory (os error 2)",
//! );
//! # }
//! ```

use std::fmt::{self, Display, Formatter};

#[cfg(feature = "location")]
use oi::Location;

#[cfg(feature = "location")]
use failure::Fail;

/// Macro returning `Position` it was invoked
#[macro_export]
macro_rules! here {
  () => {
    $crate::Position::new(module_path!(), file!(), line!(), column!())
  };
}

/// Source code position
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
  /// Module path as returned by module_path!()
  module_path: &'static str,
  /// File as returned by file!()
  file: &'static str,
  /// Line as returned by line!()
  line: u32,
  /// Column as returned by column!()
  column: u32,
}

impl Position {
  /// Construct a new `Position`
  pub fn new(module_path: &'static str, file: &'static str, line: u32, column: u32) -> Position {
    Position {
      module_path,
      file,
      line,
      column,
    }
  }

  /// Get file as returned by file!()
  pub fn file(self) -> &'static str {
    self.file
  }

  /// Get module path as returned by module_path!()
  pub fn module_path(self) -> &'static str {
    self.module_path
  }

  /// Get line as returned by line!()
  pub fn line(self) -> u32 {
    self.line
  }

  /// Get column as returned by column!()
  pub fn column(self) -> u32 {
    self.column
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line, self.column)
  }
}

#[cfg(feature = "location")]
impl Location for Position {
  fn fmt_error(&self, f: &mut Formatter, error: &dyn Fail) -> fmt::Result {
    write!(f, "{}: {}", self, error)
  }
}
