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
//! 
//! This is the `core` crate which contains the basic functions/features all other spspsf creates depends on.

#![no_std]
#![no_main]
#![feature(bigint_helper_methods)]

/// The `input` module is a wrapper for the PSP's input functions to work in a simple and cohesive manner.
pub mod input;
pub use crate::input::Buttons;
pub use crate::input::InputManager;

// The `io` module is a wrapper for the PSP's File Input and Output 
//pub mod io;
//pub mod threads;
/// The `utils` module is a set of different functions that serve multiple purposes in the SPSPF project.
pub mod utils;

/// Vector 3 (x, y and z coordinates)
#[derive(Clone, Default, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

/// Vector 2 (x and y coordinates)
#[derive(Clone, Default, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}
