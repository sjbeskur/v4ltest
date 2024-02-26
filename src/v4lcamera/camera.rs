use super::converters::*;
use crate::traits::ImageSensor;

use image::{ImageBuffer, RgbImage};
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::{context, prelude::*};

pub struct V4l2Camera {
	//dev_id: usize,
	device: v4l::Device,
}

impl V4l2Camera {
	pub fn new(id: usize) -> Self {
		Self {
			device: Device::new(id).expect("Failed to open device"),
		}
	}
}

impl Drop for V4l2Camera {
	fn drop(&mut self) {
		// self.device.release();
	}
}

impl ImageSensor for V4l2Camera {
	/// There is a lot of cruff in this implementation but I'm     
	/// just trying to understand how to use the v4l2 rust lib.  Mostly
	/// not comfortable with by how to decompress the frames etc
	fn capture(&mut self) -> crate::AppResult<Vec<u8>> {
		//let mut dev = Device::new(self.dev_id).expect("Failed to open device");
		let mut stream = MmapStream::with_buffers(&mut self.device, Type::VideoCapture, 4)
			.expect("Failed to create buffer stream");
		let (buf, _meta) = stream.next()?;
		let rgb = convert_ir_buffer(&buf);
		dbg!("len of converted: {}", rgb.len());
		Ok(rgb)
	}
}

#[test]
fn test_v4lcapture() {
	let mut sensor = V4l2Camera::new(2);
	let img_buffer = sensor.capture().unwrap();

	let format = sensor.device.format().unwrap();

	// https://github.com/raymanfx/libv4l-rs/issues/74
	// ImageBuffer<Rgb<u8>, &[u8]>
	let img: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
		ImageBuffer::from_raw(format.width, format.height, img_buffer)
			.expect("Failed to create image buffer from raw buf");


	// let img = image::load_from_memory_with_format(&out, image::ImageFormat::Jpeg)?.to_rgb8();
	img.save(format!("test_img.png"));

    println!("Buffer size: {}",
		//, timestamp: {}",
		img.len(),
		//meta.sequence,
		//meta.timestamp
	);
}

#[test]
fn test_list_devices() {
	let devices = context::enum_devices();
	for dev in devices {
		println!("{}: {}", dev.index(), dev.name().unwrap());
	}
}

#[test]
fn test_dump_formats() {
	let sensor = V4l2Camera::new(0);

	let format = sensor.device.format().unwrap();
	println!("Active format: {}", format);

	let params = sensor.device.params().unwrap();
	println!("Active parameters:\n{}", params);
}

#[test]
fn test_query_controlls() {
	let sensor = V4l2Camera::new(0);
	let controls = sensor.device.query_controls().unwrap();
	for control in controls {
		println!("\t{}", control);
	}
}

/*

	let len = buf.len();
	let mut out = [0; 640 * 480 * 2 ];

*/
