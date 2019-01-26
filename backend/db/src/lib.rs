#![feature(custom_attribute)] // Are we stuck on nightly with this???

#[macro_use]
extern crate diesel;


pub mod event;
pub mod user;
pub mod stock;
pub mod funds;
mod util;
mod schema;




