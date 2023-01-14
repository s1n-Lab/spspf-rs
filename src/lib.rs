//! # SPSPF
//!
//! `SPSPF` or Simple PSP Framework is a simple, Rust-based, modular framework and high-level abstraction
//! layer for Sony's Playstation Portable. Although its main use is for game development, SPSPF aims
//! to allow users to develop all sorts of homebrew to their PSP.
//!
//! Due to its modularity you can choose to only use some portions of the framework and just disregard
//! the ones that fit little to no use in your project. The source code is also publicly available so if
//! you need to extract a function and pop it into your application you can do so with no problems, just
//! making sure to comply with the License of this project and its references.

#![no_std]
#![no_main]
#![feature(bigint_helper_methods)]

pub mod core;
#[cfg(feature = "graphics")]
pub mod graphics;
