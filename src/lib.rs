#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
// remove before flight
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused)]
pub mod data;
pub mod geometry;
pub mod io;
