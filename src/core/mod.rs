//! This is the `core` default module which contains the basic functions/features all other
//! spspsf creates depends on.

/// The `input` module is a wrapper for the PSP's input functions to work in a simple and cohesive manner.
pub mod input;
pub use input::{Buttons, InputManager};

// The `io` module is a wrapper for the PSP's File Input and Output
//pub mod io;
//pub mod threads;
/// The `utils` module is a set of different functions that serve multiple purposes in the SPSPF project.
pub mod utils;

/// Vector 3 (x, y and z coordinates)
#[derive(Clone, Default, Copy, Debug)]
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
#[derive(Clone, Default, Copy, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}