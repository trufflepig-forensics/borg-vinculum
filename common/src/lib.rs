//! # common
//!
//! The common module for borg-drone and borg-vinculum.
//!
//! This library holds all types that are used throughout both binaries.
#![warn(missing_docs)]

use std::fmt::{Display, Formatter};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// The type of the hook
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum HookType {
    /// a pre hook is executed before the archive is created
    Pre,
    /// a post hook is executed after the archive is created
    Post,
}

impl Display for HookType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HookType::Pre => write!(f, "pre"),
            HookType::Post => write!(f, "post"),
        }
    }
}

/// The statistics from each operation of borg drone.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Stats {
    /// The stats of a hook operation
    Hook {
        /// The duration of the pre hook operation
        duration: Duration,
        /// The type of the hook this stats are for
        hook_type: HookType,
    },
    /// The stats of a create archive operation
    Create {
        /// Original file size in bytes
        original_size: u64,
        /// Compressed file size in bytes
        compressed_size: u64,
        /// Deduplicated file size in bytes
        deduplicated_size: u64,
        /// Number of archived files
        nfiles: u64,
        /// The duration of the create archive operation
        duration: Duration,
    },
}
