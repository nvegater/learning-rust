// lib.rs exposes modules as a library crate.
// Integration tests in `tests/` can only import from the library, not from main.rs.
// This is like the difference between `export` (lib.rs) and your server entry point (main.rs).
pub mod posts_client;
