use crate::traits::ImageSensor;

use cv::core::MatTraitConstManual;
use cv::types::VectorOfKeyPoint;
use cv::videoio::{VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, VideoWriterTrait};
use cv::{
	core::{KeyPoint, Mat, Scalar, Vector},
	features2d::*,
	highgui::{imshow, named_window, WindowFlags},
};
use opencv as cv;

pub struct OCVCamera {
	//device: VideoCapture,
	index: i32,
}

impl OCVCamera {
	pub fn new(index: i32) -> Self {
		Self { index }
	}
	pub fn capture_video(&self) {
		//-> Result<Vec<u8>, Box<dyn std::error::Error>> {
		let mut device = VideoCapture::new(self.index, cv::videoio::CAP_V4L2).unwrap();
		//let mut device  = VideoCapture::new_def(index).unwrap();

		let mut akaze = AKAZE::create_def().unwrap();

		//let img1 = imread(file1, IMREAD_GRAYSCALE)?;
		let mut kp = VectorOfKeyPoint::new();
		let mut desc = Mat::default();

		let mask = Mat::default();
		let red_color = Scalar::new(0.0, 0.0, 255.0, 0.0);

		let mut img = cv::prelude::Mat::default();

		//let fourcc = cv::videoio::VideoWriter::fourcc('X', 'V', 'I', 'D').unwrap();
		// let fourcc = cv::videoio::VideoWriter::fourcc('m', 'p' , '4' , 'v').unwrap();
		// let mut writer = cv::videoio::VideoWriter::new("detects.mp4", fourcc, 20.0, cv::core::Size2i::new(640,512),false).unwrap();
		// let frame_width =  device.get(3).unwrap();
		// let frame_height = device.get(4).unwrap();
		// println!("width {}, height {}", frame_width, frame_height);

		loop {
			let _ = device.read(&mut img).unwrap();
			//writer.write(&img).unwrap();

			let _ = akaze.detect_and_compute(&img, &mask, &mut kp, &mut desc, false);

			let mut out_img = cv::prelude::Mat::default();

			draw_keypoints(
				&img,
				&kp,
				&mut out_img,
				red_color,
				DrawMatchesFlags::DEFAULT,
			)
			.unwrap();
			//println!("{:?}",img.size());

			cv::highgui::imshow("Display Window", &out_img).unwrap();

			let k = opencv::highgui::wait_key(1).unwrap();
			if k == 32 {
				break;
			}
		}
		// writer.release().unwrap();
		device.release().unwrap();

		// cv::highgui::named_window("Display Window", WindowFlags::WINDOW_NORMAL as i32).unwrap();
		// opencv::highgui::wait_key(0).unwrap();        //Ok(image);
	}
}

impl ImageSensor for OCVCamera {
	fn capture(&mut self) -> crate::AppResult<Vec<u8>> {
		let mut device = VideoCapture::new(self.index, cv::videoio::CAP_V4L2).unwrap();
		let mut img = cv::prelude::Mat::default();
		let _ = device.read(&mut img).unwrap();

		//let vec2d: Vec<Vec<u8>> = img.to_vec_2d()?;
		Ok(img.data_bytes()?.to_vec())
		//Ok(vec2d)
	}
}
