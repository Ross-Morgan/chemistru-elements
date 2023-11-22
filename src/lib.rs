//! # chemistru-elements
//!
//! This library is for parsing and representing elements from JSON data in a
//! memory efficient format, allowing construction of complex molecules that do
//! not destroy the performance of the program.

pub mod data;
pub mod element;
pub mod inner;
pub mod ion;
pub mod raw;
pub mod reaction;
