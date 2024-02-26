#![allow(dead_code)]

mod ocvcamera;
mod traits;
mod v4lcamera;

pub use traits::*;

pub use ocvcamera::*;

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;
