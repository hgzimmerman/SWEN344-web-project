#![feature(custom_attribute)] // Are we stuck on nightly with this???

#[macro_use]
extern crate diesel;

pub mod event;
pub mod funds;
pub mod pool;
mod schema;
pub mod stock;
pub mod user;
mod util;
