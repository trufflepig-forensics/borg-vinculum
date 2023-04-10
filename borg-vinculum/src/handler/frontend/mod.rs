//! All handler for the frontend are defined in this module

pub use crate::handler::frontend::auth::*;
pub use crate::handler::frontend::drones::*;
pub use crate::handler::frontend::key::*;

mod auth;
mod drones;
mod key;
