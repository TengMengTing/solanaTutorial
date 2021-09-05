pub mod entrypoint;
pub mod instruction;
pub mod error;
pub mod state;

#[cfg(not(feature = "no-entrypoing"))]
pub mod entrypoint;