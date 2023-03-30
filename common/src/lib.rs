//! # common
//!
//! The common module for borg-drone and borg-vinculum.
//!
//! This library holds all types that are used throughout both binaries.
#![warn(missing_docs)]

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The state of the operation
#[derive(Deserialize, Serialize, Copy, Clone, Debug, ToSchema)]
pub enum State {
    /// Pre hook
    PreHook,
    /// Archive creation
    Create,
    /// Post hook
    PostHook,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::PreHook => write!(f, "pre hook"),
            State::PostHook => write!(f, "post hook"),
            State::Create => write!(f, "archive creation"),
        }
    }
}

/// The statistics from each operation of borg drone.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, ToSchema)]
pub struct HookStats {
    /// The duration of the pre hook operation in seconds
    pub duration: u64,
}

/// The stats of the creation of an archive
#[derive(Serialize, Deserialize, Clone, Copy, Debug, ToSchema)]
pub struct CreateStats {
    /// Original file size in bytes
    pub original_size: u64,
    /// Compressed file size in bytes
    pub compressed_size: u64,
    /// Deduplicated file size in bytes
    pub deduplicated_size: u64,
    /// Number of archived files
    pub nfiles: u64,
    /// The duration of the create archive operation in seconds
    pub duration: u64,
}

/// The report of the collected stats that sent from a drone to the vinculum
#[derive(Serialize, Deserialize, Clone, Copy, Debug, ToSchema)]
pub struct StatReport {
    /// The stats of the pre hook
    pub pre_hook_stats: Option<HookStats>,
    /// The stats of the archive creation
    pub create_stats: CreateStats,
    /// The stats of the post hook
    pub post_hook_stats: Option<HookStats>,
}
