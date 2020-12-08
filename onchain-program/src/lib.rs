#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! a lottery game for solong

use solana_program::{
    msg,
};

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;



#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

// logger
pub(crate) fn log_info(message: &str) {
    msg!(format!("[{}]:{}", "solong-lottery", message).as_str());
}
