#![allow(dead_code)]

mod ocvcamera;
mod traits;
mod v4lcamera;

pub use traits::*;

pub use ocvcamera::*;

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct CameraImage {
	pub width: usize,
	pub height: usize,
	pub data: Vec<u8>,
}

impl CameraImage {
	pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
		Self {
			width,
			height,
			data,
		}
	}
}
