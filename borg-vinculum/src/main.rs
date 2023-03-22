//! # borg-vinculum
//!
//! The control unit of all borg-drones.
#![warn(missing_docs)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]

pub mod config;
pub mod models;

#[rorm::rorm_main]
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
