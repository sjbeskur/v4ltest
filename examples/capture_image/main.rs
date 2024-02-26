use cv::prelude::*;
use image::ImageBuffer;
use opencv as cv;
use v4ltest::*;

fn main() -> v4ltest::AppResult<()> {
	println!("Capture single image example");
	let mut camera = v4ltest::camera::OCVCamera::new(2);
	let img_buffer = camera.capture()?;

	// let img =
	// 	Mat::new_rows_cols_with_default(512, 640, cv::core::CV_8UC1, cv::core::Scalar::all(0.0))
	// 		.unwrap();

	let img: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(640, 512, img_buffer)
		.expect("Failed to create image buffer from raw buf");

	img.save("sample_image.png");
	//println!("{:?}",img.size());
	//cv::highgui::imshow("Display Window", &img).unwrap();
	//let k = opencv::highgui::wait_key(0).unwrap();

	Ok(())
}
