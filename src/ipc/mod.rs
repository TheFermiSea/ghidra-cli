//! IPC module for CLI-to-daemon communication.
//!
//! This module provides cross-platform IPC using local sockets:
//! - Unix domain sockets on Linux/macOS
//! - Named pipes on Windows
//!
//! Follows the pattern from debugger-cli for reliable length-prefixed
//! JSON message framing.

pub mod protocol;
pub mod transport;
