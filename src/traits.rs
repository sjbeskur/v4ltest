use crate::AppResult;

pub trait ImageSensor {
	fn capture(&mut self) -> AppResult<crate::CameraImage>;
}
