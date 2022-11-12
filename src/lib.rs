//! A modern ergonomic GitHub REST API Rust binding.

#![deny(missing_docs)]

#[cfg(test)] mod test_util;

pub mod prelude {
	//! GitHuber prelude.

	pub use crate::api::{ApiGet, ApiPost};
}

pub mod api;
